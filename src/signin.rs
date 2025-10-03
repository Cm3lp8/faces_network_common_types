use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{UserDisplayContext, UserPeersInfos};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninData {
    username: String,
    password: String,
}
impl SigninData {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
    pub fn username(&self) -> &str {
        self.username.as_str()
    }
    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninResponseData {
    username: String,
    unique_id: Uuid,
    user_creation_ts: DateTime<Utc>,
    jwt: String,
    user_display_contexts: Vec<UserDisplayContext>,
    user_peer_infos: Vec<UserPeersInfos>,
}

impl SigninResponseData {
    pub fn new(
        username: &str,
        uuid: Uuid,
        user_creation_ts: DateTime<Utc>,
        jwt: String,
        user_display_contexts: Vec<UserDisplayContext>,
        user_peer_infos: Vec<UserPeersInfos>,
    ) -> Self {
        Self {
            username: username.to_owned(),
            unique_id: uuid,
            user_creation_ts,
            jwt,
            user_display_contexts,
            user_peer_infos,
        }
    }
    pub fn id(&self) -> Uuid {
        self.unique_id
    }
    pub fn username(&self) -> &str {
        self.username.as_str()
    }
    pub fn jwt(&self) -> &str {
        self.jwt.as_str()
    }
    pub fn take_user_display_contexts(&mut self) -> Vec<UserDisplayContext> {
        std::mem::take(&mut self.user_display_contexts)
    }
    pub fn user_peer_infos(&mut self) -> Vec<UserPeersInfos> {
        std::mem::take(&mut self.user_peer_infos)
    }
    pub fn user_creation_ts(&self) -> DateTime<Utc> {
        self.user_creation_ts
    }
}
