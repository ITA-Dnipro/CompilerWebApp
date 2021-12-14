extern crate nix;
use super::super::filter::build_filter;
use super::Runner;
use super::super::config::config_struct::Config;
use crate::data::output::OutputData;
use crate::data::error::Error;
use super::lib_wrapper::LibWrapper as Lib;
use seccompiler::{apply_filter};
use shh;
use slog::{Logger, trace, debug};
use sharedlib:: {Symbol};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::path::{PathBuf};
use std::{process, str, thread, time::Instant};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, Pid};

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
    fn run(&self) -> Result<OutputData, Error> 
    {
        let bpf_prg = build_filter(self.logger)?;

        trace!(self.logger, "Shared object path: {:?}", self.shared_object_path);

        let lib = Lib::new(
            self.logger, 
            self.shared_object_path.clone(), 
            self.config.entry_point.clone()
        )?;
        let shared_func = lib.shared_func()?;
        let mut shh_output = shh::stdout()?;

        match unsafe {fork() } 
        {
            Ok(ForkResult::Parent{child}) => 
            {
                let (exit_code , err_msg)= self.join_child(child)?;
                let mut buf: Vec<u8> = Vec::new();
                shh_output.read_to_end(&mut buf)?;
                let stdout = str::from_utf8(&buf)?;
                drop(shh_output);
                let output_data = OutputData::new(stdout, &err_msg, exit_code);
                debug!(self.logger, "output_data: {:?}", output_data);
 
                return Ok(output_data);
            }
            Ok(ForkResult::Child) => 
            {
                let exit_code = match apply_filter(&bpf_prg) {
                    Ok(_) => 
                    {
                        unsafe { 
                            shared_func.get()()
                        }
                    },
                    Err(_) => 
                    {
                        process::exit(0)
                    },
                };
                process::exit(exit_code);
            }
            Err(_i) => 
            {
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
    fn join_child(&self, child: Pid) -> Result<(i32, String), Error>
    {
        let _prev = Instant::now();
        let error = Arc::new(Mutex::new(String::new()));
        let error_clone = Arc::clone(&error);
        let handle = thread::spawn(move || {
            let status = waitpid( child, None);
            
            let status = status.unwrap();
            match status 
            {
                WaitStatus::Exited(_, exit_code) => exit_code,
                WaitStatus::Stopped(_, sig) => 
                {
                    error_clone.lock().unwrap()
                        .push_str(format!("Terminated with {sig}").as_str());

                    0
                },
                WaitStatus::PtraceSyscall(_) => 
                {
                    error_clone.lock().unwrap()
                        .push_str(format!("Terminated w").as_str());
                    0
                },
                WaitStatus::PtraceEvent(_, sig, ex_c) => 
                {
                    error_clone.lock().unwrap()
                        .push_str(format!("Terminated with {sig}").as_str());

                    ex_c
                },
                WaitStatus::Signaled(_, sig, _) => 
                {
                    error_clone.lock().unwrap()
                        .push_str(format!("Terminated with {sig}").as_str());

                    0
                },
                _ =>
                {
                    error_clone.lock().unwrap()
                        .push_str(format!("Terminated with").as_str());

                    0
                } 
            }
        });
        if let Some(time_limit) = self.config.execution_limit
        {
            while handle.is_running() {
                if _prev.elapsed().as_millis() > time_limit {
                    unsafe {
                        libc::kill(child.as_raw(), 9);
                    }
                    error.lock().unwrap()
                        .push_str("Process reached execution time limit. ");
                    break;
                }
            }
        }
        let exit_code = handle.join()?;
        let error =  format!("{}", *error.lock().unwrap());

        Ok((exit_code, error))
    } 
}
