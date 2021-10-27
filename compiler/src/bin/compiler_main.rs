
#![allow(unused)]

use std::env;
use std::process::Command;
use std::io::{self, Write};


fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    // TODO remove compiled bin file from directory (call "rm")
    
    // TODO read about using env variables - in Ubuntu "export varName = varValue"
    // read varName with program start - validate varValue!

    // TODO check g++ compiler installed

    //
    let path = env::current_dir()?;
    
    //println!("The current directory is {}", path.display());
    
    
    let output = Command::new("g++")
                .current_dir(path)
                .args(["-o", "./src/bin/test", "./src/bin/test.cpp"])
                .output()
                .expect("failed to execute process");
    

    if output.status.success() {
        

    } else {

    }


    let s: String = String::from(output.status);
                
    println!("Compiling status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success()); 
    
    Ok(())
}
