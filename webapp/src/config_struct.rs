use std::{collections::hash_map::HashMap, path::PathBuf};
use serde::Deserialize;

/// Holds backend logic configurations
/// ===
/// ## Fields:
/// 
/// * `sessions_data_dir` - a folder where sessions data folders weill be stored;
/// * `sessions_data_file` - a file, where sessions tracker data will be serialized to and deserialized from;
/// ----
/// _All time-based fields store time values in milliseconds._
/// 
/// * `session_life_duration` - amount of time a session can be active for;
/// * `sessions_cleanup_interval` - interval of time that server waits before cleaning up expired sessions from the tracker and `sessions_data_dir`;
/// * `sessions_save_interval` - interval of time that server waits before saving sessions tracker to the `sessions_data_file`;
/// ----
/// * `lang_extensions` - a map of languages and their respective source code files extensions.
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
