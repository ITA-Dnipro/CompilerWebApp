#![allow(unused)]

use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::compiler::Compiler;

pub struct CppCompiler {

}

impl Compiler for CppCompiler {
    fn compile(&self, input_data: &InputData) -> OutputData {
        let output_data = OutputData {
            is_succes: false,
            returnet_text: String::from("Empty result"),
        };
 
        //output_data.header = ...
        //output_data.result = ...

        output_data
    }

}