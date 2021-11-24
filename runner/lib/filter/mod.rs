use std::fs::{File};
use std::path::Path;
use std::env;
use std::string::String;
use seccompiler::{compile_from_json, BpfProgram, TargetArch};

const FILTERS_CONFIG_PATH: &str = "config/filters.json";

pub fn build_filter() -> Result<BpfProgram, String> {
    // TODO: change default json_path
    let preset_name= String::from("default");
    let config_path = match env::var("CWA_FILTERS_CONFIG_PATH") {
        Ok(env_path) => { 
            env_path 
        },
        Err(_) => {
            String::from(FILTERS_CONFIG_PATH)
        }
    };
    if !Path::new(config_path.as_str()).exists() {
        return Err(format!("{}: no such file.", config_path))
    }
    let mut _filters = compile_from_json(
        File::open(config_path.as_str()).unwrap(), 
        TargetArch::x86_64
    );
    let mut filters = match _filters {
        Ok(_filt) => _filt,
        Err(error) => {
            return Err(error.to_string())
        }
    };
    
    // TODO: use preset string as func param
    if let Some(bpf_prg) = filters.remove(preset_name.as_str()) {
        Ok(bpf_prg)
    } else {
        Err(
            format!(
                "{}: no such preset in {}", 
                preset_name, 
                config_path
            )
        )
    }
}


#[test]
fn error_message() {
    let expected_path = "some non-existing file";
    env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
    if let Err(error) =  build_filter() {
        assert_eq!(format!("{}: no such file.", expected_path), error);
    };
}

#[test]
fn broken_json() {
    let expected_path = "test/data/test.json";
    env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
    if let Err(error) = build_filter() {
        assert!(
            error.contains("Json Frontend error:")
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
            error
        );       
    };
}