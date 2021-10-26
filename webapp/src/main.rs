#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::fs::FileServer;

// Temporary working structures
#[derive(Deserialize)]
struct InputCode
{
    code: String,
    lang: String,
    options: String
}

#[derive(Serialize)]
struct OutputCode
{
    stdin: String,
    stdout: String,
    result: Vec<u8>
}

#[post("/submit", format = "json", data = "<compilation_data>")]
async fn post_submit(compilation_data: Json<InputCode>) 
    -> Result<Json<OutputCode>, String> // TODO: Err should be some struct with detailed info
{
    // Input data validation
    // TODO: validate input data: compilation options, language etc.

    // Compilation goes here
    let compilation_args = compilation_data.into_inner();
    // TODO: call compiler here

    // Handle compiler's result here
    let some_result_path = "target/example/test.txt";
    let compiled_data = std::fs::read(some_result_path);
    
    match compiled_data
    {
        Ok(bytes) => Ok(Json(OutputCode {
            stdin: "some stdin".to_owned(),
            stdout: "some stdout".to_owned(),
            result: bytes
        })),
        Err(_) => Err("Error while reading compiler file.\
            Compilation error or smthn, i dunno".to_owned())
    }    
}

#[launch]
fn rocket() -> _ 
{
    // I'm not sure about FileServer, but it seems safe.
    // And official tutorials use it too
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![post_submit])
}
