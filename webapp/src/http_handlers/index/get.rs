use rocket_dyn_templates::{Template};
use serde::Serialize;

use crate::http_handlers::sessions::Session;

/// Context for the index template
#[derive(Serialize)]
struct IndexContext 
{
    source_code: String
}

impl IndexContext
{
    pub fn new(code: String) -> IndexContext
    {
        IndexContext 
        {
            source_code: code
        }
    }
}

#[get("/")]
pub async fn get_index(session: Session) -> Template
{
    let source_code = std::fs::read_to_string(session.source_path)
        .unwrap_or("".to_owned());
    
    let context = IndexContext::new(source_code);

    Template::render("index", context)
}
