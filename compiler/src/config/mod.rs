pub(crate) mod common;
pub(crate) mod compiler;

use std::path::{Path, PathBuf};
use serde::Deserialize;
use figment::{Figment, providers::Yaml, providers::Format};

use compiler::Compiler;
use common::Common;


#[derive(Debug, Deserialize)]
pub (crate) struct Config {
    pub common: Common,
    pub gcc: Compiler,
    pub rustc: Compiler,
}

impl Config {
    pub fn new(common: Common,
               gcc: Compiler,
               rustc: Compiler) -> Self {
        Self {
            common,
            gcc,
            rustc
        }
    }
}


pub(crate) fn load_config(config_file_path: PathBuf) -> Result<Config, &'static str> {
    
    if !Path::new(config_file_path.to_str().unwrap()).exists() {
        let config = load_default_config().unwrap(); 
        return Ok(config)       
    }
    
    let config = Figment::new()
                    .merge(Yaml::file(config_file_path))
                    .extract().unwrap(); 

    return Ok(config)    
}


pub(crate) fn load_default_config() -> Result<Config, &'static str> {

    // common
    let log_level: u32 = 1;
    let common = Common::new(log_level);

    // gcc
    let version = String::from("gcc 9.3.0");
    let mut options_whitelist: Vec<String> = Vec::new();
    options_whitelist.push("-v".to_owned());
    options_whitelist.push("--version".to_owned());
    options_whitelist.push("--verbose".to_owned());
    options_whitelist.push("--Wall".to_owned());
    let gcc = Compiler::new(version, options_whitelist);
    
    // rustc
    let version = String::from("rustc 1.58.0-nightly");
    let mut options_whitelist: Vec<String> = Vec::new();
    options_whitelist.push("-v".to_owned());
    options_whitelist.push("--version".to_owned());
    options_whitelist.push("--verbose".to_owned());
    options_whitelist.push("--Wall".to_owned());
    let rustc = Compiler::new(version, options_whitelist);

    let config = Config::new(common, gcc, rustc);

    Ok(config)
}

