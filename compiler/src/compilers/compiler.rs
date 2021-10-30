use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;


pub trait Compiler {
    fn compile(&self, input_data: &InputData) -> OutputData;
}
