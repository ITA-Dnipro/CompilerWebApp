#![allow(unused)]

use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::compiler::Compiler;

pub struct RustCompiler {

}

impl Compiler for RustCompiler {
    fn compile(&self, input_data: &InputData) -> OutputData {
        let output_data = OutputData {
            is_succes: false,
            status_code: Some(-1),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };

        //output_data.header = ...
        //output_data.result = ...


        output_data
    }

}