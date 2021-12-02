pub mod name_generator;

trait Storage {
    fn save_source_code();
    fn read_output();
}
