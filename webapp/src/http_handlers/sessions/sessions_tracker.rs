use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};

use crate::filework::delete_folder;

use super::session::Session;

// Tracks anonymous user sessions
pub struct SessionsTracker 
{
    pub sessions: HashSet<Session>,
    pub life_duration: Duration
}

impl SessionsTracker
{
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: HashSet::<Session>::new(),
            life_duration: Duration::days(0)
        }
    }

    pub fn life_duration(mut self, duration: &Duration) -> Self
    {
        self.life_duration = duration.to_owned();
        self
    }

    pub fn get_session(&self, id: u128) -> Option<&Session>
    {
        self.sessions.get(&Session::with_id(id))
    }

    pub fn insert_session(&mut self, session: Session) -> bool
    {
        self.sessions.insert(session)
    }

    pub fn set_last_connection(&mut self, id: u128, new_date: &DateTime<Utc>) -> bool
    {
        match self.get_session(id)
        {
            Some(session) => 
            {
                // TODO: rewrite sessions field of the tracker to HashMap

                true
            },
            None => false
        }
    }

    pub fn delete_old(&mut self) -> usize
    {
        let now = Utc::now();
        // TODO: Use it when drain_filter it becomes stable
        // self.sessions.drain_filter(|session| now - session.last_connection > self.life_duration).count() 
        let sessions_iter = self.sessions.iter();
        let duration = self.life_duration;
        let to_delete = sessions_iter.filter(|session| 
            now - session.last_connection > duration).map(|session| session.to_owned()).collect::<HashSet<Session>>();
        let mut deleted: usize = 0;
        
        for session in to_delete
        {
            delete_folder(&session.folder);
            self.sessions.remove(&session);
            deleted += 1;
        }

        deleted
    }
}
