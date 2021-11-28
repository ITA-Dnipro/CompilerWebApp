use rocket_dyn_templates::{Template};
use serde::Serialize;

use crate::http_handlers::sessions::Session;

// Context for the index template
#[derive(Serialize)]
struct IndexContext 
{}

#[get("/")]
pub async fn get_index(_session: Session) -> Template
{
    Template::render("index", IndexContext {})
}
