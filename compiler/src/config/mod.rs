pub(crate) mod common;
pub(crate) mod compiler;

use std::path::{Path, PathBuf};
use serde::Deserialize;
use figment::Figment;
//use figment::providers::{Format, Yaml};

use compiler::Compiler as CompilerConfig;
use common::Common;

//use self::common;
//use self::language;

#[derive(Debug)]
pub (crate) struct Config {
    common: Common,
    gcc: CompilerConfig,
    rustc: CompilerConfig,
}

impl Config {
    pub fn new(common: Common,
               gcc: CompilerConfig,
               rustc: CompilerConfig) -> Self {
        Self {
            common,
            gcc,
            rustc
        }
    }
}


pub(crate) fn load_config(config_file_path: PathBuf) -> Result<Config, &'static str> {
    
    let config: Config::new();
    
    if !Path::new(config_file_path).exists() {
        config = load_default_config().unwrap(); 
        return Ok(config)       
    }
    
    let config = Figment::new()
                    .merge(Yaml::file(config_file_path))
                    .extract().unwrap(); 

    return Ok(config)    
}


pub(crate) fn load_default_config() -> Result<Config, &'static str> {

    // gcc
    let log_level: u32 = 1;
    common: <Common as Trait>::new(log_level);

    let version = String::from("gcc 9.3.0");
    let options_whitelist = Vec::new();
    options_whitelist.push("-v");
    options_whitelist.push("--version");
    options_whitelist.push("--verbose");
    options_whitelist.push("--Wall");
    let cpp: CompilerConfig::new(common, options_whitelist);
    
    // rustc
    let log_level: u32 = 1;
    common: <Common as Trait>::new(log_level);

    let version = String::from("rustc 1.58.0-nightly");
    let options_whitelist = Vec::new();
    options_whitelist.push("-v");
    options_whitelist.push("--version");
    options_whitelist.push("--verbose");
    options_whitelist.push("--Wall");
    let rustc: CompilerConfig::new(common, options_whitelist);
    


    let config: Config::new(common, gcc, rustc);

    Ok(config)
}

