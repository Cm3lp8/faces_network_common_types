use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::faces_network_errors::FNtwrkCommonTypesErrors;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPeersInfos {
    peer_id: Uuid,
    username: String,
    created_at: DateTime<Utc>,
}

impl UserPeersInfos {
    pub fn new(peer_id: Uuid, username: String, created_at: DateTime<Utc>) -> Self {
        Self {
            peer_id,
            username,
            created_at,
        }
    }
}
