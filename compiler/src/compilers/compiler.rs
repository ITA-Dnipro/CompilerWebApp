use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;


pub(crate) trait Compiler {
    fn compile(&self, input_data: &InputData) -> Result<OutputData, &'static str>;
}
