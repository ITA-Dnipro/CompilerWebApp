// TODO: predict memory manipulating
#[cfg(test)]
mod tests {
    use runner::run_code;
    use std::{fs::{remove_file, File}, io::Read};
    use compiler::data::input_data::compiler_type::{CompilerType};
    use std::{str, path::{Path, PathBuf}, thread};
    use slog::{*};

    const TEST_DIR: &str = "test/data";
    const CPP: CompilerType = CompilerType::Cpp;
    #[test]
    fn casual_cpp() {
        let root = get_logger();
        let path = PathBuf::from("test/lib/libcasual_cpp");
        run_code(CPP, path, &root).unwrap();
        assert!(true);
    }


    #[test]
    fn file_is_not_created() {
        let root = get_logger();
        const FILE_NAME: &str = "test/data/new_file_created_with_so";
        let file_path = Path::new(FILE_NAME);
        
        if file_path.exists() {
            remove_file(FILE_NAME).unwrap();
        }
        let path = PathBuf::from("test/lib/libnew_file");
        run_code(CPP,path, &root).unwrap();
        assert!(file_path.exists());
    }

    #[test]
    fn file_is_not_removed() {
        let root = get_logger();
        const FILE_NAME: &str = "testfile";
        let file_path = Path::new(TEST_DIR).join(FILE_NAME);
        if !file_path.exists() {
            File::create(file_path)
                .expect("Could not create testfile.");
        };
        let path = PathBuf::from("test/lib/libremove_file.so");
        run_code(CPP, path, &root).unwrap();
        let file_path = Path::new(TEST_DIR).join(FILE_NAME);
        assert!(file_path.exists());
    }

    #[test]
    fn prints_text() {
        let root = get_logger();
        let path = PathBuf::from("test/lib/libsimple_print.so");
        let output= run_code(CPP, path, &root);
        match output {
            Err(_) => assert!(false),
            Ok(output_data) => {
                assert_eq!("", output_data.stderr);
                assert_eq!("HI from main()!\n", output_data.stdout);
            }
        }
    }

    #[test]
    fn fail_on_no_lib() {
        let root = get_logger();
        let path = PathBuf::from("no such lib");
        let output = run_code(CPP, path, &root);
        match output {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn loop_cpp() {
        let root = get_logger();
        let path = PathBuf::from("test/lib/libloop.so");
        let handle  = thread::spawn(move || {       
            run_code(CPP, path, &root).unwrap();
            //assert!(false);
            return;
            //std::process::exit(0);
        });
        match handle.join() {
            Err(err) => println!("{:?}", err),
            Ok(_) => {}
        }
        //handle.join().unwrap();
        //thread::sleep(Duration::new(2, 0));
        //panic!("shared function runs too long");
    }

    #[test]
    fn runtime_error() {
        let root = get_logger();
        let path = PathBuf::from("test/lib/libruntime_error.so");
        let output = run_code(CPP, path, &root).unwrap();
        assert_eq!(output.stderr, "");
    }
    
    #[test]
    fn change_file_content() {
        const CONTENT_FILE: &str = "test/data/file_with_content.txt";
        let root = get_logger();
        let mut file = File::open(CONTENT_FILE).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        drop(file);
        let path = PathBuf::from("test/lib/librewrite_file_content.so");
        run_code(CPP, path, &root).unwrap();
        let mut file = File::open(CONTENT_FILE).unwrap();
        let mut buf_after = Vec::new();
        file.read_to_end(&mut buf_after).unwrap();
        assert_eq!(str::from_utf8(&buf), str::from_utf8(&buf_after));
    }


    fn get_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        slog::Logger::root(drain, o!())
    }
}