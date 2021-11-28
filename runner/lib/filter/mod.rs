use std::fs::{File};
use std::env;
use std::string::String;
use std::path::PathBuf;
use crate::Error;
use seccompiler::{compile_from_json, BpfProgram, TargetArch};
use slog::{Logger, info, trace};

const FILTERS_CONFIG_PATH: &str = "RunnerFilters.json";
/// # Reads filters from specified config file
/// config file is to be found in following priority:
/// 1. Read from CWA_FILTERS_CONFIG_PATH env variable
/// 2. Read at RunnerFilters.json
pub(crate) fn build_filter(logger: &Logger) -> Result<BpfProgram, Error> {
    trace!(logger, "Current dir: {:?}", 
        env::current_dir().unwrap_or("<Couldnt get cwd!!!>".into())
    );

    let preset_name = String::from("default");
    let config_path = match env::var("CWA_FILTERS_CONFIG_PATH") {
        Ok(env_path) => { 
            env_path 
        },
        Err(_) => {
            String::from(FILTERS_CONFIG_PATH)
        }
    };
    let path = PathBuf::from(config_path.clone());

    info!(logger, "Try to open {:?}", path.canonicalize()?);
    
    let mut filters = compile_from_json(
        File::open(path)?, 
        TargetArch::x86_64
    )?;
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

mod test 
{
    use std::env;
    use super::build_filter;
    use slog::{Logger, Drain, o};
    use slog_async;
    use slog_term;

    #[test]
    fn error_message() {
        let expected_path = "some non-existing file";
        env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
        let logger = get_logger();
        if let Err(error) =  build_filter(&logger) {
            assert_eq!(format!("No such file or directory (os error 2)"), error.to_string());
        };
    }

    #[test]
    fn broken_json() {
        let expected_path = "test/data/test.json";
        env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
        let logger = get_logger();
        if let Err(error) = build_filter(&logger) {
            assert!(
                error.to_string().contains("Json Frontend error:")
            );       
        };
    }

    #[test]
    fn no_such_preset() {
        let expected_path = "test/data/no_such_preset.json";
        env::set_var("CWA_FILTERS_CONFIG_PATH", expected_path);
        let logger = get_logger();
        if let Err(error) = build_filter(&logger) {
            assert_eq!(
                format!("{}: no such preset in {}", "default", expected_path),
                error.to_string()
            );       
        };
    }

    fn get_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        slog::Logger::root(drain, o!())
    }
}