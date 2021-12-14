use std::collections::HashMap;
use std::io::Error;
pub trait Configurable<'a> : Sync + Send
{
    fn get_settings(&self) -> Result<HashMap<String, String>, Error>;
    fn set_settings(&self, map: HashMap<String, String>) -> Result<(), Error>; 
}