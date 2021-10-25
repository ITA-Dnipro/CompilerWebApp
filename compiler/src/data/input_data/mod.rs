pub mod compiler_type;
pub mod header;
pub mod options;
pub mod source_code;

use self::header::Header;
use self::options::Options;
use self::source_code::SourceCode;

pub struct InputData {
    pub header: Header,
    pub options: Options,
    pub source_code: SourceCode,
}