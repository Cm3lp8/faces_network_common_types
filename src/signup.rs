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
}

impl SignupResponseData {
    pub fn new(username: &str, uuid: Uuid) -> Self {
        Self {
            username: username.to_owned(),
            unique_id: uuid,
        }
    }
}
