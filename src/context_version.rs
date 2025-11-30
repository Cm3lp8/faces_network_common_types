use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Clone, Eq)]
/// [`ServerContextVersion`] represents two version counter states from the server.
/// It shows the actual server session version for the user and a specific context version
pub struct ServerContextVersion {
    context_id: Uuid,
    context_version: u64,
    user_session_version: u64,
}

impl ServerContextVersion {
    pub fn new(context_id: Uuid, context_version: u64, user_session_version: u64) -> Self {
        Self {
            context_id,

            context_version,
            user_session_version,
        }
    }
    pub fn context_id(&self) -> Uuid {
        self.context_id
    }
    pub fn context_version(&self) -> u64 {
        self.context_version
    }
    pub fn user_session_version(&self) -> u64 {
        self.user_session_version
    }
}
