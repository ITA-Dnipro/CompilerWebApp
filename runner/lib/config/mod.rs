use std::env;
use serde::Deserialize;
use serde_yaml;
use crate::Error;
use std::path::PathBuf;
use std::fs::File;
use slog::{Logger, error, info, o, Drain, debug};


#[derive(Deserialize, Debug)]
pub(crate) struct Config{
    pub(crate) execution_limit: Option<u128>,
    pub(crate) entry_point: String
}

impl Config 
{
    pub(crate) fn new(logger: &Logger) -> Result<Config, Error>
    {
        let reader = match get_config_reader(logger)
        {
            Some(_reader) => _reader,
            None =>
            {
                return Err(
                    Error::ConfigError("Failed to open config file".to_string())
                )
            }
        };

        match serde_yaml::from_reader(reader)
        {
            Ok(map) => 
            {
                Ok(map)
            },
            Err(error) => 
            {
                error!(logger, "Config file is broken. {}", error);

                Err(Error::ConfigError(error.to_string()))
            }
        }
    }
}

fn get_config_reader(logger: &Logger) -> Option<File> 
{
    const ENV_VAR: &str = "CWA_RUNNER_CONFIG_PATH";
    const CONFIG_FILENAME: &str = "config.yaml";
    const ETC_DEFAULT: &str = "/etc/CompilerWebApp";
    let etc_file_path = PathBuf::from(ETC_DEFAULT).join(CONFIG_FILENAME);

    if let Ok(config_path) = env::var(ENV_VAR) 
    {
        if let Ok(reader) = File::open(config_path.clone())
        {
            debug!(logger, "Open config file: {}", config_path);
            return Some(reader)
        }
    }
    if let Ok(cwd) = env::current_dir()
    {
        let cwd_file_path = PathBuf::from(cwd).join(CONFIG_FILENAME);
        if let Ok(reader) = File::open(cwd_file_path.clone())
        {
            debug!(logger, "Open config file: {:?}", cwd_file_path);
            return Some(reader)
        }
    };
    if let Ok(reader) = File::open(etc_file_path.clone())
    {
        debug!(logger, "Open config file: {:?}", etc_file_path);
        return Some(reader)
    }
    error!(logger, "Failed to open config file!");
    
    None
}

#[test]
fn parse_some_config()
{
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());
    const ENV_VAR: &str = "CWA_RUNNER_CONFIG_PATH";
    env::set_var(ENV_VAR, "test/data/test_config.yaml");
    let mut config =  Config::new(&log).unwrap();
    println!("{:?}", config);
    //println!("{:?}", config.entry("execution_limit".to_string()));
    //println!("{:?}", config.get(&"execution_limit".to_string()));
}