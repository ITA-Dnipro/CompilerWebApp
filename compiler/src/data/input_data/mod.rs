pub mod header;
pub mod options;
pub mod source_code;

pub struct InputData {
    header: Header;
    options: Options;
    source_code: SourceCode;
}