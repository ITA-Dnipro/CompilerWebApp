pub mod cpp_runner;
use crate::data::output::OutputData;
pub(crate) trait Runner {
    fn run(&self) -> Result<OutputData, &'static str>;
}