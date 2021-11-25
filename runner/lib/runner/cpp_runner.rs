use super::super::filter::build_filter;
use super::Runner;
use crate::data::output::OutputData;
use crate::data::error::Error;
use fork::{fork, Fork};
use seccompiler::{apply_filter};
use sharedlib::{Func, Lib, Symbol};
use shh;
use slog::Logger;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{process, str, thread, time::Instant};

pub(crate) struct CppRunner<'time> 
{
    shared_object_path: PathBuf,
    logger: &'time Logger,
    execution_limit: u128,
}

impl<'time> CppRunner<'time> 
{
    pub fn new<T>(
        path: T, 
        logger: &'time Logger, 
        execution_limit: u128
    ) -> Self
    where
        (T, &'time Logger, u128): Into<CppRunner<'time>>,
    {
        (path, logger, execution_limit).into()
    }
}

impl<'time> From<(&str, &'time Logger, u128)> for CppRunner<'time> 
{
    fn from(
        (path, logger, execution_limit): (&str, &'time Logger, u128)
    ) -> CppRunner<'time> {
        let path = PathBuf::from(path);
        CppRunner {
            shared_object_path: path,
            logger,
            execution_limit,
        }
    }
}

impl<'time> From<(String, &'time Logger, u128)> for CppRunner<'time> 
{
    fn from(
        (path, logger, execution_limit): (String, &'time Logger, u128)
    ) -> CppRunner {
        let path = PathBuf::from(path);
        CppRunner{shared_object_path: path, logger, execution_limit}
    }
}

impl<'time> From<(PathBuf, &'time Logger, u128)> for CppRunner<'time> 
{
    fn from(
        (path, logger, execution_limit): (PathBuf, &'time Logger, u128)
    ) -> CppRunner {
        CppRunner{shared_object_path: path, logger, execution_limit}
    }
}

impl<'time> Runner<'time> for CppRunner<'time> 
{
    fn run(&self) -> Result<OutputData, Error> {
        let bpf_prg = build_filter()?;
        let path_to_lib  
            = match self.shared_object_path
                .canonicalize() 
                    {
                        Err(error) => {
                            return Err(
                                Error::NoLibError(error.to_string())
                            )
                        },
                        Ok(_path) => _path
                    };
        let lib;
        let shared_func: Func<extern "C" fn() -> i32> 
              = unsafe {

                    lib = match Lib::new(path_to_lib) {
                        Ok(_lib) => _lib,
                        Err(error) => return Err(
                            Error::NoLibError(error.to_string())
                        )
                    };

                    let shared_func_wrapper: Func<extern "C" fn() -> ::std::os::raw::c_int> =
                        match lib.find_func("main") {
                            Ok(_shared_func) => _shared_func,
                            Err(error) => return Err(
                                Error::EntryPointError(error.to_string())
                            )
                        };

                    shared_func_wrapper
                };

        let mut shh_stdout = shh::stdout().unwrap();
        let mut shh_stderr = shh::stderr().unwrap();
        match fork() {
            Ok(Fork::Parent(child)) => {
                let _prev = Instant::now();
                let handle = thread::spawn(move || {
                    let mut child_status: i32 = -1;
                    let _pid_done = unsafe { libc::waitpid(child, &mut child_status, 0) };
                });
                while handle.is_running() {
                    if _prev.elapsed().as_millis() > self.execution_limit {
                        unsafe {
                            libc::kill(child, 9);
                        }
                    }
                }

                let mut buf: Vec<u8> = Vec::new();
                shh_stdout.read_to_end(&mut buf).unwrap();
                let stdout = str::from_utf8(&buf).unwrap();

                let mut buf: Vec<u8> = Vec::new();
                shh_stderr.read_to_end(&mut buf).unwrap();
                let stderr = str::from_utf8(&buf).unwrap();
                (drop(shh_stdout), drop(shh_stderr));
                let output_data = OutputData::new(stdout, stderr);

                return Ok(output_data);
            }
            Ok(Fork::Child) => {
                match apply_filter(&bpf_prg) {
                    Ok(_) => unsafe { 
                        shared_func.get()() 
                    },
                    // TODO: research an option to return valuable exit code
                    Err(_) => process::exit(0),
                };
                process::exit(0);
            }
            Err(_i) => {
                return Err(
                    Error::ForkError(
                        String::from("Failed to launch user code")
                    )
                );
            }
        }
    }
}
