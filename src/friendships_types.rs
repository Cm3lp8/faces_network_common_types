use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Deserialize, Encode, Decode, Clone, PartialEq, Eq, Hash)]
pub struct FriendRegisterDelta {
    pulled_version: u64,
    delta_collection: Vec<FriendContact>,
}

impl FriendRegisterDelta {
    pub fn new(delta_collection: Vec<FriendContact>, pulled_version: u64) -> Self {
        Self {
            pulled_version,
            delta_collection,
        }
    }

    pub fn get_pulled_version(&self) -> u64 {
        self.pulled_version
    }
    pub fn take_collection(&mut self) -> Vec<FriendContact> {
        std::mem::take(&mut self.delta_collection)
    }
}

#[derive(Serialize, Debug, Deserialize, Encode, Decode, Clone, PartialEq, Eq, Hash)]
pub struct FriendContact {
    username: String,
    user_id: [u8; 16],
}

impl FriendContact {
    pub fn new(username: String, user_id: Uuid) -> Self {
        Self {
            username,
            user_id: user_id.into_bytes(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn take_username(&mut self) -> String {
        std::mem::take(&mut self.username)
    }
    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.user_id)
    }
}
