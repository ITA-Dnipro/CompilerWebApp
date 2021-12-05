use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::State;
use slog::Logger;

use compiler::data::input_data::{InputData, compiler_type::CompilerType};
use compiler::handler::run_compilation;
use runner::{run_code, data::output::OutputData};

use crate::http_handlers::submit::structs::{SubmitInput, SubmitOutput, SubmitHeaders};
use crate::http_handlers::sessions::{Session, SessionsTracker};
use crate::config_struct::BackendConfig;
use crate::filework::save_source;

/// ## `POST /submit` handler.
/// ----
/// ## Args
/// * `compilation_json` - request body JSON with compilation data;
/// * `submit_options` - additional submission options;
/// * `config_lock` - `RwLock` with server configuration. Only locked for reading;
/// * `logger` - logger to log to;
/// * `tracker` - `Arc` with server sessions tracker object;
/// * `session` - info about current user session.
/// ----
/// Returns:
/// ---
/// `Result` with `Json<SubmitOutput>` as `Ok` value, or a response of `500 internal server error`.
#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(
    compilation_json: Json<SubmitInput>, 
    submit_options: SubmitHeaders,
    config_lock: &State<RwLock<BackendConfig>>, 
    logger: &State<Arc<Logger>>,
    tracker: &State<Arc<SessionsTracker>>,
    session: Session) 
    -> Result<Json<SubmitOutput>, Custom<()>>
{
    let config = config_lock.read().unwrap_or_else(|_| std::process::exit(1));

    if !config.lang_extensions.contains_key(&compilation_json.lang)
    {
        return Ok(Json(SubmitOutput::new(-1, "", "Unknown language")));
    }

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
        None => return Err(Custom(Status::InternalServerError, ()))
    }
    tracker.set_source_file(&session.id, &source_file);
    trace!(logger, "Source code file created: {:?}", source_file);

    // Compilation
    let lang_type = if let Ok(lang_type) = get_lang_type(&compilation_json.lang)
    {
        lang_type
    }
    else
    {
        return Err(Custom(Status::InternalServerError, ()));
    };

    let mut compilation_data = compilation_json.into_inner();
    // TODO: come up with something less hardcoded
    compilation_data.options += "-fpic -shared";    // Required for the runner
    let mut response_obj: SubmitOutput;
    let binary_path;

    match try_compile(lang_type, &compilation_data, &source_file, &logger)
    {
        Ok((comp_result, binary)) => 
        {
            binary_path = source_file.parent().unwrap().join(binary);
            debug!(logger, "binary_path {:?}", binary_path);
            response_obj = comp_result;
        },
        Err(_) => 
        {
            error!(logger, "Fatal error while compiling");
            return Err(Custom(Status::InternalServerError, ()));
        }
    }

    // TODO: remove when CompilerType will be clonable
    let lang_type = get_lang_type(&compilation_data.lang).unwrap();

    // Execution
    if submit_options.execute && response_obj.status_code == 0
    {
        match try_execute(lang_type, &binary_path, &logger)
        {
            Ok(exec_output) => 
            {
                response_obj.set_runner_output(exec_output);
            },
            Err(_) => 
            {
                return Err(Custom(Status::InternalServerError, ()));
            },
        }
    }
    
    Ok(Json(response_obj))
}

/// Returns language enum required by the compiler and the runner
fn get_lang_type(name: &str) -> Result<CompilerType, ()>
{
    // TODO: come up with something less hardcoded
    match name
    {
        "c++" => Ok(CompilerType::Cpp),
        "rust" => Ok(CompilerType::Rust),
        _ => Err(())
    }
}

/// Tries to compile a given source code
fn try_compile(
    lang: CompilerType,
    compilation_data: &SubmitInput, 
    source_code: &Path,
    logger: &Logger)
    -> Result<(SubmitOutput, PathBuf), ()>
{
    // Compilation goes here
    let compiler_input = InputData::new(lang, 
        source_code.to_owned(),
        source_code.parent().unwrap().to_owned(), 
        compilation_data.options.clone());
    let compilation_result;
    // Compiler call
    match run_compilation(&compiler_input)
    {
        Ok(comp_result) => compilation_result = comp_result,
        Err(err_msg) => 
        {
            error!(logger, "Compilation failed with error message: {}", err_msg);
            
            return Ok((SubmitOutput::new(-1, "", err_msg), PathBuf::new()));
        }
    }

    Ok((SubmitOutput::from_compiler_result(&compilation_result),
        compilation_result.compiled_file_name))
}

/// Tries to execute a given binary
fn try_execute(
    lang: CompilerType,
    binary_path: &Path, 
    logger: &Logger)
    -> Result<OutputData, ()>
{  
    match run_code(lang, binary_path.to_owned(), logger)
    {
        Ok(output) => 
        {
            Ok(output)
        },
        Err(e) => 
        {
            error!(logger, "Runner failed: {:?}", e);

            Err(())
        }
    }
}
