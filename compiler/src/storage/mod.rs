pub mod name_generator;

trait Storage {
    fn save_source_code();
    fn read_output();
}

// TODO Write stdout and stderr to temp files
// TODO Chech file existance with the same name
// TODO Clean bin directory
