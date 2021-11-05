mod structs;

use std::path::PathBuf;
use rocket::serde::json::Json;
use super::super::filework::*;
use compiler::data::input_data::{InputData, compiler_type::CompilerType};
use compiler::handler::run_compilation;
use super::super::languages::{lang_info::LangInfo, flags_validator::FlagsValidator};

use rocket::State;
use super::super::LangsInfo;

#[post("/submit", format = "json", data = "<compilation_json>")]
pub async fn post_submit(compilation_json: Json<structs::InputData>,
    langs_info: &State<LangsInfo>) -> Json<structs::OutputData>
{
    let compilation_data = compilation_json.into_inner();
    if langs_info.contains_key(&compilation_data.lang)
    {
        let lang_info = &langs_info[&compilation_data.lang];
        let mut validator = FlagsValidator::new();
        Json(try_to_compile(&compilation_data, &lang_info, &mut validator))
    }
    else
    {
        Json(structs::OutputData::new(-1, "", "Unknown language"))
    }  
}

// TODO: change return type to Result when there'll be backend specific error cases
fn try_to_compile(compilation_data: &structs::InputData, lang_info: &LangInfo, 
    validator: &mut FlagsValidator) -> structs::OutputData
{
    // Input data validation
    if validator.validate(&compilation_data.options, &lang_info)
    {
        // TODO: respond with "internal server errors" explicitly?
        // Compilation goes here
        let source_file = write_source_to_file(&compilation_data.code,
            &compilation_data.lang).unwrap();
        let compiler_input = InputData::new(CompilerType::Cpp, PathBuf::from(&source_file),
            PathBuf::from(source_file.parent().unwrap()), compilation_data.options.clone());
        // Compiler call
        let compilation_result = run_compilation(&compiler_input);

        delete_file(&source_file);
        if compilation_result.status_code.unwrap() == 0
        {
            // TODO: rework when compilation_result.compiled_file_name will contain full path
            delete_file(&source_file.parent().unwrap().join(&compilation_result.compiled_file_name));
        }

        structs::OutputData::from_compiler_result(&compilation_result)
    }
    else 
    {
        let mut invalid_flags = format!("Options list contains prohibited flags: {}", 
            validator.invalid_flags.drain(0..).map(|flag| flag + ", ")
            .collect::<String>());
        invalid_flags.pop();    // ' '
        invalid_flags.pop();    // ','

        structs::OutputData::new(-1, "", 
            &invalid_flags)
    }
}
