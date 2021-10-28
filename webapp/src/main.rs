#[macro_use] extern crate rocket;

mod http_handlers;

use http_handlers::submit;
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ 
{
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![submit::post_submit])
}
