#[derive(Debug)]
pub(crate) struct Compiler {
    version: String,
    options_whitelist: Vec<String>,
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