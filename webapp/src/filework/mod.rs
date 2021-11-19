#![allow(unused)]
use std::fs::{File, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::env;
use std::path::{PathBuf, Path};
use std::error::Error;

pub fn write_source_to_file(
    source_code: &str, 
    lang_extension: &str,
    parent_folder: &Path,
    session_id: &str,
    logger: &slog::Logger) 
    -> Option<PathBuf>
{
    let input_file = parent_folder.to_owned().join(
        &("source-".to_owned() + session_id + lang_extension)
    );

    if !input_file.is_absolute()
    {
        error!(logger, "File path is not valid: {:?}", input_file);
        return None;
    }

    let mut code_file: File;
    match File::create(&input_file)
    {
        Ok(file) => code_file = file,
        Err(_) =>
        {
            error!(logger, "Couldn't create a file at {:?}", input_file);
            return None;
        }
    }
        
    match code_file.write_all(source_code.as_bytes())
        .and_then(|()| code_file.seek(SeekFrom::Start(0)))
    {
        Ok(_) => {},
        Err(_) =>
        {
            error!(logger, "Couldn't write to {:?}", input_file);
            drop(code_file);
            delete_file(Path::new(&input_file));
            return None;
        }
    }

    Some(PathBuf::from(input_file))
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
