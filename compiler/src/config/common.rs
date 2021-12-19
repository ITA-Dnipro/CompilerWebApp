use serde::{Serialize, Deserialize};


/// Contains configuration data which is common for each compiler
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Common {
    pub(crate) log_level: u32,
}


impl Common {
    /// Constructor for this struct
    pub(crate) fn new(log_level: u32) -> Self {
        Self {
            log_level
        }
    }
}
