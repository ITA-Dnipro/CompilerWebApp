use std::env;
use std::sync::Arc;
use slog::Logger;
use rocket::response::Redirect;
use rocket::http::{CookieJar, Cookie};
use rocket::State;
use super::handle_get::admin_panel;

// TODO: formid to send config sets if token is not passed
#[post("/admin", data="<token>")]
pub fn handle_authorization(
    cookies: &CookieJar<'_>,
    token: String,
    logger: &State<Arc<Logger>>
) -> Redirect
{
    const COOKIE_NAME: &str = "Admin";

    match env::var("CWA_ADMIN_TOKEN")
    {
        Ok(token) =>
        {
            let cookie = Cookie::build(COOKIE_NAME, "true")
                .finish();
            cookies.add_private(cookie);
        },
        Err(_) => 
        {
            error!(logger, "Environment token variable is not set")
        }
    }
    
    Redirect::to(uri!("/admin"))
}