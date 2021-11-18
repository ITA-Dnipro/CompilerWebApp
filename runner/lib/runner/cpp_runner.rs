use super::Runner;
use slog::Logger;
use crate::data::output::OutputData;
use std::{process, str};
use seccompiler::{apply_filter, BpfProgramRef};
use std::path::Path;
use fork::{fork, Fork};
use sharedlib::{Func, Lib, Symbol};
use shh;
use std::io::Read;
use super::super::filter::build_filter;

pub(crate) struct CppRunner<'time> {
    shared_object_path: String,
    logger: &'time Logger
}

impl<'time> CppRunner<'time> {
    pub fn new<T>(path: T, logger: &'time Logger) -> Self
        where (T, &'time Logger): Into<CppRunner<'time>> {
            (path, logger).into()
        }
}

impl<'time> From<(&str, &'time Logger)> for CppRunner<'time> {
    fn from((path, logger): (&str, &'time Logger)) -> CppRunner<'time> {
        CppRunner{
            shared_object_path: String::from(path),
            logger
        }
    }
}

impl<'time> From<(String, &'static Logger)> for CppRunner<'time> {
    fn from((path, logger): (String, &'static Logger)) -> CppRunner {
        CppRunner{
            shared_object_path: path,
            logger
        }
    }
}

impl<'time> Runner for CppRunner<'time> {
    fn run(&self) -> Result<OutputData, &'static str> {
        let bpf_prg = build_filter().unwrap();
        let mut shh_stdout = shh::stdout().unwrap();
        let mut shh_stderr = shh::stderr().unwrap();
    
        let forked = fork();
        match forked {
            Ok(Fork::Parent(child)) => {
                let mut child_status: i32 = -1;
                let _pid_done = unsafe { libc::waitpid(child, &mut child_status, 0) };
    
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
                wrap_func_witch_seccomp(
                    self.shared_object_path.as_str(), 
                    &bpf_prg
                );
                process::exit(0);
            }
            Err(_i) => {
                return Err("Failed to launch user code");
            }
        }
    }
        
}

fn wrap_func_witch_seccomp(path_to_lib: &str, bpf_prg: BpfProgramRef) {
    let path_to_lib = Path::new(path_to_lib).canonicalize().unwrap();
    unsafe {
        let lib = Lib::new(path_to_lib).unwrap();
        let shared_func_wrapper: Func<extern "C" fn() -> ::std::os::raw::c_int> =
            lib.find_func("main").expect("Could not find func.");
        let shared_func = shared_func_wrapper.get();

        match apply_filter(&bpf_prg) {
            Ok(_) => shared_func(),
            // TODO: research an option to return valuable exit code
            Err(_) => process::exit(0),
        };
    }
}