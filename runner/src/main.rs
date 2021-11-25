use runner::run_code;
use runner::data::error::Error;
use std::fs::{remove_file, File};
use std::path::{Path};
use compiler::data::input_data::compiler_type::{CompilerType};
use {slog, slog::o};

const TEST_DIR: &str = "test/lib";
const CPP: CompilerType = CompilerType::Cpp;
fn main() {
    casual_cpp();
    file_is_not_created();
    file_is_not_removed();
    println!("Exit main");
}

fn casual_cpp() {
    let root = slog::Logger::root(
        slog::Discard, 
        o!("key1" => "value1", "key2" => "value2")
    ); 
    run_code(CPP, "test/lib/libcasual_cpp.so", &root).unwrap();
    assert!(true);
}

fn file_is_not_removed() {
    let root = slog::Logger::root(
        slog::Discard, 
        o!("key1" => "value1", "key2" => "value2")
    ); 
    const FILE_NAME: &str = "testfile";
    let file_path = Path::new(TEST_DIR).join(FILE_NAME);
    if !file_path.exists() {
        File::create(file_path)
            .expect("Could not create testfile.");
    }
    
    run_code(CPP, "test/lib/libremove_file.so", &root)
        .unwrap();
    let file_path = Path::new(TEST_DIR).join(FILE_NAME);
    assert!(file_path.exists());
}

fn file_is_not_created() {
    let root = slog::Logger::root(
        slog::Discard, 
        o!("key1" => "value1", "key2" => "value2")
    ); 
    const FILE_NAME: &str = "test/data/new_file_created_with_so";
    let file_path = Path::new(FILE_NAME);
    if file_path.exists() {
        remove_file(FILE_NAME)
            .expect("Could not remove file");
    }

    run_code(CPP,"test/lib/libcreate_new_file.so", &root)
        .unwrap();
    
    assert!(! file_path.exists());
}