use std::fs::{File};
use std::path::PathBuf;
use std::io::Error;
use std::env;

enum Mode{
    Read,
    Write
}

pub(crate) struct ConfigIO
{
}

impl ConfigIO
{
    fn get_config_file(mode: Mode) -> Option<File>
    {
        const ENV_VAR: &str = "CWA_RUNNER_CONFIG_PATH";
        const CONFIG_FILENAME: &str = "config.yaml";
        const ETC_DEFAULT: &str = "/etc/CompilerWebApp";
        let etc_file_path = PathBuf::from(ETC_DEFAULT).join(CONFIG_FILENAME);

        if let Ok(config_path) = env::var(ENV_VAR) 
        {
            let config_path_buf = PathBuf::from(config_path);
            if let Ok(reader) = Self::get_file(&mode, config_path_buf)
            {
                return Some(reader)
            }
        }
        if let Ok(cwd) = env::current_dir()
        {
            let cwd_file_path = PathBuf::from(cwd).join(CONFIG_FILENAME);
            if let Ok(reader) = Self::get_file(&mode, cwd_file_path.clone())
            {
                return Some(reader)
            }
        };
        if let Ok(reader) = Self::get_file(&mode, etc_file_path.clone())
        {
            return Some(reader)
        }

        None
    }

    fn get_file(mode: &Mode, path: PathBuf) -> Result<File, Error>
    {
        match mode 
        {
            Mode::Read => File::open(path),
            Mode::Write => File::create(path)
        }
    }

    /// ## Gets file to read from
    /// File is to be find in following order:
    /// 1) `CWA_RUNNER_CONFIG_PATH` environment varialbe
    /// 2) < current working directory >/config.yaml
    /// 3) /etc/CompilerWeApp/config.yaml
    pub fn get_config_reader() -> Option<File> 
    {
        Self::get_config_file(Mode::Read)
    }

    pub fn get_config_writer()-> Option<File>
    {
        Self::get_config_file(Mode::Write)
    }
}