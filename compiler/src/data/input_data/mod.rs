pub mod compiler_type;
pub mod header;
pub mod options;
pub mod source_code;

// TODO implement this complete structure 
/*
use self::header::Header;
use self::options::Options;
use self::source_code::SourceCode;

pub struct InputData {
    pub header: Header,
    pub options: Options,
    pub source_code: SourceCode,
}
*/

// Simplified structure for the very first step
use self::compiler_type::CompilerType;

pub struct InputData {
    pub compiler_type: CompilerType,
    pub source_code_filepath: String,    
}