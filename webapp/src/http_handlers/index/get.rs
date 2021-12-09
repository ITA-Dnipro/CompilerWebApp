use rocket_dyn_templates::{Template};

use crate::http_handlers::sessions::{Session, UniPath};
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
    let source_code = match session.source_path
    {
        UniPath::FsPath(path) =>
        {
            std::fs::read_to_string(path)
                .unwrap_or("".to_owned())
        },
        _ => "".to_owned()
    };
    
    let context = IndexContext::new(source_code);

    Template::render("index", context)
}
