#![allow(dead_code)]
use std::{collections::HashMap, path::Path};
use std::time::Duration;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::filework::delete_folder;
use super::session::Session;

/// Anonymous user sessions tracker
#[derive(Serialize, Deserialize)]
pub struct SessionsTracker 
{
    pub sessions: HashMap<u128, Session>,
    pub life_duration: Duration
}

impl SessionsTracker
{
    /// Creates a new, empty SessionsTracker
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: HashMap::<u128, Session>::new(),
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

    /// Returns a ref to a tracked session by its id
    pub fn get_session(&self, id: &u128) -> Option<&Session>
    {
        self.sessions.get(&id)
    }

    /// Returns a mut ref to a tracked session by its id
    pub fn get_mut_session(&mut self, id: &u128) -> Option<&mut Session>
    {
        self.sessions.get_mut(id)
    }

    /// Adds a new session to be tracked
    pub fn insert_session(&mut self, session: Session)
    {
        self.sessions.insert(session.id, session);
    }

    /// Clears expired sessions
    pub fn delete_old(&mut self) -> usize
    {
        let now = Utc::now();
        let sessions_iter = self.sessions.keys();
        let duration =  chrono::Duration::from_std(self.life_duration.clone()).unwrap();
        let to_delete = sessions_iter.filter(|s_id| 
            now - self.sessions[s_id].last_connection > duration).map(|s_id| s_id.to_owned()).collect::<Vec<u128>>();
        let mut deleted: usize = 0;
        
        for s_id in to_delete
        {
            delete_folder(&self.sessions[&s_id].folder);
            self.sessions.remove(&s_id);
            deleted += 1;
        }

        deleted
    }
}
