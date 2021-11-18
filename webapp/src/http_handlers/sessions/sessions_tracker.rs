use std::collections::HashSet;

use super::session::Session;

// Tracks anonymous user sessions
pub struct SessionsTracker 
{
    pub sessions: HashSet<Session>
}

impl SessionsTracker
{
    pub fn new() -> SessionsTracker
    {
        SessionsTracker
        {
            sessions: HashSet::<Session>::new()
        }
    }

    pub fn get_session(&self, id: u128) -> Option<&Session>
    {
        self.sessions.get(&Session::with_id(id))
    }

    pub fn insert_session(&mut self, session: Session) -> bool
    {
        self.sessions.insert(session)
    }
}
