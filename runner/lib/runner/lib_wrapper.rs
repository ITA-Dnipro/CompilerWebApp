use sharedlib:: {Lib, Func};
use std::path::{PathBuf};
use crate::Error;
use slog::{Logger, error};


/// ## Wraps sharelib's Lib struct
/// 
/// * Members
/// 
/// `logger` logger from slog crate
/// 
/// `lib`   lib created by path arg
/// 
/// `name`  entry point name (e.g. "main")
pub(crate) struct LibWrapper<'a>
{
    logger: &'a Logger, 
    lib: Lib,
    name: String
}


impl<'a> LibWrapper<'a>
{
    /// # LibWrapper's constructor
    /// 
    /// # Args
    /// 
    /// `logger` logger from slog crate
    /// 
    /// `path` path to shared object where target lib is to be found
    /// 
    /// `name` entry point name (e.g. "main")
    ///  ## Example
    /// ```rust
    /// let drain = slog::Discard;
    /// let logger = slog::Logger::root(drain, o!());
    /// let lib = Lib::new(
    ///     logger, 
    ///     PathBuf::from("libmy_function.so"), 
    ///     String::from("main")
    /// )?;
    /// let shared_func = lib.shared_func()?.get();
    /// shared_func();
    /// ```
    pub(crate) fn new(logger: &'a Logger, path: PathBuf, name: String) -> Result<Self, Error>
    {
        let path = path.canonicalize()?;

        unsafe {
            let lib = match Lib::new(path.clone()) {
                Ok(_lib) => _lib,
                Err(error) => 
                {
                    error!(logger, "{:?}: Failed to open", path);
                    return Err(
                        Error::NoLibError(error.to_string())
                    )
                }
            };

            Ok(Self { logger, lib, name})
        }
    }
    /// Return a function symbol withing the shared library
    pub(crate)   fn shared_func(&self) -> Result<Func<extern "C" fn() -> i32>, Error>
    {
        unsafe {
            let shared_func_wrapper: Func<extern "C" fn() -> ::std::os::raw::c_int> =
                match self.lib.find_func(self.name.clone()) {
                    Ok(_shared_func) => _shared_func,
                    Err(error) => 
                    {
                        error!(
                            self.logger, "{}: {}", 
                            self.name.clone(), 
                            error
                        );

                        return Err(
                            Error::EntryPointError(error.to_string())
                        )
                    }   
                };

                Ok(shared_func_wrapper)
        }
    }
}