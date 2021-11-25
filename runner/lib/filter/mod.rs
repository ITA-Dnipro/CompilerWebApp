use std::fs::{File};
use std::env;
use std::string::String;
use crate::Error;
use seccompiler::{compile_from_json, BpfProgram, TargetArch};

const FILTERS_CONFIG_PATH: &str = "config/filters.json";
/// # Reads filters from specified config file
/// config file is to be found in following priority:
/// 1. Read from CWA_FILTERS_CONFIG_PATH env variable
/// 2. Read at config/filters.json
pub(crate) fn build_filter() -> Result<BpfProgram, Error> {
    // TODO: change default json_path
    let preset_name = String::from("default");
    let config_path = match env::var("CWA_FILTERS_CONFIG_PATH") {
        Ok(env_path) => { 
            env_path 
        },
        Err(_) => {
            String::from(FILTERS_CONFIG_PATH)
        }
    };
    let mut filters;
    match File::open(config_path.as_str()) 
    {
        Ok(reader) => 
        {
            match compile_from_json(reader, TargetArch::x86_64)
            {
                Ok(_filters) =>
                {
                    filters = _filters
                }
                Err(error) => 
                {
                    return Err(Error::ConfigError(error.to_string()))
                }
            }
        },
        Err(_error) => {
            return Err(
                Error::ConfigError(format!("{}: failed to open file.", config_path))
            )
        }
    }

    // TODO: use preset string as func param
    if let Some(bpf_prg) = filters.remove(preset_name.as_str()) 
    {
        Ok(bpf_prg)
    } 
    else 
    {
        Err(
            Error::ConfigError(
                format!("{}: no such preset in {}", preset_name, config_path)
            )
        )
    }
}


#[test]
fn error_message() {
    let expected_path = "some non-existing file";
    env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
    if let Err(error) =  build_filter() {
        assert_eq!(format!("{}: failed to open file.", expected_path), error.to_string());
    };
}

#[test]
fn broken_json() {
    let expected_path = "test/data/test.json";
    env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
    if let Err(error) = build_filter() {
        assert!(
            error.to_string().contains("Json Frontend error:")
        );       
    };
}

#[test]
fn no_such_preset() {
    let expected_path = "test/data/no_such_preset.json";
    env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
    if let Err(error) = build_filter() {
        assert_eq!(
            format!("{}: no such preset in {}", "default", expected_path),
            error.to_string()
        );       
    };
}