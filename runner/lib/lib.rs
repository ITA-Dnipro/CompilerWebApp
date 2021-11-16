use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path};
use std::{str, process};
use sharedlib::{Func, Lib, Symbol};
use seccompiler::*;
use shh;
use std::process::Command;
use std::process;
use fork::{daemon, Fork};

#[no_mangle]
pub fn run_shared(path_to_lib: &'static str) {
    
    let pid = unsafe { libc::fork() };
    if let Ok(Fork::Child) = daemon(false, false ) { 

    }
    match pid {
        0 => {
            wrap_func_witch_seccomp(path_to_lib);
            process::exit(0);
        }
        child_pid => {
            let mut child_status: i32 = -1;
            let pid_done = unsafe { libc::waitpid(child_pid, &mut child_status, 0) };
            assert_eq!(pid_done, child_pid);
            //assert!(libc::WIFSIGNALED(child_status));
            //assert_eq!(libc::WTERMSIG(child_status), libc::SIGSYS);
        }
    }
}

fn wrap_func_witch_seccomp(path_to_lib: &str) {
    let path_to_lib = Path::new(path_to_lib)
        .canonicalize()
        .unwrap();
    unsafe {
        let seccomp_level = unsafe { libc::prctl(libc::PR_GET_SECCOMP) };
        assert_eq!(seccomp_level, 0);
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
        let seccomp_level = unsafe { libc::prctl(libc::PR_GET_SECCOMP) };
        //assert_eq!(seccomp_level, 2);
        shared_func();
    }
}

