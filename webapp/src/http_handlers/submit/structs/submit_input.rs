use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitInput
{
    pub lang: String,
    pub options: String,
    pub code: String
}
