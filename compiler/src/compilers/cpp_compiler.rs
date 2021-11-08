#![allow(unused)]

//use std::env;
use std::path::PathBuf;
use std::process::Command;

use super::compiler::Compiler;
use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::super::storage::name_generator::generate_filename;

pub(crate) struct CppCompiler {

}


impl Compiler for CppCompiler {
    fn compile(&self, input_data: &InputData) -> Result<OutputData, &'static str> {

        let mut output_data = OutputData {
            status_code: Some(-1),
            compiled_file_name: PathBuf::from(""),
            stdout: String::new(),
            stderr: String::new(),
        };


        output_data.stdout.push_str("Compilation started...\n");

        let bin_file_name = generate_filename().to_owned();
        
        let mut full_bin_filename = PathBuf::new();
        full_bin_filename.push(&input_data.compiled_directory_path);
        full_bin_filename.push(&bin_file_name);
        
        let mut output_binary_argument = String::from("-o");
        output_binary_argument.push_str(&full_bin_filename.into_os_string().into_string().unwrap());


        output_data.stdout.push_str("Binary file name generated...\n");
        
        output_data.stdout.push_str("Running compiler...\n");

        let mut compiler_command = Command::new("g++");

        if input_data.compiler_options != "" {
            compiler_command.arg(&input_data.compiler_options);
        }

        compiler_command.arg(output_binary_argument);
        compiler_command.arg(input_data.source_code_file_path.to_str().unwrap());

        let compiler_output = compiler_command.output().expect("failed to execute process");


        output_data.status_code = compiler_output.status.code();
        output_data.compiled_file_name = PathBuf::from(bin_file_name);
        
        output_data.stdout.push_str(&String::from_utf8(compiler_output.stdout.clone()).unwrap());
        output_data.stdout.push_str(&format!("Compilation finished with code: {:?}\n", &output_data.status_code.unwrap()));
        
        output_data.stderr = String::from_utf8(compiler_output.stderr.clone()).unwrap();        
      
        Ok(output_data)
    }

}