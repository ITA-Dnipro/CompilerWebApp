use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Compiler {
    pub(crate) version: String,
    pub(crate) options_whitelist: Vec<String>,
}


impl Compiler {
    pub(crate) fn new(version: String,
               options_whitelist: Vec<String>) -> Self {
        Self {
            version,
            options_whitelist
        }
    }
}
