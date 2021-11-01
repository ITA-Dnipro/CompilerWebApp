pub mod header;
pub mod result;

use std::path::PathBuf;

// Simplified structure for the very first step - based on Struct std::process::Output
pub struct OutputData {
    pub status_code: Option<i32>,
    pub compiled_file_name: PathBuf,
    pub stdout: String,
    pub stderr: String, 
}

// TODO implement this complete structure 
/*
use self::header::Header;
use self::result::Result;

pub struct OutputData {
    pub header: Header,
    pub result: Result,
}
*/