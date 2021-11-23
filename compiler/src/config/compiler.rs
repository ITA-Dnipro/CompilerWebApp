use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Compiler {
    pub version: String,
    pub options_whitelist: Vec<String>,
}


impl Compiler {
    pub fn new(version: String,
               options_whitelist: Vec<String>) -> Self {
        Self {
            version,
            options_whitelist
        }
    }
}