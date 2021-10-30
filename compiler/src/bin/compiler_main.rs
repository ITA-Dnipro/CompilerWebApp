
#![allow(unused)]

extern crate compiler;

use std::env;
use std::process::Command;
use std::io::{self, Write};
use std::path::PathBuf;


use compiler::handler::run_compilation;
use compiler::data::input_data::InputData;
use compiler::data::input_data::compiler_type::CompilerType;


fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    let mut input_data = InputData {
        compiler_type: CompilerType::Cpp,
        source_code_filepath: PathBuf::from("./temp/src/test.cpp"),
        compiler_options: String::from("-g"),
    };

    let output_data = run_compilation(&input_data);

    println!("Compiling status: {}", output_data.status_code.unwrap());

    println!("\nValue of stdout:");
    io::stdout().write_all(&output_data.stdout).unwrap();
    println!("\nValue of stderr:");
    io::stderr().write_all(&output_data.stderr).unwrap();
    


    
    // TODO remove compiled bin file from directory (call "rm")
    
    // TODO read about using env variables - in Ubuntu "export varName = varValue"
    // read varName with program start - validate varValue!

    // TODO check g++ compiler installed

    /*
    //
    let path = env::current_dir()?;
    
    //println!("The current directory is {}", path.display());
    
    
    let output = Command::new("g++")
                .current_dir(path)
                .args(["-o", "./src/bin/test", "./src/bin/test.cpp"])
                .output()
                .expect("failed to execute process");
    

    //let s: String = String::from(output.status);
                
    println!("Compiling status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success()); 
    */
    Ok(())

    
}
