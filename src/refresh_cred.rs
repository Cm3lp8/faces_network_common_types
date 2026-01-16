use bincode::{Decode, Encode};

#[derive(Clone, Encode, Decode)]
pub struct RefreshToken {
    token: String,
}

impl RefreshToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
