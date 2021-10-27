#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::fs::FileServer;
use std::io::{Write, Seek, SeekFrom};

// Temporary working structures
#[derive(Deserialize)]
struct InputCode
{
    lang: String,
    options: String,
    code: String
}

#[derive(Serialize)]
struct OutputCode
{
    stdin: String,
    stdout: String,
    result: Vec<u8>
}

#[post("/submit", format = "json", data = "<compilation_json>")]
async fn post_submit(compilation_json: Json<InputCode>) 
    -> Result<Json<OutputCode>, String> // TODO: Err should be some struct with detailed info
{
    // Input data validation
    // TODO: validate input data: compilation options, language etc.


    // Compilation goes here
    // If error ever happens on work with temp files it's not on the user, 
    // so he should get "internal server error" here
    // TODO: talk about how to better assign temp folder
    // TODO: respond with "internal server error" explicitly?
    let mut code_file = tempfile::tempfile().expect("Panic on temp file creation");  
    code_file.write_all(compilation_json.code.as_bytes())
        .expect("Panic on copying of source code into the temp file");
    code_file.seek(SeekFrom::Start(0)).expect("Panic on temp file seek");
    // TODO: call compiler here

    
    drop(code_file);

    // Handle compiler's result here
    let some_result_path = "target/example/test.txt";
    let compiled_data = std::fs::read(some_result_path)
        .expect("Panic on reading compiler output");

    Ok(Json(OutputCode {
        stdin: "some stdin".to_owned(),
        stdout: "some stdout".to_owned(),
        result: compiled_data
    }))
}

#[launch]
fn rocket() -> _ 
{
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![post_submit])
}
