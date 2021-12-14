#![allow(dead_code)]
use std::sync::{Mutex, MutexGuard};
use std::{collections::HashMap, path::Path};
use std::time::Duration;
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::filework::delete_folder;
use super::session::Session;

/// ## Anonymous user sessions tracker.
/// _Internal sessions data is wrapped in a mutex, so it is save to use between threads as is._
/// 
/// ------
/// Fields:
/// * `sessions` - tracked sessions. They are not guaranteed to not be expired at all times;
/// * `life_duration` - life span of a session, after which it is considered expired.
#[derive(Serialize, Deserialize)]
pub struct SessionsTracker 
{
    pub sessions: Mutex<HashMap<u128, Session>>,
    pub life_duration: Duration
}

impl SessionsTracker
{
    /// Creates a new, empty `SessionsTracker`
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: Mutex::new(HashMap::<u128, Session>::new()),
            life_duration: Duration::from_millis(0)
        }
    }

    /// ## Deserializes a `SessionsTracker` istance from a `.json` file.
    /// 
    /// ----
    /// Args:
    /// ---
    /// * `path` - path to the file.
    /// ----
    /// ## Returns:
    /// `Some(SessionsTracker)` is deserialization was successful, `None` otherwise.  
    /// 
    /// `None` may be returned if the file couldn't be read, or its contents could not be deserialized.
    pub fn from_file(path: &Path) -> Option<SessionsTracker>
    {
        let contents;
        match std::fs::read_to_string(path)
        {
            Ok(conts) => contents = conts,
            Err(_) => return None
        }

        match serde_json::from_str(&contents)
        {
            Ok(deser) => Some(deser),
            Err(_) => None
        }
    }

    /// ## Serializes `SessionsTracker` into a `.json` file.
    /// 
    /// ----
    /// Args:
    /// ---
    /// * `path` - path to the file.
    /// ----
    /// ## Returns:
    /// * `true`, if serialization was successful;
    /// * `false`, if the object could not be serialized to text, or the file could not be written to.
    pub fn save(&self, path: &Path) -> bool
    {
        let serialized;
        match serde_json::to_string_pretty(&self)
        {
            Ok(ser) => serialized = ser,
            Err(_) => return false
        }

        match std::fs::write(path, serialized)
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// ## Builder method, that sets sessions life duration.
    /// ----
    /// Args:
    /// ---
    /// * `duration` - new `life_duration` value.
    pub fn life_duration(mut self, duration: &Duration) -> Self
    {
        self.life_duration = duration.to_owned();
        self
    }

    /// ## Locks `sessions_data`.
    /// ----
    /// ## Returns:
    /// `MutexGuard` for `sessions`. If the mutex is poisoned - it will end the process with the exit code of 1.
    fn lock_sessions(&self) -> MutexGuard<HashMap<u128, Session>>
    {
        self.sessions.lock().unwrap_or_else(|_| std::process::exit(1))
    }
    
    /// ## Sets session's `source_path` field.
    /// ----
    /// Args:
    /// ---
    /// * `session_id` - id of the session to modify;
    /// * `sourse_path` - new field value.
    pub fn set_source_file(&self, session_id: &u128, source_path: &Path)
    {
        if let Some(session) = self.lock_sessions().get_mut(session_id)
        {
            session.set_source(source_path);
        }
    }

    /// ## Sets session's `last_connection` field.
    /// ----
    /// Args:
    /// ---
    /// * `session_id` - id of the session to modify;
    /// * `new_date` - new field value.
    pub fn set_last_conn_timestamp(&self, session_id: &u128, new_date: DateTime<Utc>)
    {
        let sessions = &mut self.lock_sessions();
        if let Some(session) = sessions.get_mut(&session_id)
        {
            session.last_connection_timestamp = new_date;
        }
    }

    /// ## Returns a cloned `Session` object from `sessions`.
    /// ----
    /// Args:
    /// ---
    /// * `session_id` - id the session to return.
    /// ----
    /// ## Returns:
    /// `Some<Session>`, if the session is present in the `sessions`, `None` if it is not.
    pub fn get_session(&self, session_id: &u128) -> Option<Session>
    {
        if let Some(session) = self.lock_sessions().get(session_id)
        {
            Some(session.to_owned())
        }
        else
        {
            None
        }
    }

    /// ## Adds a new session to be tracked.
    /// ----
    /// Args:
    /// ---
    /// * `session` - a `Session` to insert.
    pub fn insert_session(&self, session: Session)
    {
        self.lock_sessions().insert(session.id, session);
    }

    /// ## Clears expired sessions.
    /// 
    /// Session is expired if `current time - last_connection` is more than `life_duration`.
    /// 
    /// Deletes sessions from the `sessions` fields, as well as their respective folder.
    /// 
    /// ----
    /// ## Returns:
    /// Amount of deleted sessions.
    pub fn delete_old(&self) -> usize
    {
        let now = Utc::now();
        let sessions = &mut self.lock_sessions();
        let sessions_iter = sessions.keys();
        let duration =  chrono::Duration::from_std(self.life_duration.clone()).unwrap();
        let to_delete = sessions_iter.filter(|s_id| 
            now - sessions[s_id].last_connection_timestamp > duration)
                .map(|s_id| s_id.to_owned()).collect::<Vec<u128>>();
        let mut deleted: usize = 0;
        
        for s_id in to_delete
        {
            delete_folder(&sessions[&s_id].folder);
            sessions.remove(&s_id);
            deleted += 1;
        }

        deleted
    }
}
