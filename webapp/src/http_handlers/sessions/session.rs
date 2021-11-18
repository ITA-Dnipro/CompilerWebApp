use rocket::{Request, http::{Cookie, CookieJar, Status},
    request::{self, FromRequest}};
use std::{hash::{Hash, Hasher}, sync::{Arc, Mutex}};

use super::sessions_tracker::SessionsTracker;

// Anonymous session data guard
#[derive(Eq, Clone)]
pub struct Session
{
    pub id: u128
}

impl Session
{
    pub fn with_id(id: u128) -> Session
    {
        Session
        {
            id
        }
    }

    pub(crate) fn establish_new(cookies: &CookieJar, tracker: &mut SessionsTracker,
        logger: &slog::Logger) -> Option<u128>
    {
        let session_id = uuid::Uuid::new_v4().as_u128();

        if let false = tracker.insert_session(Session {
            id: session_id
        })
        {
            return None;
        }

        cookies.add(Cookie::new("session_id", session_id.to_string()));
        info!(logger, "New session established: {}", session_id);

        Some(session_id)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error>
    {
        let logger = req.rocket().state::<slog::Logger>().unwrap();
        // TODO: there theoretically is a case when mutex locking fails
        let mut sessions = req.rocket()
            .state::<Arc<Mutex<SessionsTracker>>>().unwrap().lock().unwrap();  

        match req.cookies().get("session_id")
        {
            // Check if a request already has session id
            Some(session_id) => 
            {
                info!(logger, "Request from session: {}", session_id);

                let parsed_id;
                match session_id.value().parse::<u128>()
                {
                    Ok(parsed) => parsed_id = parsed,
                    Err(_) => // Received cookie with corrupted session id
                    {
                        info!(logger, "Corrupted session ID");

                        match Session::establish_new(req.cookies(),
                        &mut sessions, logger)
                        {
                            Some(session_id) =>  return request::Outcome::Success(Self { id: session_id }),
                            None => 
                            {
                                error!(logger, "Couldn't establish a session");
                                return request::Outcome::Failure((Status::InternalServerError, ()))
                            }
                        }  
                    }
                }
                  
                match sessions.get_session(parsed_id)
                {
                    Some(session) =>
                    {
                        request::Outcome::Success(session.to_owned())
                    },
                    None =>  // Received cookie with untracked session id
                    {
                        info!(logger, "Untracked session ID");
                        let session_id = Session::establish_new(req.cookies(),
                        &mut sessions, logger).unwrap();

                        request::Outcome::Success(Self { id: session_id })
                    }
                }
            },
            None =>
            {
                match Session::establish_new(req.cookies(),
                    &mut sessions, logger)
                    {
                        Some(session_id) =>  return request::Outcome::Success(Self { id: session_id }),
                        None => 
                        {
                            error!(logger, "Couldn't establish a session");
                            return request::Outcome::Failure((Status::InternalServerError, ()))
                        }
                    }  
            }
        }
    }
}

impl Hash for Session
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.id.hash(state);
    }
}

impl PartialEq for Session
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}
