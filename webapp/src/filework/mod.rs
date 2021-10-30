use std::fs::{File, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::env;

pub fn write_source_to_file(source_code: &str, lang: &str) -> String
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

pub fn delete_file(filename: &str)
{
    remove_file(&filename)
        .expect("Panic on deletion of input source code file");
}
