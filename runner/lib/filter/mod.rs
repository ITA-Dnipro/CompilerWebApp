use std::fs::{File};
use std::path::Path;
use std::env;
use seccompiler::{compile_from_json, BpfProgram, TargetArch};

const FILTERS_CONFIG_PATH: &str = "config/filters.json";

pub fn build_filter() -> Result<BpfProgram, &'static str> {
    // TODO: read json path from env variable, change default one
    let config_path = match env::var("FILTERS_CONFIG_PATH") {
        Ok(env_path) => { 
            env_path 
        },
        Err(_) => {
            String::from(FILTERS_CONFIG_PATH)
        }
    };
    assert!(Path::new(config_path.as_str()).exists());
    let mut filters = compile_from_json(
        File::open(config_path.as_str()).unwrap(), 
        TargetArch::x86_64
    ).expect("Cannot compile filters from json");
    // TODO: use preset string as func param
    let bpf_prg: BpfProgram = filters.remove("main_thread").unwrap();
    
    return Ok(bpf_prg);
}
