use super::super::filter::build_filter;
use super::Runner;
use crate::data::output::OutputData;
use fork::{fork, Fork};
use seccompiler::{apply_filter, BpfProgramRef};
use sharedlib::{Func, Lib, Symbol};
use shh;
use slog::Logger;
use std::io::Read;
use std::path::Path;
use std::{process, str, thread, time::Instant};

pub(crate) struct CppRunner<'time> {
    shared_object_path: String,
    logger: &'time Logger,
    execution_limit: u128,
}

impl<'time> CppRunner<'time> {
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

impl<'time> From<(&str, &'time Logger, u128)> for CppRunner<'time> {
    fn from(
        (path, logger, execution_limit): (&str, &'time Logger, u128)
    ) -> CppRunner<'time> {
        CppRunner {
            shared_object_path: String::from(path),
            logger,
            execution_limit,
        }
    }
}

impl<'time> From<(String, &'static Logger, u128)> for CppRunner<'time> {
    fn from(
        (path, logger, execution_limit): (String, &'static Logger, u128)
    ) -> CppRunner {
        CppRunner{shared_object_path: path, logger, execution_limit}
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
                wrap_func_with_seccomp(self.shared_object_path.as_str(), &bpf_prg);
                process::exit(0);
            }
            Err(_i) => {
                return Err("Failed to launch user code");
            }
        }
    }
}

fn wrap_func_with_seccomp(path_to_lib: &str, bpf_prg: BpfProgramRef) {
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
