use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path};
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;
use std::{str, thread, panic, process};
use std::sync::Arc;
use sharedlib::{Func, Lib, Symbol};
use seccompiler::*;
extern crate shh;
#[no_mangle]
pub fn run_shared(path_to_lib: &'static str) {
    let mut buf = Vec::new();
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        println!("Panic hook!");
        println!("{:?}", thread::current().name());
    }));

    let builder = thread::Builder::new();
    let _ = builder.spawn(
        move || wrap_func_witch_seccomp(&path_to_lib, &mut buf)
    );
    
    sleep(Duration::new(2,0));
    //let _ = result.join().expect("vector");
}

fn wrap_func_witch_seccomp(path_to_lib: &str, buf: &mut Vec<u8>) {
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
        let mut shh = shh::stdout().unwrap();
        
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
        assert_eq!(seccomp_level, 2);
        shared_func();
        shh.read_to_end(buf).unwrap();
    }
}

