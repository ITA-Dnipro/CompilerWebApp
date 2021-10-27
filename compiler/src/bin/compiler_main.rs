
#![allow(unused)]

extern crate compiler;

use std::env;
use std::process::Command;
use std::io::{self, Write};


use compiler::handler::run_compilation;
use compiler::data::input_data::InputData;
use compiler::data::input_data::compiler_type::CompilerType;


fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    let input_data = InputData {
        compiler_type: CompilerType::Cpp,
        source_code: String::from("Here will be filepath to source code"),
    };

    let output_data = run_compilation(&input_data);

    println!("Compiling status: {}", output_data.status_code.unwrap());
    io::stdout().write_all(&output_data.stdout).unwrap();
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
