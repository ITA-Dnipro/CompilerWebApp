use std::{collections::hash_map::HashMap, path::PathBuf};
use serde::Deserialize;

/// Holds backend logic configurations
#[derive(Debug, Deserialize)]
pub struct BackendConfig 
{
    pub sessions_data_dir: PathBuf,
    pub sessions_data_file: PathBuf,
    pub session_life_duration: u64,
    pub sessions_cleanup_interval: u64,
    pub sessions_save_interval: u64,
    pub lang_extensions: HashMap<String, String>
}
