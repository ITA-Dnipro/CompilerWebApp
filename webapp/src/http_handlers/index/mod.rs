/// `GET` method handling for "/" route
pub mod get;
/// Template context for `index.html.terra`
mod index_context;

pub use get::get_index;
