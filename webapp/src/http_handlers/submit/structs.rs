#![allow(unused)]
use rocket::serde::{Serialize, Deserialize};
use compiler::data::output_data;

#[derive(Deserialize)]
pub struct InputData
{
    pub lang: String,
    pub options: String,
    pub code: String
}

#[derive(Serialize)]
pub struct OutputData
{
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String
}

impl OutputData
{
    pub fn new(status_code: i32, stdout: &str, stderr: &str) -> OutputData
    {
        OutputData {
            status_code: status_code,
            stdout: String::from(stdout),
            stderr: String::from(stderr)
        }
    }

    pub fn from_compiler_result(data: &output_data::OutputData) -> OutputData
    {
        OutputData
        {
            status_code: data.status_code.unwrap_or(1),
            stdout: String::from_utf8(data.stdout.clone())
                .unwrap_or_default(),
            stderr: String::from_utf8(data.stderr.clone())
                .unwrap_or_default()
        }
    }
}
