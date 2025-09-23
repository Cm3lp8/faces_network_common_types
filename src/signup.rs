use serde::{Deserialize, Serialize};

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
