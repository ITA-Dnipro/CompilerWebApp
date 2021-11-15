use std::io::Read;
use std::path::{Path};
use std::{thread, str};
use sharedlib::{Func, Lib, Symbol};
extern crate prctl;
extern crate shh;
#[no_mangle]
pub fn run_shared(path_to_lib: &'static str) {
    //let builder = thread::Builder::new();
    let handle = thread::spawn(
        move || wrap_func_witch_seccomp(path_to_lib)
    );
    let result = handle.join().unwrap();
    let s = match str::from_utf8(&result) {
        Ok(v) => v,
        Err(_) => ""
    };
    println!("Wrapped message: {}", s);
    
}

fn wrap_func_witch_seccomp(
    path_to_lib: &str
) -> Vec<u8> {
    println!("{}", "new run_shared");
    let path_to_lib = Path::new(path_to_lib)
        .canonicalize()
        .unwrap();
    let mut buf = Vec::new();
    unsafe {
        let lib = Lib::new(path_to_lib)
            .unwrap();
        let hello_world_symbol: Func<extern "C" fn() -> ::std::os::raw::c_int> = lib.find_func("main")
            .expect("Could not find func.");
        let _func = hello_world_symbol.get();
        let mut shh = shh::stdout().unwrap();
        /*match prctl::set_seccomp_strict() {
            Ok(()) => println!("{}", "seccomp activate"),
            Err(ret) => println!("seccomp failed {}", ret)
        }*/
        _func();
        shh.read_to_end(&mut buf).unwrap();
    }

    println!("exit wrapfn");

    buf
}