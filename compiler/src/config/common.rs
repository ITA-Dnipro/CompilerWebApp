use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Common {
    pub log_level: u32,
}

impl Common {
    pub fn new(log_level: u32) -> Self {
        Self {
            log_level
        }
    }
}