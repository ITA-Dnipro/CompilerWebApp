pub mod header;
pub mod result;


// TODO implement this complete structure 
/*
use self::header::Header;
use self::result::Result;

pub struct OutputData {
    pub header: Header,
    pub result: Result,
}
*/

// Simplified structure for the very first step - based on Struct std::process::Output
pub struct OutputData {
    pub is_succes: bool,
    pub status_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>, 
}