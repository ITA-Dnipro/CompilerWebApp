#![allow(unused)]
use std::fs::{File, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::env;
use std::path::{PathBuf, Path};
use std::error::Error;

pub fn write_source_to_file(source_code: &str, lang: &str) -> Option<PathBuf>
{
    // If error ever happens on work with temp files it's not on the user, 
    // so he should get "internal server error" here
    let mut input_file_name = env::var("COMPILATION_TEMP_DIR")
        .expect("COMPILATION_TEMP_DIR doesn't exist");
    input_file_name.push_str("/");
    input_file_name.push_str(
        &("compilation_input-".to_owned() + &generate_file_signature())
    );
    input_file_name.push_str(".cpp");     // TODO: add lang specific extensions

    let mut code_file: File;
    match File::create(&input_file_name)
    {
        Ok(file) => code_file = file,
        Err(_) =>
        {
            println!("[ERROR]: Couldn't create a file at \"{}\"", input_file_name);
            return None;
        }
    }
        
    match code_file.write_all(source_code.as_bytes())
        .and_then(|()| code_file.seek(SeekFrom::Start(0)))
    {
        Ok(_) => {},
        Err(_) =>
        {
            println!("[ERROR]: Error while working with \"{}\"", input_file_name);
            return None;
        }
    }

    println!("[INFO]: Created \"{}\"", input_file_name);
    Some(PathBuf::from(input_file_name))
}

pub fn delete_file(filename: &Path)
{
    match remove_file(filename)
    {
        Ok(_) => println!("[INFO]: Removed {:?}.", filename),
        Err(e) => println!("[ERROR]: Couldn't remove {:?}", filename)   
    }    
}

fn generate_file_signature() -> String
{
    chrono::Utc::now()
        .format("%Y-%m-%d-%H-%M-%S-%f").to_string()
}

#[cfg(test)]
mod tests
{
    // Fail delete_file on purpose, it should not panic
    #[test]
    fn fail_delete()
    {
        super::delete_file(std::path::Path::new("Doesn't exits"));
    }
}
