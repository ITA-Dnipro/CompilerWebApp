pub mod uni_path;

use chrono::{DateTime, Utc};
use rocket::{Request, http::{Cookie, CookieJar, Status}, request::{self, FromRequest}};
use serde::{Deserialize, Serialize};
use std::{hash::{Hash, Hasher}, path::{Path, PathBuf}, sync::{Arc, RwLock}};

use crate::{config_struct::BackendConfig, filework::new_session_folder};
use super::sessions_tracker::SessionsTracker;
use uni_path::UniPath;

/// ## Anonymous session data guard.
/// ----
/// Fields:
/// ---
/// * `id` - session's identifier;
/// * `last_connection_timestamp` - session's last connection date;
/// * `folder` - a path to a folder, reserved for this session's files;
/// * `source_path` - a path to the session's source code.
#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct Session
{
    pub id: u128,
    pub last_connection_timestamp: DateTime<Utc>,
    pub folder: PathBuf,
    pub source_path: UniPath
}

impl Session
{
    /// ## Create a new `Session` instance with the given id
    /// ----
    /// Args:
    /// ---
    /// * `id` - session's id.
    pub fn with_id(id: u128) -> Session
    {
        Session
        {
            id,
            last_connection_timestamp: Utc::now(),
            folder: PathBuf::new(),
            source_path: UniPath::FsPath(PathBuf::new())
        }
    }

    /// `Session` builder method, that sets the `folder` field.
    /// ----
    /// Args:
    /// ---
    /// * `parent_folder` - a new 'folder' field value.
    pub fn folder(mut self, parent_folder: &Path) -> Session
    {
        self.folder = parent_folder.to_owned();

        self
    }

    /// ## Sets `source_path` field.
    /// ----
    /// Args:
    /// ---
    /// * `source_path` - a new `source_path` field value.
    pub fn set_source(&mut self, source_path: &Path)
    {
        self.source_path = UniPath::FsPath(source_path.to_owned());
    }

    /// ## Establishes a new session.
    /// 
    /// The new session is addded to the tracker, and a folder for it is created.
    /// 
    /// HTTP session cookie `session_id` is set to the new session's id with `htttp_only` flag set to true.
    /// ----
    /// Args:
    /// ---
    /// * `cookies` - current HTTP session cookies;
    /// * `tracker` - `SessionsTracker` to add the new session to;
    /// * `parent_folder` - a folder, in which this session's folder will be created;
    /// * `logger` - a logger to log to.
    /// ----
    /// ## Returns:
    /// New session's id, or `None` if a folder for it couldn't be created.
    pub(crate) fn establish_new(
        cookies: &CookieJar<'_>, 
        tracker: &SessionsTracker,
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

    /// ## Updates session's `last_connection` field to `Utc::now()`.
    /// ----
    /// Args:
    /// ---
    /// * `tracker` - `SessionsTracker` that holds the session;
    /// * `session_id` - session's id.
    pub(crate) fn update_session(tracker: &Arc<SessionsTracker>, session_id: &u128)
    {
        tracker.set_last_conn_timestamp(session_id, Utc::now()); 
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error>
    {
        let logger = req.rocket().state::<Arc<slog::Logger>>().unwrap();
        let mut tracker = req.rocket().state::<Arc<SessionsTracker>>().unwrap();
        let config = req.rocket().state::<RwLock<BackendConfig>>().unwrap()
            .read().unwrap_or_else(|_| std::process::exit(1));
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

                        match Session::establish_new(req.cookies(), tracker,
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
                
                match tracker.get_session(&parsed_id)
                {
                    Some(session) => // Request from a tracked session
                    {
                        Session::update_session(tracker, &parsed_id);

                        request::Outcome::Success(session)
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
