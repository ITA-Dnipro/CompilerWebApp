#![allow(unused)]

use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Hello from [compiler] crate!");
    
    let output = Command::new("g++")
                .args(["-o", "test", "test.cpp"])
                .spawn();
    

    //println!("{}", output.err().unwrap());

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());    
}