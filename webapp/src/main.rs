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
use slog::Drain;

use std::env::current_dir;
use std::sync::{Arc, Mutex};
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
    backend_config.sessions_data_dir = current_dir().unwrap().join(backend_config.sessions_data_dir);

    // Sessions tracker, wrapped in a mutex because it has to be mutable across threads
    let sessions_tracker = Arc::new(Mutex::new(SessionsTracker::new()));

    // Logger
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    info!(log, "Backend config:\n {:?}", backend_config);

    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![submit::post_submit])
        .manage(backend_config)
        .manage(sessions_tracker)
        .manage(log)
}
