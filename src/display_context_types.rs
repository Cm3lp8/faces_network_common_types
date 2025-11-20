use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Encode, Decode, Debug, Clone)]
pub struct DisplayContext {
    id: [u8; 16],
    participants: Vec<[u8; 16]>,
    version: u64,
    created_at: i64,
    updated_at: i64,
    kind: DisplayContextKind,
}

impl DisplayContext {
    pub fn new_multiple_participants(
        id: Uuid,
        participants: Vec<Uuid>,
        version: u64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into_bytes(),
            participants: participants.into_iter().map(|it| it.into_bytes()).collect(),
            version,
            created_at: created_at.timestamp(),
            updated_at: updated_at.timestamp(),
            kind: DisplayContextKind::Conversation,
        }
    }
    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
    pub fn participants(&self) -> Vec<Uuid> {
        self.participants
            .iter()
            .map(|it| Uuid::from_bytes(*it))
            .collect()
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.created_at, 0)
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.updated_at, 0)
    }
    pub fn kind(&self) -> DisplayContextKind {
        self.kind
    }
}

#[derive(Encode, Decode, Debug, Clone, Copy)]
pub enum DisplayContextKind {
    Conversation,
    Solo,
}
