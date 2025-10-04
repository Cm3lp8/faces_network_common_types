use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::faces_network_errors::FNtwrkCommonTypesErrors;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Clone, Eq)]
pub struct UserDisplayContext {
    context_id: Uuid,
    author_id: Uuid,
    context_kind: UserContextKind,
    created_at: DateTime<Utc>,
}

impl UserDisplayContext {
    pub fn new(
        context_id: Uuid,
        author_id: Uuid,
        context_kind: UserContextKind,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            context_id,
            author_id,
            context_kind,
            created_at,
        }
    }

    pub fn context_id(&self) -> Uuid {
        self.context_id
    }
    pub fn author_id(&self) -> Uuid {
        self.author_id
    }
    pub fn context_kind(&self) -> UserContextKind {
        self.context_kind
    }
    pub fn create_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UserContextKind {
    Experiment,
    Conversation,
}

const EXPERIMENT_CONTEXT_KIND: &'static str = "experiment";
const CONVERSATION_CONTEXT_KIND: &'static str = "conversation";

pub trait ToUserContextKind {
    fn to_user_context_kind(self) -> Result<UserContextKind, FNtwrkCommonTypesErrors>;
}

impl ToUserContextKind for &str {
    fn to_user_context_kind(self) -> Result<UserContextKind, FNtwrkCommonTypesErrors> {
        match self {
            EXPERIMENT_CONTEXT_KIND => Ok(UserContextKind::Experiment),
            CONVERSATION_CONTEXT_KIND => Ok(UserContextKind::Conversation),
            _ => Err(FNtwrkCommonTypesErrors::ParsingFailure(format!(
                "Failed to parse user context kind str from db [{}]",
                self
            ))),
        }
    }
}
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Deserialize, Clone)]
pub struct UserPeersInfos {
    peer_id: Uuid,
    username: String,
    created_at: DateTime<Utc>,
    context_participation: Vec<PeerContextParticipation>,
}

impl UserPeersInfos {
    pub fn new(
        peer_id: Uuid,
        username: String,
        created_at: DateTime<Utc>,
        context_participation: Vec<PeerContextParticipation>,
    ) -> Self {
        Self {
            peer_id,
            username,
            created_at,
            context_participation,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct PeerContextParticipation {
    context_id: Uuid,
}

impl PeerContextParticipation {
    pub fn new(context_id: Uuid) -> Self {
        Self { context_id }
    }
}
