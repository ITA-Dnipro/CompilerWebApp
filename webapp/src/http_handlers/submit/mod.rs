mod structs;

use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::State;
use slog::Logger;

use compiler::data::input_data::{InputData, compiler_type::CompilerType};
use compiler::handler::run_compilation;

use super::super::BackendConfig;
use std::path::Path;
use super::super::filework::*;
use super::sessions::session::Session;

// Submit code and get compilation results
#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(
    compilation_json: Json<structs::InputData>,
    config: &State<BackendConfig>, 
    logger: &State<Logger>, 
    session: Session) 
    -> Result<Json<structs::OutputData>, Custom<()>>
{
    trace!(logger, "Entered post_submit");
    // Source code saving
    let source_file;
    let session_id_str = session.id.to_string();
    match save_source(&compilation_json.code,
        &config.lang_extensions[&compilation_json.lang],
        &Path::new(&config.sessions_data_dir.join(&session_id_str)),
        &session_id_str,
        &logger)
    {
        Some(path) => source_file = path,
        None => 
        {
            return Err(Custom(Status::InternalServerError, ()));
        }
    }
    trace!(logger, "Source code file created: {:?}", source_file);
    // Compilation
    let compilation_data = compilation_json.into_inner();
    if config.lang_extensions.contains_key(&compilation_data.lang)
    {
        match try_to_compile(&compilation_data, &source_file, &logger)
        {
            Ok(comp_result) => 
                Ok(Json(comp_result)),
            Err(_) => 
            {
                error!(logger, "Fatal error while compiling");
                Err(Custom(Status::InternalServerError, ()))
            }
        }
    }
    else
    {
        Ok(Json(structs::OutputData::new(-1, "", "Unknown language")))
    }  
}

fn try_to_compile(
    compilation_data: &structs::InputData, 
    source_code: &Path,
    logger: &Logger)
    -> Result<structs::OutputData, ()>
{
    // Compilation goes here
    // TODO: figure out how to properly deduce CompilerType from String
    let compiler_input = InputData::new(CompilerType::Cpp, 
        source_code.to_owned(),
        source_code.parent().unwrap().to_owned(), 
        compilation_data.options.clone());
    // Compiler call
    let compilation_result;
    match run_compilation(&compiler_input)
    {
        Ok(comp_result) => compilation_result = comp_result,
        Err(err_msg) => 
        {
            error!(logger, "Compilation failed with error message: {}", err_msg);
            
            return Ok(structs::OutputData::new(-1, "", err_msg));
        }
    }

    Ok(structs::OutputData::from_compiler_result(&compilation_result))
}
