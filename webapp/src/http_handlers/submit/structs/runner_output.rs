use runner::data::output::OutputData;
use serde::Serialize;

#[derive(Serialize)]
pub struct RunnerOutput 
{
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String
}

impl RunnerOutput
{
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
