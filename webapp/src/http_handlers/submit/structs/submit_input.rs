use serde::Deserialize;

/// ## Request body of `POST /submit`.
/// ----
/// Fields:
/// ---
/// * `lang` - name of the source code language;
/// * `options` - compilation flags;
/// * `code` - source code.
#[derive(Deserialize)]
pub struct SubmitInput
{
    pub lang: String,
    pub options: String,
    pub code: String
}
