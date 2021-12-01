use std::path::PathBuf;

pub struct OutputData {
    pub status_code: Option<i32>,
    pub compiled_file_name: PathBuf,
    pub stdout: String,
    pub stderr: String, 
}

impl OutputData {
    pub fn new () -> Self {
        OutputData { 
            status_code: Some(-1), 
            compiled_file_name: PathBuf::from(""), 
            stdout: String::new(), 
            stderr: String::new() 
        }
    }
}
