/// `POST` method handling for `"/submit"` route
pub mod post;
pub mod admin;

/// Structs used by the "/submit" endpoint
mod structs;

pub use post::post_submit;
pub use admin::handle_post::handle_post_configs;
pub use admin::handle_post::handle_authorization as admin_authorization;
pub use admin::handle_get::admin_panel as admin_panel;
pub use admin::handle_get::authorize_admin as authorize_admin;
