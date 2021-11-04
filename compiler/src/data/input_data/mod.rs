pub mod compiler_type;
pub mod header;
pub mod options;
pub mod source_code;


use std::path::PathBuf;


// Simplified structure for the very first step
use self::compiler_type::CompilerType;

pub struct InputData {
    pub compiler_type: CompilerType,
    pub source_code_file_path: PathBuf,  
    pub compiled_directory_path: PathBuf,
    pub compiler_options: String,
}

impl InputData {
    pub fn new(compiler_type: CompilerType,
               source_code_file_path: PathBuf,  
               compiled_directory_path: PathBuf,
               compiler_options: String) -> Self {
        Self {
            compiler_type,
            source_code_file_path,
            compiled_directory_path,
            compiler_options
        }
    }
}


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