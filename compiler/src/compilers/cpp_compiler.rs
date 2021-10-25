use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;
use super::compiler::Compiler;

pub struct CppCompiler {

}

impl Compiler for CppCompiler {
    fn Compile(&self, input_data: InputData) -> OutputData {
        let output_data: OutputData;

        //output_data.header = ...
        //output_data.result = ...

        output_data
    }

}