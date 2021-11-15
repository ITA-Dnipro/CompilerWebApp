mod structs;

use rocket::serde::json::Json;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::State;

use compiler::data::input_data::{InputData, compiler_type::CompilerType};
use compiler::handler::run_compilation;

use super::super::LangsInfo;
use std::path::PathBuf;
use super::super::filework::*;
use super::super::languages::{lang_info::LangInfo, flags_validator::FlagsValidator};

#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(compilation_json: Json<structs::InputData>,
    langs_info: &State<LangsInfo>) 
    -> Result<Json<structs::OutputData>, Custom<()>>
{
    let compilation_data = compilation_json.into_inner();
    if langs_info.contains_key(&compilation_data.lang)
    {
        let lang_info = &langs_info[&compilation_data.lang];
        match try_to_compile(&compilation_data, &lang_info)
        {
            Ok(comp_result) => 
                Ok(Json(comp_result)),
            Err(_) => Err(Custom(Status::InternalServerError, ()))
        }
    }
    else
    {
        Ok(Json(structs::OutputData::new(-1, "", "Unknown language")))
    }  
}

fn try_to_compile(compilation_data: &structs::InputData, lang_info: &LangInfo)
    -> Result<structs::OutputData, ()>
{
    let mut validator = FlagsValidator::new();
    // Input data validation
    if validator.validate(&compilation_data.options, &lang_info)
    {
        // Compilation goes here
        let source_file;
        match write_source_to_file(&compilation_data.code,
            &lang_info.lang_extension)
        {
            Some(path) => source_file = path,
            None => return Err(())
        }

        let compiler_input = InputData::new(CompilerType::Cpp, 
            PathBuf::from(&source_file),
            PathBuf::from(source_file.parent().unwrap()), 
            compilation_data.options.clone());
        // Compiler call
        let compilation_result;
        match run_compilation(&compiler_input)
        {
            Ok(comp_result) => compilation_result = comp_result,
            Err(err_msg) => return Ok(structs::OutputData::new(-1, "", err_msg))
        }

        delete_file(&source_file);
        if compilation_result.status_code.unwrap() == 0
        {
            // TODO: rework when compilation_result.compiled_file_name will contain full path
            delete_file(&source_file.parent().unwrap()
                .join(&compilation_result.compiled_file_name));
        }

        Ok(structs::OutputData::from_compiler_result(&compilation_result))
    }
    else 
    {
        let mut invalid_flags = format!("Options list contains prohibited flags: {}", 
            validator.invalid_flags.drain(0..).map(|flag| flag + ", ")
            .collect::<String>());
        invalid_flags.pop();    // ' '
        invalid_flags.pop();    // ','

        Ok(structs::OutputData::new(-1, "", &invalid_flags))
    }
}
