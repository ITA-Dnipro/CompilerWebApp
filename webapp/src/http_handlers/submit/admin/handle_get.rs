use std::sync::Arc;
use slog::Logger;
use rocket::{State, http::{ContentType}, log::private::logger};
use super::super::super::sessions::Session;
use super::admin_user::AdminUser;

#[get("/admin")]
pub fn admin_panel(
    admin: AdminUser,
    logger: &State<Arc<Logger>>
) -> Result<(ContentType, &'static str), String>
{
    let html = 
    (
        ContentType::HTML,
        "
        <!DOCTYPE HTML>
        <html>
            <body>
                <div>
                    Admin
                </div>
            </body>
        </html>
        "
    );

    Ok(html)
}

#[get("/admin", rank = 2)]
pub fn authorize_admin(
    logger: &State<Arc<Logger>>,
    session: Session) -> Result<(ContentType, &'static str), String>
{
    let html = (ContentType::HTML,
    "
    <!DOCTYPE HTML>
    <form action=/admin method='post'>
        <input name='token' type=\"text\"> </input>
    </form>
    ");
    Ok(html)
}