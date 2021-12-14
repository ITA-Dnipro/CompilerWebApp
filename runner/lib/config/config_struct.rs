
use std::env;
use std::io;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_yaml;
use crate::Error;
use filepath::FilePath;
use slog::{Logger, error, debug};
use super::config_io::ConfigIO;


/// ## Base runner config struct
/// 
/// ### Struct fields
/// * `execution_limit` - time limit of shared object execution, in ms
/// * `entry_point` - function in shared object to invoke, usually "main"
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config
{
    pub(crate) execution_limit: Option<u128>,
    pub(crate) entry_point: String
}

impl Config 
{
    pub(crate) fn new(logger: &Logger) -> Result<Config, Error>
    {
        let reader = match ConfigIO::get_config_reader()
        {
            Some(_reader) => 
            {
                debug!(logger, "Open config file: {:?}", _reader.path()?);

                _reader
            },
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

    pub(crate) fn from_hash_map(map: HashMap<String, String>) -> Result<Self, io::Error>
    {
        let entry_point = map["entry_point"].clone();
        let execution_limit = match map["execution_limit"].parse::<u128>()
        {
            Ok(limit) => Some(limit),
            Err(_) => None
        };

        Ok(
            Self
            {
                entry_point,
                execution_limit
            }
        )
    }
}


#[test]
fn parse_some_config()
{
    use slog::{o, Drain};

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());
    const ENV_VAR: &str = "CWA_RUNNER_CONFIG_PATH";
    env::set_var(ENV_VAR, "test/data/test_config.yaml");
    
    let config =  Config::new(&log).unwrap();
    println!("{:?}", config);
}