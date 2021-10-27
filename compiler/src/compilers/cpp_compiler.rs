#![allow(unused)]

//use std::env;
use std::process::Command;

use super::compiler::Compiler;
use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::super::storage::name_generator::generate_filename;

pub struct CppCompiler {

}

impl Compiler for CppCompiler {
    fn compile(&self, input_data: &InputData) -> OutputData {
        
        let mut output_data = OutputData {
            is_succes: false,
            status_code: Some(-1),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };

        let bin_filename = generate_filename().to_owned();
        let mut full_bin_filename = "./temp/bin/".to_owned();
        full_bin_filename.push_str(&bin_filename);

        let compiler_output = Command::new("g++")
                //.current_dir(path)
                .args(["-o", &full_bin_filename, &input_data.source_code_filepath])
                .output()
                .expect("failed to execute process");
    
        output_data.is_succes = compiler_output.status.success();
        output_data.status_code = compiler_output.status.code();
        output_data.stderr = compiler_output.stderr;
        output_data.stdout = compiler_output.stdout;

        output_data
    }

}