#![allow(unused)]

use std::path::PathBuf;
use std::process::Command;

use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::compiler::Compiler;

pub(crate) struct RustCompiler {

}

impl Compiler for RustCompiler {
    fn compile(&self, input_data: &InputData) -> Result<OutputData, &'static str> {
        let mut output_data = OutputData::new();
        // binary is unused now, so use "null" file
        // some "permission denied" error may occur,
        // so execute `chmod a+w /dev` as root
        let bin_file_name = "/dev/null";
        let mut command_to_run = Command::new("rustc");
        command_to_run.arg("-o");
        command_to_run.arg(bin_file_name);
        command_to_run.arg(input_data.source_code_file_path.to_str().unwrap());
        if (input_data.compiler_options != "")
        {
            for option in input_data.compiler_options.split(' ')
            {
                command_to_run.arg(option);
            }
        }
            
        let compiler_output = command_to_run
            .output()
            .expect("failed to execute process");
        
        output_data.status_code = compiler_output.status.code();
        output_data.compiled_file_name = PathBuf::from(bin_file_name);
        output_data.stdout = String::from_utf8(compiler_output.stdout.clone()).unwrap();
        output_data.stderr = String::from_utf8(compiler_output.stderr.clone()).unwrap();

        Ok(output_data)
    }

}