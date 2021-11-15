use runner::run_shared;
use std::fs::{remove_file, File};
use std::{thread};
use std::path::{Path};
const TEST_DIR: &str = "test/lib";
fn main() {
    //casual_cpp();
    file_is_not_created();
    //file_is_not_removed();
}

fn casual_cpp() {
    /*let handle = thread::spawn(
        || run_shared(
            format!("{}/{}", TEST_DIR, "libcasual_cpp.so").as_str()
        )
    );*/

    //handle.join().unwrap();

    run_shared(
        "test/lib/libcasual_cpp.so"
    );
    assert!(true);
}

fn file_is_not_removed() {
    const FILE_NAME: &str = "testfile";
    let file_path = Path::new(TEST_DIR).join(FILE_NAME);
    if !file_path.exists() {
        File::create(file_path)
            .expect("Could not create testfile.");
    }
    
    run_shared("test/lib/libremove_file.so");
    let file_path = Path::new(TEST_DIR).join(FILE_NAME);
    assert!(file_path.exists());
}

fn file_is_not_created() {
    const FILE_NAME: &str = "test/data/new_file_created_with_so";
    let file_path = Path::new(FILE_NAME);
    if file_path.exists() {
        remove_file(FILE_NAME)
            .expect("Could not remove file");
    }

    run_shared("test/lib/libcreate_new_file.so");
    
    assert!(! file_path.exists());
}