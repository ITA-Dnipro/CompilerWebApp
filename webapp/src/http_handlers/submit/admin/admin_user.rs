use rocket;
use rocket::request::{FromRequest, Outcome, Request};
use slog::Logger;
use std::sync::Arc;
use super::consts::*;

pub struct AdminUser;

/// The only purpose is to implement simple authorization
/// system
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        let cookies = req.cookies();
        let logger = req.rocket().state::<Arc<Logger>>().unwrap();
        match cookies.get_private(ADMIN)
        {
            Some(admin) =>
            {
                match admin.value().parse::<bool>()
                {
                    Ok(is_admin) => 
                    {
                        if is_admin 
                        {
                            Outcome::Success(AdminUser)
                        } else 
                        {
                            Outcome::Forward(())
                        }
                    },
                    Err(_) => 
                    {
                        error!(logger, "Failed to parse  `{ADMIN}` value from cookie");
                        Outcome::Forward(())
                    }
                }
            },
            None => Outcome::Forward(())
        }
    }
}