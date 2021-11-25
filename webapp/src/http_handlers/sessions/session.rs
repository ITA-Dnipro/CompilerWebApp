use chrono::{DateTime, Utc};
use rocket::{Request, http::{Cookie, CookieJar, Status}, request::{self, FromRequest}};
use serde::{Deserialize, Serialize};
use std::{hash::{Hash, Hasher}, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use crate::{config_struct::BackendConfig, filework::new_session_folder};
use super::sessions_tracker::SessionsTracker;

// Anonymous session data guard
#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct Session
{
    pub id: u128,
    pub last_connection: DateTime<Utc>,
    pub folder: PathBuf
}

impl Session
{
    pub fn with_id(id: u128) -> Session
    {
        Session
        {
            id,
            last_connection: Utc::now(),
            folder: PathBuf::new()
        }
    }

    pub fn folder(mut self, parent_folder: &Path) -> Session
    {
        self.folder = parent_folder.to_owned();
        self
    }

    pub(crate) fn establish_new(
        cookies: &CookieJar<'_>, 
        tracker: &mut SessionsTracker,
        parent_folder: &Path,
        logger: &slog::Logger) 
        -> Option<u128>
    {
        let session_id = uuid::Uuid::new_v4().as_u128();
        let session_id_str = session_id.to_string();
        let folder;
        match new_session_folder(parent_folder, &session_id_str,
            logger)
        {
            Some(new_folder) => 
            {
                folder = new_folder;
            },
            None => 
            {
                return None;
            }
        }
        
        tracker.insert_session(Session::with_id(session_id)
            .folder(&folder));
        cookies.add_private(Cookie::build("session_id", session_id_str)
            .http_only(true)
            .finish());

        info!(logger, "New session established: {}", session_id);

        Some(session_id)
    }

    pub(crate) fn update_session(session: &mut Session)
    {
        session.last_connection = Utc::now();   
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error>
    {
        // TODO: refactor this ugly piece of code
        let logger = req.rocket().state::<Arc<slog::Logger>>().unwrap();
        let mut tracker = req.rocket().state::<Arc<Mutex<SessionsTracker>>>()
            .unwrap().lock().unwrap_or_else(|_| std::process::exit(1));
        let config = req.rocket().state::<BackendConfig>().unwrap();
        let cookies = req.cookies();

        match cookies.get_private("session_id")
        {
            // Check if a request already has session id
            Some(session_cookie) => 
            {
                info!(logger, "Request from session: {}", session_cookie);

                let parsed_id;
                match session_cookie.value().parse::<u128>()
                {
                    Ok(parsed) => parsed_id = parsed,
                    Err(_) => // Received cookie with corrupted session id
                    {
                        info!(logger, "Corrupted session ID");

                        match Session::establish_new(req.cookies(),&mut tracker,
                            &config.sessions_data_dir, logger)
                        {
                            Some(session_id) =>  
                            {
                                return request::Outcome::Success(Self::with_id(session_id));
                            }
                            None => 
                            {
                                error!(logger, "Couldn't establish a session");

                                return request::Outcome::Failure((Status::InternalServerError, ()))
                            }
                        }  
                    }
                }
                
                match tracker.get_mut_session(&parsed_id)
                {
                    Some(session) => // Request from a tracked session
                    {
                        Session::update_session(session);

                        request::Outcome::Success(session.to_owned())
                    },
                    None =>  // Received cookie with untracked session id
                    {
                        info!(logger, "Untracked session ID");
                        let session_id = Session::establish_new(req.cookies(),
                        &mut tracker, &config.sessions_data_dir, logger).unwrap();

                        request::Outcome::Success(Self::with_id(session_id))
                    }
                }
            },
            None =>
            {
                match Session::establish_new(req.cookies(),
                    &mut tracker, &config.sessions_data_dir, logger)
                {
                    Some(session_id) => 
                    {
                        return request::Outcome::Success(Self::with_id(session_id))
                    },
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
