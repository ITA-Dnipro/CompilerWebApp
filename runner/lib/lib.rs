mod data;
mod filter;
mod runner;
use std::{process, str};
use crate::data::output::OutputData;
use compiler::data::input_data::compiler_type::CompilerType;
use runner::{Runner, cpp_runner::CppRunner};
use slog::Logger;

/// Runs user's code
/// # Args
/// * `lang` - CompilerType's enum variant, corresponds to compiled language
/// * `object_path` - path to shared object
/// * `logger` - logger
/// # Result
/// * `Result<OutputData, &'static str>`
/// * `OutputData` is a struct that contains results of running code: stdout, stderr
///
pub fn run_code<'time>(
    lang: CompilerType,
    object_path: &'static str,
    logger: &'time Logger,
) -> Result<OutputData, &'static str> {
    let runner: Box<dyn Runner> = match lang {
        CompilerType::Cpp => Box::new(CppRunner::new(object_path, logger)),
        _ => return Err("Not implemented"),
    };

    runner.run()
}


