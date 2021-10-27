pub fn generate_filename() -> String {
    let now = chrono::Utc::now();
    let filename = String::from(now.format("file-%Y-%m-%d-%H-%M-%S-%f").to_string());
    filename
}