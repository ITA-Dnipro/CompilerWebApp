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

// Simplified structure for the very first step
pub struct OutputData {
    pub is_succes: bool,
    pub returnet_text: String,    
}