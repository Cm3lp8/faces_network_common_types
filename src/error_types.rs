use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserLoginResponseErrorKind {
    WrongPwd { user_name: String },
    UserNotFound { user_name: String },
}
