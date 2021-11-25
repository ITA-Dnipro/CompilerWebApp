use compiler::data::output_data::OutputData;
use serde::Serialize;

#[derive(Serialize)]
pub struct SubmitOutput
{
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String
}

impl SubmitOutput
{
    pub fn new(status_code: i32, stdout: &str, stderr: &str) -> SubmitOutput
    {
        SubmitOutput {
            status_code: status_code,
            stdout: String::from(stdout),
            stderr: String::from(stderr)
        }
    }

    pub fn from_compiler_result(data: &OutputData) -> SubmitOutput
    {
        SubmitOutput
        {
            status_code: data.status_code.unwrap_or(1),
            stdout: data.stdout.clone(),
            stderr: data.stderr.clone()
        }
    }
}
