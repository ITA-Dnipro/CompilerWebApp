mod structs;

use super::super::filework::*;
use rocket::serde::json::Json;
use compiler::data::input_data::{InputData, compiler_type::CompilerType};
use compiler::handler::run_compilation;
use std::path::PathBuf;

#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(compilation_json: Json<structs::InputData>) 
    -> Json<structs::OutputData>
{
    // TODO: respond with "internal server errors" explicitly?
    // Input data validation
    // TODO: validate input data: compilation options, language etc.

    // Compilation goes here
    let source_file = write_source_to_file(&compilation_json.code,
        &compilation_json.lang);
    let compiler_input = InputData::new(CompilerType::Cpp, PathBuf::from(&source_file),
        PathBuf::from(source_file.parent().unwrap()), compilation_json.options.clone());
    // Compiler call
    let compilation_result = run_compilation(&compiler_input);

    delete_file(&source_file);
    if compilation_result.status_code.unwrap() == 0
    {
        // TODO: rework when compilation_result.compiled_file_name will contain full path
        delete_file(&source_file.parent().unwrap().join(&compilation_result.compiled_file_name));
    }

    // Handling of the result
    Json(structs::OutputData::from_compiler_result(&compilation_result))
}
