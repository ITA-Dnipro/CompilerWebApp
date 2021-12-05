use serde::Serialize;

/// ## Template context for `index.html.terra`
/// ----
/// Fields:
/// ---
/// * `source_code` - session's last stored source code.
#[derive(Serialize)]
pub(crate) struct IndexContext 
{
    source_code: String
}

impl IndexContext
{
    /// ## Creates a new `IndexContext` instance.
    /// ----
    /// Args:
    /// ---
    /// * `code` - session's last stored source code.
    pub fn new(code: String) -> IndexContext
    {
        IndexContext 
        {
            source_code: code
        }
    }
}
