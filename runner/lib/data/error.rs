/// # Error enum describes error that occurs while running user code
/// It is divided into 2 cases:
/// * asdf
/// 1.
/// 2)
#[derive(Debug)]
pub enum Error 
{
    ForkError(String),
    EntryPointError(String),
    NoLibError(String),
    NotImplementedError(String),
    ConfigError(String)
}

impl Error 
{
    pub fn to_string(self) -> String
    {
        match self 
        {
            Error::ForkError(_str) => _str,
            Error::EntryPointError(_str) => _str,
            Error::NoLibError(_str ) => _str,
            Error::NotImplementedError(_str) => _str,
            Error::ConfigError(_str) => _str
        }   
    }
}