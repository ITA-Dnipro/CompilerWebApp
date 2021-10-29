mod structs;

use rocket::serde::json::Json;
use std::fs::{File, remove_file};
use std::io::{Write, Seek, SeekFrom};
use structs::{InputCode, OutputCode};
use std::env;

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

    remove_file(&source_file)
        .expect("Panic on deletion of input source code file");

    // Handling of the result
    // TODO: if result is Ok - write bin file into json, else respond with error messages

    Ok(Json(OutputCode {
        status_code: 0,
        stdout: "some stdout".to_owned(),
        stderr: "some stderr".to_owned()
    }))
}

fn write_source_to_file(source_code: &str, lang: &str) -> String
{
    // If error ever happens on work with temp files it's not on the user, 
    // so he should get "internal server error" here
    let mut input_file_name = env::var("COMPILATION_TEMP_DIR")
        .expect("COMPILATION_TEMP_DIR doesn't exist");
    input_file_name.push_str("/");
    input_file_name.push_str(
        &chrono::Utc::now()
        .format("compilation_input-%Y-%m-%d-%H-%M-%S-%f").to_string()
    );
    input_file_name.push_str(".cpp");     // TODO: add lang specific extensions

    let mut code_file = File::create(&input_file_name)
        .expect("Panic on temp file creation");  
    code_file.write_all(source_code.as_bytes())
        .expect("Panic on copying of source code into the temp file");
    code_file.seek(SeekFrom::Start(0)).expect("Panic on temp file seek");
    
    drop(code_file);

    input_file_name
}
