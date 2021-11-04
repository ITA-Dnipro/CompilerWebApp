#[macro_use] extern crate rocket;

mod http_handlers;
mod filework;

use http_handlers::submit;
use rocket::fs::FileServer;
use std::env::{set_var, current_dir};

#[launch]
fn rocket() -> _ 
{
    // Dir for compilation files
    let mut temp_dir = current_dir().unwrap().to_str().unwrap().to_owned();
    temp_dir.push_str("/tempdata");
    set_var("COMPILATION_TEMP_DIR", temp_dir);

    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![submit::post_submit])
}
