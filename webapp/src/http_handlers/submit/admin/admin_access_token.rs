use std::env;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use configurable::Configurable;
use super::consts::*;

pub struct AdminAccessToken;

impl<'a> Configurable<'a> for AdminAccessToken
{
    fn get_settings(&self) -> Result<HashMap<String, String>, Error>
    {
        let token;
        match env::var(ENV_ADMIN_TOKEN)
        {
            Ok(_token) => 
            {
                token = _token 
            },
            Err(what) => 
            {   
                let err = Error::new(
                    ErrorKind::NotFound,
                    what.to_string()
                );
                return Err(err)
            }
        }
        let map = HashMap::from([(String::from(ACCESS_TOKEN_KEY), token)]);

        Ok(map)
    }

    fn set_settings(&self, map: HashMap<String, String>) -> Result<(), Error>
    {
        let token = &map[ACCESS_TOKEN_KEY];
        env::set_var(ENV_ADMIN_TOKEN, token);

        Ok(())
    }
}