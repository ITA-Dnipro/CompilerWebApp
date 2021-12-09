#![feature(thread_is_running)]
pub mod data;
mod filter;
mod config;
mod runner;
use std::{path::PathBuf};
use crate::data::output::OutputData;
use crate::data::error::Error;
use compiler::data::input_data::compiler_type::CompilerType;
use runner::{Runner, cpp_runner::CppRunner};
use slog::Logger;

/// Runs user's code
/// # Args
/// * `lang` - CompilerType's enum variant, corresponds to compiled language
/// * `object_path` - path to shared object
/// * `logger` - logger
/// # Result
/// * `Result<OutputData, Error>`
/// * `OutputData` is a struct that contains results of running code: stdout, stderr
///
pub fn run_code<'time>
(
    lang: CompilerType,
    object_path: PathBuf,
    logger: &'time Logger,
) -> Result<OutputData, Error> 
{
    let runner: Box<dyn Runner + 'time> = match lang 
    {
        CompilerType::Cpp => 
        {

            Box::new(CppRunner::new(object_path, logger)?)
        },
        _ => return Err
        (
            Error::NotImplementedError(String::from("Not implemented"))
        ),
    };

    runner.run()
}


