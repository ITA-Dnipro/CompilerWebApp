mod http_handlers;
mod filework;
mod config_struct;

#[macro_use] extern crate rocket;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
use figment::providers::Format;
use figment::{Figment, providers::Yaml};
use rocket::fairing::AdHoc;
use rocket::fs::relative;
use slog::Drain;

use std::env::current_dir;
use std::sync::{Arc, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;
use rocket::fs::FileServer;
use config_struct::BackendConfig;

use http_handlers::{submit, sessions::SessionsTracker, index};

#[launch]
fn rocket() -> _ 
{
    // Backend config loading
    let mut backend_config: BackendConfig = Figment::new()
        .merge(Yaml::file(relative!("BackendConfig.yaml")))
        .extract().unwrap();

    // Logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    // Logger uses an async drain, so it doesn't need to be manually managed with a mutex
    // Or at least I believe so, correct me if I'm wrong
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Arc::new(slog::Logger::root(drain, o!()));

    // Paths validation
    if !backend_config.sessions_data_dir.is_absolute()
    {
        backend_config.sessions_data_dir = current_dir().unwrap()
            .join(&backend_config.sessions_data_dir);
    }

    if !backend_config.sessions_data_file.is_absolute()
    {
        backend_config.sessions_data_file = current_dir().unwrap()
            .join(&backend_config.sessions_data_file);
    }

    if !backend_config.sessions_data_dir.is_dir()
    {
        // Sessions directory doesn't exist yet
        if let Err(_) = std::fs::create_dir(&backend_config.sessions_data_dir)
        {
            crit!(logger, "Couldn't create sessions files storage dir at: {:?}", 
                backend_config.sessions_data_dir);
        }
    }

    // Sessions tracker
    let sessions_tracker;
    match SessionsTracker::from_file( &backend_config.sessions_data_file)
    {
        Some(tr) => 
        {
            info!(logger, "Read sessions data from: {:?}", 
                backend_config.sessions_data_file);
            sessions_tracker = Arc::new(tr.life_duration(
                &Duration::from_millis(backend_config.session_life_duration)));
        }
        None => 
        {
            info!(logger, "Couldn't read sessions data from: {:?}", 
                backend_config.sessions_data_file);
            sessions_tracker = Arc::new(SessionsTracker::new().life_duration(
                &Duration::from_millis(backend_config.session_life_duration)))
        }
    }

    info!(logger, "Backend config:\n {:?}", backend_config);

    rocket::build()
        // index.html getters
        .mount("/", routes![index::get_index])
        .mount("/", FileServer::from(relative!("static")))
        // Submission endpoint
        .mount("/", routes![submit::post_submit])
        // Server states
        .manage(RwLock::new(backend_config))
        .manage(sessions_tracker)
        .manage(logger)
        // Templaiting fairing
        .attach(rocket_dyn_templates::Template::fairing())
        // Sessions cleaner thread startup
        .attach(AdHoc::on_liftoff("Sessions cleaner", |rocket| Box::pin(async move 
            {
                let tracker = rocket.state::<Arc<SessionsTracker>>()
                    .unwrap().to_owned();
                let logger = rocket.state::<Arc<slog::Logger>>().unwrap().to_owned();
                let interval = rocket.state::<RwLock<BackendConfig>>()
                    .unwrap().read().unwrap().sessions_cleanup_interval;
                info!(logger, "Sessions cleaner started");
                
                thread::spawn(move ||
                {
                    loop
                    {
                        sleep(std::time::Duration::from_millis(interval));                           
                        let deleted = tracker.delete_old();
                        info!(logger, "Deleted {} old sessions", deleted);
                    }
                });
            })))
        // Sessions saver thread startup
        .attach(AdHoc::on_liftoff("Sessions saver", |rocket| Box::pin(async move 
            {
                let tracker = rocket.state::<Arc<SessionsTracker>>()
                    .unwrap().to_owned();
                let logger = rocket.state::<Arc<slog::Logger>>().unwrap().to_owned();
                let config = rocket.state::<RwLock<BackendConfig>>().unwrap()
                    .read().unwrap();
                let interval = config.sessions_save_interval;
                let save_path = config.sessions_data_file.clone();
                
                drop(config);
                info!(logger, "Sessions saver started");
                
                thread::spawn(move ||
                {
                    loop
                    {
                        sleep(std::time::Duration::from_millis(interval));
                        tracker.save(&save_path);
                        info!(logger, "Saved sessions data to a file");
                    }
                });
            })))
}
