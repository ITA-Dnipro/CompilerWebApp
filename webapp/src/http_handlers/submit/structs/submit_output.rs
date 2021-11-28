use super::RunnerOutput;
use serde::Serialize;

#[derive(Serialize)]
pub struct SubmitOutput
{
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub runner_output: Option<RunnerOutput>
}

impl SubmitOutput
{
    pub fn new(status_code: i32, stdout: &str, stderr: &str) -> SubmitOutput
    {
        SubmitOutput {
            status_code: status_code,
            stdout: String::from(stdout),
            stderr: String::from(stderr),
            runner_output: None
        }
    }

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

    pub fn append_runner_output(&mut self, runner_out: runner::data::output::OutputData)
        -> &mut Self
    {
        self.runner_output = Some(RunnerOutput::from_runner_result(runner_out));

        self
    }
}
