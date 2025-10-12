use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Clone, Eq)]
pub struct ServerContextVersion {
    context_id: Uuid,
    version: u64,
}

impl ServerContextVersion {
    pub fn new(context_id: Uuid, version: u64) -> Self {
        Self {
            context_id,
            version,
        }
    }
    pub fn context_id(&self) -> Uuid {
        self.context_id
    }
    pub fn context_version(&self) -> u64 {
        self.version
    }
}
