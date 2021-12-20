use std::sync::Arc;
use std::collections::HashMap;
use slog::Logger;
use rocket::{State};
use rocket_dyn_templates::Template;
use super::super::super::sessions::Session;
use super::admin_user::AdminUser;
use super::config_value::Context;
use super::consts::*;
use configurable::Configurable;


#[get("/admin")]
pub async fn admin_panel
(
    _admin: AdminUser,
    _logger: &State<Arc<Logger>>,
    config_handler: &State<Vec<Box<dyn Configurable<'_>>>>
) -> Template
{
    let mut settings:Vec<HashMap<String, String>> = Vec::new();
    for handler in config_handler.iter()
    {
        settings.push(handler.get_settings().unwrap())
    }
    let context = Context {
        settings
    };    

    Template::render(OPTIONS_LIST, context)
}

#[get("/admin", rank = 2)]
pub fn authorize_admin
(
    _logger: &State<Arc<Logger>>,
    _session: Session
) -> Template
{
    let context: HashMap<&str, String> = HashMap::new();
    
    Template::render(ADMIN_PANEL, context)
}