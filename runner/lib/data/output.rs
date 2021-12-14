/// Struct to hold results of running user code
/// 
/// Fields:
/// * `stdout` both stdout and stderr of runned code
/// * `stderr` error messages, that descripe what went wrong
/// * `exit_code` process's exit exit
/// 
/// Stdout and Stderr are both redirected into `stdout`, so it may
/// lead to some confusion, naming convention is temporary and has to be
/// changed in the future
#[derive(Default)]
#[derive(Debug)]
pub struct OutputData {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl OutputData {
    pub fn new<T>(stdout: T, stderr: T, exit_code: i32) -> Self 
        where (T, T, i32): Into<OutputData>
    {
        (stdout, stderr, exit_code).into()
    }
}

impl From<(&str, &str, i32)> for OutputData {
    fn from((stdout, stderr, exit_code): (&str, &str, i32)) -> OutputData {
        OutputData {
            stdout: String::from(stdout), 
            stderr: String::from(stderr),
            exit_code
        }
    }
}

impl From<(String, String, i32)> for OutputData {
    fn from((stdout, stderr, exit_code): (String, String, i32)) -> OutputData {
        OutputData {stdout, stderr, exit_code}
    }
}
