mod structs;

use super::super::filework::*;
use rocket::serde::json::Json;
use structs::{InputCode, OutputCode};


#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(compilation_json: Json<InputCode>) 
    -> Result<Json<OutputCode>, String> // TODO: Err should be some struct with detailed info
{
    // TODO: respond with "internal server errors" explicitly?
    // Input data validation
    // TODO: validate input data: compilation options, language etc.

    // Compilation goes here
    let source_file = write_source_to_file(&compilation_json.code,
        &compilation_json.lang);
    // TODO: call compiler here

    delete_file(&source_file);

    // Handling of the result
    // TODO: if result is Ok - write bin file into json, else respond with error messages

    Ok(Json(OutputCode {
        status_code: 0,
        stdout: "some stdout".to_owned(),
        stderr: "some stderr".to_owned()
    }))
}
