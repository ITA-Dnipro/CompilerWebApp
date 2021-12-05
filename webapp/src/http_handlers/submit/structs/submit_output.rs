use super::RunnerOutput;
use serde::Serialize;

/// ## Response body of `POST /submit`
/// ----
/// Fields:
/// ---
/// _Compilatin result._
/// * `status_code` - status code of the compilation;
/// * `stdout` - output stream data of the compilation;
/// * `stderr` - error stream data of the compilation;
/// ----
/// _Runner result._
/// * `runner_output` - results of code execution. Can be ommited, in which case it will not be serialized.
#[derive(Serialize)]
pub struct SubmitOutput
{
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_output: Option<RunnerOutput>
}

impl SubmitOutput
{
    /// Creates a new instance with compilation results.
    pub fn new(status_code: i32, stdout: &str, stderr: &str) -> SubmitOutput
    {
        SubmitOutput {
            status_code: status_code,
            stdout: String::from(stdout),
            stderr: String::from(stderr),
            runner_output: None
        }
    }

    /// ## Creates a new instance from compilation results
    /// ----
    /// Args:
    /// ---
    /// * `data` - output structure, returned by the compiler.
    pub fn from_compiler_result(data: &compiler::data::output_data::OutputData)
        -> SubmitOutput
    {
        SubmitOutput
        {
            status_code: data.status_code.unwrap_or(1),
            stdout: data.stdout.clone(),
            stderr: data.stderr.clone(),
            runner_output: None
        }
    }

    /// ## Sets `runner_output` field of the struct.
    /// ----
    /// Args:
    /// ---
    /// * `runner_out` - output structure, returned by the runner.
    pub fn set_runner_output(&mut self, runner_out: runner::data::output::OutputData)
        -> &mut Self
    {
        self.runner_output = Some(RunnerOutput::from_runner_result(runner_out));

        self
    }
}
