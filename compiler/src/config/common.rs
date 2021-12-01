use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Common {
    pub(crate) log_level: u32,
}

impl Common {
    pub(crate) fn new(log_level: u32) -> Self {
        Self {
            log_level
        }
    }
}