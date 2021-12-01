
//#![allow(unused)]

extern crate compiler;

use std::path::PathBuf;

use compiler::handler::run_compilation;
use compiler::data::input_data::InputData;
use compiler::data::input_data::compiler_type::CompilerType;

use std::io::{Error, ErrorKind};

/// Use case of compiler - here: compiling cpp file
fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    let input_data = InputData {
        compiler_type: CompilerType::Cpp,
        source_code_file_path: PathBuf::from("./temp/src/test.cpp"),
        compiled_directory_path: PathBuf::from("./temp/bin/"), 
        compiler_options: String::from("-g --std=c++17 -Wall -o"),        
    };

    let output_data = run_compilation(&input_data);


    match output_data {
        Ok(output_data) => {
            println!(">> Compiling status: {}", output_data.status_code.unwrap());
            println!(">> Compiled file path: {}", output_data.compiled_file_name.into_os_string().into_string().unwrap());
    
            println!(">> Value of stdout:");
            println!("{}", output_data.stdout);
    
            println!(">> Value of stderr:");
            println!("{}", output_data.stderr);
    
            Ok(())
        }

        Err(e) => {
            println!("Error compiling: {:?}", e);
            Err(Error::new(ErrorKind::Other, e))
        }
    } 
}    