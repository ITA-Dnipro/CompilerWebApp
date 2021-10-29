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
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String
}
