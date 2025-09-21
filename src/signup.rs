use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupData {
    id: String,
    password: String,
}
impl SignupData {
    pub fn new(id: String, password: String) -> Self {
        Self { id, password }
    }
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}
