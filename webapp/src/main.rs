#[macro_use] extern crate rocket;

mod http_handlers;
mod filework;
mod config_struct;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
use figment::providers::Format;
use figment::{Figment, providers::Yaml};
use rocket::fairing::AdHoc;
use slog::Drain;

use std::env::current_dir;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;
use rocket::fs::FileServer;
use config_struct::BackendConfig;

use http_handlers::{submit, sessions::sessions_tracker::SessionsTracker};

#[launch]
fn rocket() -> _ 
{
    // Backend config loading
    let mut backend_config: BackendConfig = Figment::new()
        .merge(Yaml::file("BackendConfig.yaml"))
        .extract().unwrap();
    // TODO: cover cases when the path is already full
    backend_config.sessions_data_dir = current_dir().unwrap()
        .join(&backend_config.sessions_data_dir);
    backend_config.sessions_data_file_dir = current_dir().unwrap()
        .join(&backend_config.sessions_data_file_dir);

    // Logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    // Sessions tracker, wrapped in a mutex because it has to be mutable across threads
    let sessions_tracker;
    match SessionsTracker::from_file( &backend_config.sessions_data_file_dir)
    {
        Some(tr) => 
        {
            info!(log, "Read sessions data from: {:?}", 
                backend_config.sessions_data_file_dir);
            sessions_tracker = Arc::new(Mutex::new(tr
                .life_duration(&Duration::from_millis(backend_config.session_life_duration))));
        }
        None => 
        {
            info!(log, "Couldn't read sessions data from: {:?}", 
                backend_config.sessions_data_file_dir);
            sessions_tracker = Arc::new(Mutex::new(SessionsTracker::new()
                .life_duration(&Duration::from_millis(backend_config.session_life_duration))))
        }
    }

    info!(log, "Backend config:\n {:?}", backend_config);

    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![submit::post_submit])
        .manage(backend_config)
        .manage(sessions_tracker)
        .manage(log)
        .attach(AdHoc::on_liftoff("Sessions cleaner", |rocket| Box::pin(async move 
            {
                let tracker = rocket.state::<Arc<Mutex<SessionsTracker>>>()
                    .unwrap().to_owned();
                // TODO: cloning logger here is probably not right, look into it more
                let logger = rocket.state::<slog::Logger>().unwrap().to_owned();
                let interval = rocket.state::<BackendConfig>()
                    .unwrap().sessions_cleanup_interval;
                info!(logger, "Sessions cleaner started");
                
                thread::spawn(move ||
                {
                    loop
                    {
                        sleep(std::time::Duration::from_millis(interval));
                        let mut locked = tracker.lock().unwrap();
                        let deleted = locked.delete_old();
                        drop(locked);
                        info!(logger, "Deleted {} old sessions", deleted);
                    }
                });
            })))
            .attach(AdHoc::on_liftoff("Sessions saver", |rocket| Box::pin(async move 
                {
                    let tracker = rocket.state::<Arc<Mutex<SessionsTracker>>>()
                        .unwrap().to_owned();
                    // TODO: cloning logger here is probably not right, look into it more
                    let logger = rocket.state::<slog::Logger>().unwrap().to_owned();
                    let config = rocket.state::<BackendConfig>().unwrap();
                    let interval = config.sessions_save_interval;
                    let save_path = config.sessions_data_file_dir.clone();
                    info!(logger, "Sessions saver started");
                    
                    thread::spawn(move ||
                    {
                        loop
                        {
                            sleep(std::time::Duration::from_millis(interval));
                            let locked = tracker.lock().unwrap();
                            locked.save(&save_path);
                            drop(locked);
                            info!(logger, "Saved sessions data to a file");
                        }
                    });
                })))
}
