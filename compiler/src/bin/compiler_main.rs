
//#![allow(unused)]

extern crate compiler;

use std::io::{self, Write};
use std::path::PathBuf;

use compiler::handler::run_compilation;
use compiler::data::input_data::InputData;
use compiler::data::input_data::compiler_type::CompilerType;

fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    let input_data = InputData {
        compiler_type: CompilerType::Cpp,
        source_code_file_path: PathBuf::from("./temp/src/test.cpp"),
        compiled_directory_path: PathBuf::from("./temp/bin/"), 
        compiler_options: String::from("-g"),
    };

    let output_data = run_compilation(&input_data);

    println!(">> Compiling status: {}", output_data.status_code.unwrap());
    println!(">> Compiled file path: {}", output_data.compiled_file_name.into_os_string().into_string().unwrap());
    println!(">> Value of stdout:");
    io::stdout().write_all(&output_data.stdout).unwrap();
    println!(">> Value of stderr:");
    io::stderr().write_all(&output_data.stderr).unwrap();
    
    Ok(())
}
    // TODO remove compiled bin file from directory (call "rm")
    // TODO read about using env variables - in Ubuntu "export varName = varValue"
    //   read varName with program start - validate varValue!
    // TODO check g++ compiler installed
    // TODO test - assert!(output.status.success()); 