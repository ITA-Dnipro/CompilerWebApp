
#![allow(unused)]

use std::env;
use std::process::Command;
use std::io::{self, Write};


fn main() -> std::io::Result<()> {
    println!("Hello from [compiler] crate!");

    let path = env::current_dir()?;
    //println!("The current directory is {}", path.display());
    
    
    let output = Command::new("g++")
                .current_dir(path)
                .args(["-o", "./src/bin/test", "./src/bin/test.cpp"])
                .output()
                .expect("failed to execute process");
    

    println!("Compiling status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success()); 
    
    Ok(())
}
