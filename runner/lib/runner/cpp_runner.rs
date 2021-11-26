use super::super::filter::build_filter;
use super::Runner;
use super::super::config::Config;
use crate::data::output::OutputData;
use crate::data::error::Error;
use fork::{fork, Fork};
use seccompiler::{apply_filter};
use sharedlib::{Func, Lib, Symbol};
use shh;
use slog::{Logger, trace, error, info};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{process, str, thread, time::Instant};

pub(crate) struct CppRunner<'time> 
{
    shared_object_path: PathBuf,
    logger: &'time Logger,
    config: Config
}

impl<'time> CppRunner<'time> 
{
    pub fn new(path: PathBuf, logger: &'time Logger) -> Result<Self, Error>
    {
        trace!(logger, "Create CppRunner"); 
        let config = Config::new(logger)?;

        Ok(
            CppRunner {shared_object_path: path, logger, config}
        )
    }
}

impl<'time> Runner<'time> for CppRunner<'time> 
{
    fn run(&self) -> Result<OutputData, Error> {
        let bpf_prg = build_filter()?;
        let path_to_lib = self.shared_object_path.canonicalize()?;
        trace!(self.logger, "Shared object path: {:?}", path_to_lib);
        let lib;
        let shared_func: Func<extern "C" fn() -> i32> 
              = unsafe {

                    lib = match Lib::new(path_to_lib.clone()) {
                        Ok(_lib) => _lib,
                        Err(error) => 
                        {
                            error!(self.logger, "{:?}: Failed to open", path_to_lib);
                            return Err(
                                Error::NoLibError(error.to_string())
                            )
                        }
                    };

                    let shared_func_wrapper: Func<extern "C" fn() -> ::std::os::raw::c_int> =
                        match lib.find_func(self.config.entry_point.clone()) {
                            Ok(_shared_func) => _shared_func,
                            Err(error) => 
                            {
                                error!(
                                    self.logger, "{}: {}", 
                                    self.config.entry_point.clone(), 
                                    error
                                );

                                return Err(
                                    Error::EntryPointError(error.to_string())
                                )
                            }   
                        };

                    shared_func_wrapper
                };

        let mut shh_stdout = shh::stdout()?;
        let mut shh_stderr = shh::stderr()?;
        match fork() {
            Ok(Fork::Parent(child)) => {
                self.join_child(child);

                let mut buf: Vec<u8> = Vec::new();
                shh_stdout.read_to_end(&mut buf)?;
                let stdout = str::from_utf8(&buf)?;

                let mut buf: Vec<u8> = Vec::new();
                shh_stderr.read_to_end(&mut buf)?;
                let stderr = str::from_utf8(&buf)?;
                (drop(shh_stdout), drop(shh_stderr));
                let output_data = OutputData::new(stdout, stderr);

                return Ok(output_data);
            }
            Ok(Fork::Child) => {
                match apply_filter(&bpf_prg) {
                    Ok(_) => 
                    {
                        info!(self.logger, "launch shared object code");
                        unsafe { 
                            shared_func.get()() 
                        }
                    },
                    // TODO: research an option to return valuable exit code
                    Err(_) => 
                    {
                        error!(self.logger, "Failed to apply filter");

                        process::exit(0)
                    },
                };
                process::exit(0);
            }
            Err(_i) => 
            {
                error!(self.logger, "Failed to fork!");

                return Err(
                    Error::ForkError(
                        String::from("Failed to launch user code")
                    )
                );
            }
        }
    }
      
}

impl<'time> CppRunner<'time>
{
    fn join_child(&self, child: i32) 
    {
        let _prev = Instant::now();
        let handle = thread::spawn(move || {
            let mut child_status: i32 = -1;
            let _pid_done = unsafe { libc::waitpid(child, &mut child_status, 0) };
        });
        if let Some(time_limit) = self.config.execution_limit
        {
            while handle.is_running() {
                if _prev.elapsed().as_millis() > time_limit {
                    unsafe {
                        libc::kill(child, 9);
                    }
                }
            }
        }
    } 
}

impl<'time> CppRunner<'time>
{
    fn get_shared_lib(&self)
    {

    } 
}
