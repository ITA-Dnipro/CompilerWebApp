#![allow(dead_code)]
use std::sync::{Mutex, MutexGuard};
use std::{collections::HashMap, path::Path};
use std::time::Duration;
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::filework::delete_folder;
use super::session::Session;

/// Anonymous user sessions tracker
/// 
/// Internal sessions data is wrapped in a mutex, so it is save to use between threads as is
#[derive(Serialize, Deserialize)]
pub struct SessionsTracker 
{
    pub sessions: Mutex<HashMap<u128, Session>>,
    pub life_duration: Duration
}

impl SessionsTracker
{
    /// Creates a new, empty SessionsTracker
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: Mutex::new(HashMap::<u128, Session>::new()),
            life_duration: Duration::from_millis(0)
        }
    }

    /// Deserializes a SessionsTracker istance from a .json file
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

    /// Serializes SessionsTracker into a .json file
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

    /// Builder method, that sets sessions life duration
    pub fn life_duration(mut self, duration: &Duration) -> Self
    {
        self.life_duration = duration.to_owned();
        self
    }

    fn lock(&self) -> MutexGuard<HashMap<u128, Session>>
    {
        self.sessions.lock().unwrap_or_else(|_| std::process::exit(1))
    }
    
    pub fn set_source_file(&self, session_id: &u128, source_path: &Path)
    {
        if let Some(session) = self.lock().get_mut(session_id)
        {
            session.set_source(source_path);
        }
    }

    pub fn set_last_connection(&self, session_id: &u128, new_date: DateTime<Utc>)
    {
        let sessions = &mut self.lock();
        if let Some(session) = sessions.get_mut(&session_id)
        {
            session.last_connection = new_date;
        }
    }

    pub fn get_session(&self, session_id: &u128) -> Option<Session>
    {
        if let Some(session) = self.lock().get(session_id)
        {
            Some(session.to_owned())
        }
        else
        {
            None
        }
    }

    /// Adds a new session to be tracked
    pub fn insert_session(&self, session: Session)
    {
        self.lock().insert(session.id, session);
    }

    /// Clears expired sessions
    pub fn delete_old(&self) -> usize
    {
        let now = Utc::now();
        let sessions = &mut self.lock();
        let sessions_iter = sessions.keys();
        let duration =  chrono::Duration::from_std(self.life_duration.clone()).unwrap();
        let to_delete = sessions_iter.filter(|s_id| 
            now - sessions[s_id].last_connection > duration).map(|s_id| s_id.to_owned()).collect::<Vec<u128>>();
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
