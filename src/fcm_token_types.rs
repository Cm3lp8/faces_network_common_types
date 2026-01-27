use bincode::{Decode, Encode};
use uuid::Uuid;

#[derive(Encode, Decode, Debug)]
pub struct FcmToken {
    user_id: [u8; 16],
    fcm_token: String,
    device_id: [u8; 16],
}

impl FcmToken {
    pub fn new(user_id: Uuid, fcm_token: String, device_id: Uuid) -> Self {
        Self {
            user_id: user_id.into_bytes(),
            fcm_token,
            device_id: device_id.into_bytes(),
        }
    }

    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.user_id)
    }
    pub fn device_id(&self) -> Uuid {
        Uuid::from_bytes(self.device_id)
    }
    pub fn fcm_token(&self) -> &str {
        self.fcm_token.as_str()
    }
}
