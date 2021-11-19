use std::{collections::hash_map::HashMap, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BackendConfig 
{
    pub sessions_data_dir: PathBuf,
    pub lang_extensions: HashMap<String, String>
}
