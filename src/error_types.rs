pub enum UserLoginResponseErrorKind {
    WrongPwd { user_name: String },
    UserNotFound { user_name: String },
}
