use std::fs::{File, create_dir, remove_dir_all, remove_file};
use std::io::{Write, Seek, SeekFrom};
use std::path::{PathBuf, Path};

use slog::Logger;

/// ## Creates a folder for a new session.
/// 
/// New folder's name is it's respective session's id.
/// ----
/// Args:
/// * `parent_folder` - a folder in which the new folder will be created;
/// * `parent_folder` - session's id;
/// * `logger` - a logger to log to.
/// ----
/// ## Returns:
/// A path to the created folder, or `None` if it could not be created.
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

/// ## Deletes a specified folder.
/// ----
/// Args:
/// ---
/// * `folder` - path to the folder to delete.
/// ----
/// ## Returns:
/// `true` if the folder was deleted, `false` if it wasn't.
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

/// ## Saves session's source code into a file.
/// 
/// New file's name is `"source-{session_id}"`.
/// 
/// ----
/// Args:
/// ---
/// * `source_code` - souce code to write into the file;
/// * `lang_extension` - new file's extension;
/// * `parent_folder` - a folder to create the file in;
/// * `session_id` - current session's id;
/// * `logger` - a logger to log to.
/// ----
/// ## Returns:
/// A path to the new file, or `None` if it could not be created.
/// 
/// `None` can occur when the file itself could not be created, or it couldn't be written to.
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

/// ## Deletes a specified file.
/// ----
/// Args:
/// ---
/// * `filename` - path to the file to delete.
/// ----
/// ## Returns:
/// `true` if the file was deleted, `false` if it wasn't.
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
