use std::fs::{File, create_dir, remove_dir_all, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::path::{PathBuf, Path};

use slog::Logger;

pub fn new_session_folder(
    parent_folder: &Path, 
    session_id: &str,
    logger: &Logger
) -> Option<PathBuf>
{
    let dir_name = parent_folder.join(session_id);
    match create_dir(&dir_name)
    {
        Ok(_) => 
        {
            info!(logger, "Session folder created at: {:?}", dir_name);

            Some(dir_name)
        },
        Err(_) =>
        {
            error!(logger, "Couldn't create a folder: {:?}", dir_name);

            None
        }
    }
}

pub fn delete_folder(
    folder: &Path
) -> bool
{
    match remove_dir_all(folder)
    {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn save_source(
    source_code: &str, 
    lang_extension: &str,
    parent_folder: &Path,
    session_id: &str,
    logger: &Logger) 
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
        .and_then(|_| code_file.seek(SeekFrom::Start(0)))
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
        Err(_) => false
    }    
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
