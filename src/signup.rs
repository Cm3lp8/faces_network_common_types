use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupData {
    username: String,
    password: String,
}
impl SignupData {
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
pub struct SignupResponseData {
    username: String,
    unique_id: Uuid,
    user_creation_ts: DateTime<Utc>,
    jwt: String,
    first_experiment_display_context_id: Uuid,
    experiment_display_context_creation_ts: DateTime<Utc>,
}

impl SignupResponseData {
    pub fn new(
        username: &str,
        uuid: Uuid,
        user_creation_ts: DateTime<Utc>,
        jwt: String,
        first_experiment_display_context_id: Uuid,
        experiment_display_context_creation_ts: DateTime<Utc>,
    ) -> Self {
        Self {
            username: username.to_owned(),
            unique_id: uuid,
            user_creation_ts,
            jwt,
            first_experiment_display_context_id,
            experiment_display_context_creation_ts,
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
    pub fn first_experiment_display_context_id(&self) -> Uuid {
        self.first_experiment_display_context_id
    }
    pub fn user_creation_ts(&self) -> DateTime<Utc> {
        self.user_creation_ts
    }
    pub fn experiment_display_context_creation_ts(&self) -> DateTime<Utc> {
        self.experiment_display_context_creation_ts
    }
}
