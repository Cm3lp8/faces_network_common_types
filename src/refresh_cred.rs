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

#[derive(Clone, Encode, Decode)]
pub struct JwtToken {
    token: String,
}

impl JwtToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Clone, Encode, Decode)]
pub struct NewGeneratedAuthTokens {
    refresh_token: RefreshToken,
    jwt_token: JwtToken,
}

impl NewGeneratedAuthTokens {
    pub fn new(refresh_token: String, jwt_token: String) -> Self {
        Self {
            refresh_token: RefreshToken::new(refresh_token),
            jwt_token: JwtToken::new(jwt_token),
        }
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token.token()
    }
    pub fn jwt(&self) -> &str {
        &self.jwt_token.token()
    }
}
