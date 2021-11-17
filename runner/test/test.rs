// TODO: predict memory manipulating

#[cfg(test)]
mod tests {
    use runner::run_user_prog;
    use std::fs::{remove_file, File};
    use std::{thread};
    use std::path::{Path};

    const TEST_DIR: &str = "test/data";
    #[test]
    fn casual_cpp() {
        run_user_prog("test/lib/libcasual_cpp.so");
        assert!(true);
    }


    #[test]
    fn file_is_not_created() {
        const FILE_NAME: &str = "test/data/new_file_created_with_so";
        let file_path = Path::new(FILE_NAME);
        if file_path.exists() {
            remove_file(FILE_NAME)
                .expect("Could not remove file");
        }
        run_user_prog("test/lib/libcreate_new_file.so");
        assert!(! file_path.exists());
    }

    #[test]
    fn file_is_not_removed() {
        const FILE_NAME: &str = "testfile";
        let file_path = Path::new(TEST_DIR).join(FILE_NAME);
        if !file_path.exists() {
            File::create(file_path)
                .expect("Could not create testfile.");
        }
        
        run_user_prog("test/lib/libremove_file.so");
        let file_path = Path::new(TEST_DIR).join(FILE_NAME);
        assert!(file_path.exists());
    }

    #[test]
    fn prints_text() {
        let output = run_user_prog("test/lib/libsimple_print.so");
        match output {
            Err(_) => assert!(false),
            Ok(output_data) => {
                assert_eq!("HI from main()!\n", output_data.stdout);
            }
        }
    }

    #[test]
    fn fail_on_no_lib() {
        let output = run_user_prog("no such lib");
        match output {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[test]
    #[ignore]
    fn loop_cpp() {
        run_user_prog("test/lib/libloop.so");
        assert!(true);
    }
}