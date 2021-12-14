use configurable::Configurable;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use super::{config_struct::Config, config_io::ConfigIO};
use slog::Logger;
use serde_yaml;

/// ## Struct to implement Configurable trait
/// This is used to provide setting that can be changed from admin panel.
/// ### Fields
/// * `logger` - logger to log in
pub struct RunnerConfig;

impl Configurable<'_> for RunnerConfig
{

    fn get_settings(&self) -> Result<HashMap<String, String>, Error>
    {
        let reader;
        match ConfigIO::get_config_reader()
        {
            Some(_r) => reader = _r,
            None => 
            {
                return Err(Error::new(ErrorKind::NotFound, "Config file is missing"))
            }
        }
        let map: HashMap<String, String> = 
            match serde_yaml::from_reader(reader)
            {
                Ok(_map) => _map,
                Err(what) => 
                {
                    let err = Error::new(
                        ErrorKind::InvalidData, 
                        what.to_string()
                    );
                    return Err(err)
                }
            };

        Ok(map)
    }

    fn set_settings(&self, map: HashMap<String, String>) -> Result<(), Error>
    {
        let writer;
        match ConfigIO::get_config_writer()
        {
            Some(_w) => writer = _w,
            None =>
            {
                return Err(Error::new(ErrorKind::NotFound, "Config file is missing"))
            }
        }
        let config_object = Config::from_hash_map(map)?;
        match serde_yaml::to_writer(writer, &config_object)
        {
            Ok(_) => Ok(()),
            Err(what) =>
            {
                let err = Error::new(
                    ErrorKind::InvalidData, 
                    what.to_string()
                );

                Err(err)
            }
        }
    } 
}