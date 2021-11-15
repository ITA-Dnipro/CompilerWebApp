

#[cfg(test)]
mod tests {
    use runner::run_shared;
    use std::fs::{remove_file, File};
    use std::{thread};
    use std::path::{Path};

    const TEST_DIR: &str = "test/lib";
    #[test]
    fn casual_cpp() {
        let handle = thread::spawn(
            || run_shared(
                "test/lib/libcasual_cpp.so"
            )
        );

        handle.join().unwrap();
        assert!(true);
    }


    #[test]
    fn file_is_not_created() {
        const FILE_NAME: &str = "test/lib/new_file_created_with_so";
        let file_path = Path::new(FILE_NAME);
        if file_path.exists() {
            remove_file(FILE_NAME)
                .expect("Could not remove file");
        }

        let handle = thread::spawn(
            || run_shared("test/lib/libcreate_new_file.so")
        );
        handle.join().unwrap();
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
        
        let handle = thread::spawn( 
            || run_shared("test/lib/libremove_file.so")
        );
        handle.join().unwrap();
        let file_path = Path::new(TEST_DIR).join(FILE_NAME);
        assert!(file_path.exists());
    }

    #[test]
    #[ignore]
    fn loop_cpp() {
        run_shared("test/lib/libloop.so");
        assert!(true);
    }
}