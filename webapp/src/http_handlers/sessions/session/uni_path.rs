use std::path::PathBuf;

use serde::{Serialize, Deserialize};

/// ## A unified path specifier.
/// ----
/// Variants:
/// ---
/// * `FsPath` - a filesystem path;
/// * `UrlPath` - a URL string.
#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum UniPath
{
    FsPath(PathBuf),
    UrlPath(String)
}
