use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};

use crate::filework::delete_folder;

use super::session::Session;

// Tracks anonymous user sessions
pub struct SessionsTracker 
{
    pub sessions: HashMap<u128, Session>,
    pub life_duration: Duration
}

impl SessionsTracker
{
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: HashMap::<u128, Session>::new(),
            life_duration: Duration::days(0)
        }
    }

    pub fn life_duration(mut self, duration: &Duration) -> Self
    {
        self.life_duration = duration.to_owned();
        self
    }

    pub fn get_session(&self, id: &u128) -> Option<&Session>
    {
        self.sessions.get(&id)
    }

    pub fn get_mut_session(&mut self, id: &u128) -> Option<&mut Session>
    {
        self.sessions.get_mut(id)
    }

    pub fn insert_session(&mut self, session: Session)
    {
        self.sessions.insert(session.id, session);
    }

    

    pub fn delete_old(&mut self) -> usize
    {
        let now = Utc::now();
        // TODO: Use it when drain_filter it becomes stable
        // self.sessions.drain_filter(|session| now - session.last_connection > self.life_duration).count() 
        let sessions_iter = self.sessions.keys();
        let duration = self.life_duration;
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
