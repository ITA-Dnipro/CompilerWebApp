use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Common {
    log_level: u32,
}

impl Common {
    pub fn new(log_level: u32) -> Self {
        Self {
            log_level
        }
    }
}