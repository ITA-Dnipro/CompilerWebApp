/// Generates unique string for filename 
///
///
/// # Result
///
/// * A string that consist of word "file" and date and time values separated by dashes
/// * No extension added to generated file name
/// 
/// 
/// # Example
///
/// ```
/// 
/// use super::super::storage::name_generator::generate_filename;
/// 
/// let bin_file_name = generate_filename().to_owned();  // returns file-2021-11-01-08-35-26-518851300
/// 
/// ```
/// So, ```bin_file_name``` holds ```"file-2021-11-01-08-35-26-518851300"```
/// 

pub fn generate_filename() -> String {
    let now = chrono::Utc::now();
    let filename = String::from(now.format("file-%Y-%m-%d-%H-%M-%S-%f").to_string());
    filename
}