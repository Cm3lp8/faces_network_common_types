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
    jwt: String,
    first_experiment_display_context_id: Uuid,
}

impl SignupResponseData {
    pub fn new(
        username: &str,
        uuid: Uuid,
        jwt: String,
        first_experiment_display_context_id: Uuid,
    ) -> Self {
        Self {
            username: username.to_owned(),
            unique_id: uuid,
            jwt,
            first_experiment_display_context_id,
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
}
