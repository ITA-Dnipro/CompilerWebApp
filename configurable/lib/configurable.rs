use std::collections::HashMap;
use std::io::Error;
/// * Used to provide an opportunity to get/set configuration 
/// variables
/// 
/// Implement this trait and than register it as a `State` in rocket's `manage` method
/// ### Example
/// 
pub trait Configurable<'a> : Sync + Send
{
    fn get_settings(&self) -> Result<HashMap<String, String>, Error>;
    fn set_settings(&self, map: HashMap<String, String>) -> Result<(), Error>; 
}