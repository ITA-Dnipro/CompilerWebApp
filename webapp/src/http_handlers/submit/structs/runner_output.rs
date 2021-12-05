use runner::data::output::OutputData;
use serde::Serialize;

/// ## Response body's `runner_output` field.
/// ----
/// Args:
/// ---
/// * `exit_code` - status code of the execution;
/// * `stdout` - output stream data of the execution;
/// * `stderr` - error stream data of the execution;
#[derive(Serialize)]
pub struct RunnerOutput 
{
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String
}

impl RunnerOutput
{
    /// Create a RunnerOutput instance from runner's result representation
    pub fn from_runner_result(result: OutputData) -> Self
    {
        RunnerOutput 
        {
            exit_code: result.exit_code,
            stdout: result.stdout,
            stderr: result.stderr
        }
    }
}
