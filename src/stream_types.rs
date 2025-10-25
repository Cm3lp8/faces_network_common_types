use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserStreamSessionInfo {
    user_id: Uuid,
}
impl UserStreamSessionInfo {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

// an Id field to make possible request confirmation when client responds

#[derive(Serialize, Deserialize, Encode, Decode, Clone)]
pub enum StreamMessage {
    ContactRequest { req_id: [u8; 16], peer_id: [u8; 16] },
}

impl StreamMessage {
    pub fn new_contact_request(peer_id: Uuid) -> (StreamMessageId, Self) {
        let req_id = Uuid::now_v7();
        (
            StreamMessageId { req_id },
            Self::ContactRequest {
                req_id: req_id.into_bytes(),
                peer_id: peer_id.into_bytes(),
            },
        )
    }
    pub fn get_request_id(&self) -> Uuid {
        match self {
            Self::ContactRequest { req_id, peer_id: _ } => Uuid::from_bytes(*req_id),
        }
    }
    pub fn get_peer_id(&self) -> Uuid {
        match self {
            Self::ContactRequest { req_id: _, peer_id } => Uuid::from_bytes(*peer_id),
        }
    }
}

pub struct StreamMessageId {
    req_id: Uuid,
}
