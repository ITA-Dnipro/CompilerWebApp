use super::super::data::input_data::InputData;
use super::super::data::output_data::OutputData;

/// Provides interface for all of the compilers
pub(crate) trait Compiler {

    /// Runs compilation process
    ///
    /// # Arguments
    ///
    /// * `input_data` - A struct that holds input parameters for compiler (source code, compiler options, etc)
    ///
    /// # Result
    ///
    /// * A struct that holds compiled binary file and specific compiler output (stdout, stderr)
    /// 
    fn compile(&self, input_data: &InputData) -> Result<OutputData, &'static str>;
}
