use std::collections::BTreeMap;
use std::io::{Read};
use std::path::{Path};
use std::{str, process};
use sharedlib::{Func, Lib, Symbol};
use seccompiler::{apply_filter, SeccompAction, SeccompFilter, BpfProgram, Error};
use fork::{fork, Fork};
use shh::{Shh};
mod data;
use crate::data::output::OutputData;

#[no_mangle]
pub fn run_user_prog(path_to_lib: &'static str) -> Result<OutputData, &str> {
    let mut shh_stdout = shh::stdout().unwrap();
    let mut shh_stderr = shh::stderr().unwrap();
    let forked  = fork();
    match forked {
        Ok(Fork::Parent(child)) => {
            let mut child_status: i32 = -1;
            let _pid_done = unsafe { libc::waitpid(child, &mut child_status, 0) };

            let mut buf:Vec<u8> = Vec::new();          
            shh_stdout.read_to_end(&mut buf).unwrap();
            let stdout = str::from_utf8(&buf).unwrap();

            let mut buf:Vec<u8> = Vec::new();
            shh_stderr.read_to_end(&mut buf).unwrap();
            let stderr = str::from_utf8(&buf).unwrap();
            (drop(shh_stdout), drop(shh_stderr));

            let output_data = OutputData::new(stdout, stderr);

            return Ok(output_data);
        } ,
        Ok(Fork::Child) => {
            wrap_func_witch_seccomp(path_to_lib);
            process::exit(0);
        },
        Err(i) => {
            return Err("Failed to launch user code");
        }
    } 
}

fn wrap_func_witch_seccomp(path_to_lib: &str) {
    let path_to_lib = Path::new(path_to_lib)
        .canonicalize()
        .unwrap();
    unsafe {
        let lib = Lib::new(path_to_lib)
            .unwrap();
        let shared_func_wrapper: Func<extern "C" fn() -> ::std::os::raw::c_int> = lib.find_func("main")
            .expect("Could not find func.");
        let shared_func = shared_func_wrapper.get();
        
        let rules = BTreeMap::new();
        let seccomp_filter = SeccompFilter::new(
            rules,
            SeccompAction::KillThread,
            SeccompAction::Allow,
            seccompiler::TargetArch::x86_64
        )
            .map_err(Error::Backend)
            .unwrap();
        
        let bpf_prog: BpfProgram = seccomp_filter.try_into().unwrap();
        apply_filter(&bpf_prog).expect("Could not apply filter");
        shared_func();
    }
}

