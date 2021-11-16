

use rocket::{Request, http::Cookie, request::{self, FromRequest}};

pub struct Session
{
    pub id: u128
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error>
    {
        let logger = req.rocket().state::<slog::Logger>().unwrap();
        // TODO: manage sessions saving and checks
        match req.cookies().get("session_id")
        {
            Some(session_id) => 
            {
                trace!(logger, "Request from session: {}", session_id);

                request::Outcome::Success(Session { id: session_id.value().parse::<u128>().unwrap() })
            },
            None =>
            {
                let session_id = uuid::Uuid::new_v4().as_u128();
                req.cookies().add(Cookie::new("session_id", session_id.to_string()));
                info!(logger, "New session established: {}", session_id);

                request::Outcome::Success(Self { id: session_id })
            }
        }
    }
}
