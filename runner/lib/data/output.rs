#[derive(Debug)]
pub struct OutputData {
    pub stdout: String,
    pub stderr: String
}

impl OutputData {
    pub fn new<T>(stdout: T, stderr: T) -> Self 
        where (T, T): Into<OutputData>
    {
        (stdout, stderr).into()
    }
}

impl From<(&str, &str)> for OutputData {
    fn from((stdout, stderr): (&str, &str)) -> OutputData {
        OutputData {
            stdout: String::from(stdout), 
            stderr: String::from(stderr)
        }
    }
}

impl From<(String, String)> for OutputData {
    fn from((stdout, stderr): (String, String)) -> OutputData {
        OutputData {stdout, stderr}
    }
}
