/// POST method handling for "/submit" route
pub mod post;
/// Structs used by the "/submit" endpoint
mod structs;

pub use post::post_submit;
