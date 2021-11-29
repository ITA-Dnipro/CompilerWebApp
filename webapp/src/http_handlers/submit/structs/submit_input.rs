use serde::Deserialize;

/// Request body of POST /submit
#[derive(Deserialize)]
pub struct SubmitInput
{
    pub lang: String,
    pub options: String,
    pub code: String
}
