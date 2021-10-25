pub mod header;
pub mod result;

use self::header::Header;
use self::result::Result;

pub struct OutputData {
    pub header: Header,
    pub result: Result,
}