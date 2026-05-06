use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use super::session::PtySession;

#[derive(Default)]
pub struct PtyManager {
    sessions: Mutex<HashMap<String, Arc<PtySession>>>,
}

impl PtyManager {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&self, sess: PtySession) -> String {
        let id = Uuid::new_v4().to_string();
        self.sessions.lock().insert(id.clone(), Arc::new(sess));
        id
    }

    pub fn get(&self, id: &str) -> Option<Arc<PtySession>> {
        self.sessions.lock().get(id).cloned()
    }

    pub fn remove(&self, id: &str) -> Option<Arc<PtySession>> {
        self.sessions.lock().remove(id)
    }
}
