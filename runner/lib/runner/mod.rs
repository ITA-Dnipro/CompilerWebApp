pub mod cpp_runner;
mod lib_wrapper;
use crate::data::output::OutputData;
use crate::data::error::Error;
pub(crate) trait Runner<'time> {
    fn run(&self) -> Result<OutputData, Error>;
}