#![allow(unused)]
use std::fs::{File, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::env;
use std::path::{PathBuf, Path};
use std::error::Error;

pub fn write_source_to_file(source_code: &str, lang_extension: &str) -> Option<PathBuf>
{
    // If error ever happens on work with temp files it's not on the user, 
    // so he should get "internal server error" here
    let mut input_file_name;
    match env::var("COMPILATION_TEMP_DIR")
    {
        Ok(temp_dir) => input_file_name = temp_dir,
        Err(_) => return None
    }
      
    input_file_name.push_str("/");
    input_file_name.push_str(
        &("compilation_input-".to_owned() + &generate_file_signature())
    );
    input_file_name.push_str(lang_extension);
    let mut code_file: File;
    match File::create(&input_file_name)
    {
        Ok(file) => code_file = file,
        Err(_) =>
        {
            return None;
        }
    }
        
    match code_file.write_all(source_code.as_bytes())
        .and_then(|()| code_file.seek(SeekFrom::Start(0)))
    {
        Ok(_) => {},
        Err(_) =>
        {
            drop(code_file);
            delete_file(Path::new(&input_file_name));
            return None;
        }
    }

    Some(PathBuf::from(input_file_name))
}

pub fn delete_file(filename: &Path) -> bool
{
    match remove_file(filename)
    {
        Ok(_) => true,
        Err(e) => false
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
