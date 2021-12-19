use serde::{Serialize, Deserialize};

/// Contains configuration data which is specific for each compiler (version, whitelist of options)
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Compiler {
    pub(crate) version: String,
    pub(crate) options_whitelist: Vec<String>,
}


impl Compiler {
    /// Constructor for this struct
    pub(crate) fn new(version: String,
               options_whitelist: Vec<String>) -> Self {
        Self {
            version,
            options_whitelist
        }
    }
}
