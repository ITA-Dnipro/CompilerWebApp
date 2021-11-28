use std::str::Utf8Error;
use std::any::Any;

use seccompiler;
/// # Error enum describes error that occurs while running user code
/// It is divided into 5 cases:
/// 
/// 1.  ForkError
/// *   Occurs when triggering syscall fork fails
/// 2.  NoLibError
/// *   Occurs when specified shared object path is absent
/// 3. NotImplementedError
/// *   Occurs when caller tries to launch module, that is still
///     in development state
/// 4. ConfigError
/// *   Occurs when config file is absent, or config file is 
///     invalid, or some config variables are absent or invalid
/// 5. EntryPointError
/// *   Occurs when shared object does not contain required
///     entry point function (e.g. main)
#[derive(Debug)]
pub enum Error 
{
    ForkError(String),
    NoLibError(String),
    NotImplementedError(String),
    ConfigError(String),
    EntryPointError(String),
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

impl From<std::io::Error> for Error 
{
    fn from(err: std::io::Error) -> Self 
    {
        Error::ConfigError(err.to_string())
    }
}

impl From<seccompiler::Error> for Error
{
    fn from(err: seccompiler::Error) -> Self
    {
        Error::ConfigError(err.to_string())
    }
}

impl From<Utf8Error> for Error
{
    fn from(err: Utf8Error) -> Self
    {
        Error::ConfigError(err.to_string())
    }
}

impl From<Box<dyn Any + Send>> for Error
{
    fn from(err: Box<dyn Any + Send>) -> Self
    {
        Error::NotImplementedError(format!("{:?}", err))
    }
}