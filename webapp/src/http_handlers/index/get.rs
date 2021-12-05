use rocket_dyn_templates::{Template};

use crate::http_handlers::sessions::Session;
use super::index_context::IndexContext;

/// ## `GET /` handler.
/// ----
/// Args:
/// ---
/// * `session` - current session.
/// ----
/// Returns:
/// A rendered `index.html.tera`. Is session doesn't store user's source code yet, it's value will be `""`.
#[get("/")]
pub async fn get_index(session: Session) -> Template
{
    let source_code = std::fs::read_to_string(session.source_path)
        .unwrap_or("".to_owned());
    
    let context = IndexContext::new(source_code);

    Template::render("index", context)
}
