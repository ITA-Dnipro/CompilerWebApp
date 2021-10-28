use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct InputCode
{
    pub lang: String,
    pub options: String,
    pub code: String
}

#[derive(Serialize)]
pub struct OutputCode
{
    pub stdin: String,
    pub stdout: String,
    pub result: Vec<u8>
}
