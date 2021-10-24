#[macro_use] extern crate rocket;

// TODO: receive application code to compile
#[post("/submit", data = "<code>")]
fn post_submit(code: String) -> String
{
    // TODO: send back compiler file
    "i work: ".to_owned() + &code
}

#[launch]
fn rocket() -> _ 
{
    // I'm not sure about FileServer, but it seems safe.
    // And official tutorials use it too
    rocket::build()
        .mount("/", rocket::fs::FileServer::from("static/"))
        .mount("/", routes![post_submit])
}
