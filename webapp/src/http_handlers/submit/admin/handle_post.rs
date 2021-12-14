use std::env;
use std::sync::Arc;
use std::collections::HashMap;
use slog::Logger;
use rocket::response::Redirect;
use rocket::http::{CookieJar, Cookie};
use rocket::State;
use rocket::form::Form;
use super::admin_user::AdminUser;
use super::consts::*;
use configurable::Configurable;

/// ## Base admin panel requests handler
/// ### Args
/// * `admin_user`
/// 
/// Rocket guard, only authorized users are allowed
/// to edit configs
/// 
/// * `settings` 
/// 
/// Data from the request, these have to be handle by
/// handlers
/// 
/// * `config_handlers`
/// 
/// Vector of handlers that handle post data,
/// handlers are structs that implemet `Configurable` trait. It is used
/// to obtain config values as well as to change them. Handlers
/// have to be managed by rocket with .manage(handlers)
/// 
/// * `logger`
/// 
/// Logger to log to
/// #
#[post("/admin", data="<settings>")]
pub fn handle_post_configs
(
    _admin_user: AdminUser,
    settings: Form<Vec<HashMap<String, String>>>,
    config_handlers: &State<Vec<Box<dyn Configurable>>>,
    logger: &State<Arc<Logger>>
) -> Redirect
{
    info!(logger, "Set following configuration from remote: {:?}", settings);
    for (sets, handler) in settings.iter().zip(config_handlers.iter())
    {
        if let Err(_what) = handler.set_settings(sets.clone()) 
        {
            error!(logger, "Failed to set_settings: {:?}", sets.clone());
        }
    }
    Redirect::to(uri!("/admin"))
}

#[post("/admin", data="<token>", rank=2)]
pub fn handle_authorization(
    cookies: &CookieJar<'_>,
    token: String,
    logger: &State<Arc<Logger>>
) -> Redirect
{
    match env::var(ENV_ADMIN_TOKEN)
    {
        Ok(env_token) =>
        {
            if env_token == token
            {
                let cookie = Cookie::build(ADMIN, "true")
                    .finish();
                cookies.add_private(cookie);
            }
        },
        Err(_) => 
        {
            error!(logger, "Environment token variable is not set")
        }
    }
    
    Redirect::to(uri!("/admin"))
}