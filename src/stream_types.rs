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

#[derive(Serialize, Debug, Deserialize, Encode, Decode, Clone)]
pub enum StreamMessage {
    InvitationRequest {
        req_id: [u8; 16],
        invitation_message_id: [u8; 16],
        emitter_id: [u8; 16],
        emitter_name: String,
        dest_id: [u8; 16],
    },
    ContactRequest {
        req_id: [u8; 16],
        peer_id: [u8; 16],
    },
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
    pub fn new_invitation_request(
        invitation_message_id: Uuid,
        emitter_id: Uuid,
        emitter_name: &str,
        dest_id: Uuid,
    ) -> (StreamMessageId, Self) {
        let req_id = Uuid::now_v7();
        (
            StreamMessageId { req_id },
            Self::InvitationRequest {
                req_id: req_id.into_bytes(),
                invitation_message_id: invitation_message_id.into_bytes(),
                emitter_id: emitter_id.into_bytes(),
                emitter_name: emitter_name.to_string(),
                dest_id: dest_id.into_bytes(),
            },
        )
    }
    pub fn get_request_id(&self) -> Uuid {
        match self {
            Self::ContactRequest { req_id, peer_id: _ }
            | Self::InvitationRequest {
                req_id,
                invitation_message_id: _,
                emitter_id: _,
                emitter_name: _,
                dest_id: _,
            } => Uuid::from_bytes(*req_id),
        }
    }
    pub fn get_peer_id(&self) -> Uuid {
        match self {
            Self::ContactRequest { req_id: _, peer_id } => Uuid::from_bytes(*peer_id),
            Self::InvitationRequest {
                req_id: _,
                invitation_message_id: _,
                emitter_id: _,
                emitter_name: _,
                dest_id,
            } => Uuid::from_bytes(*dest_id),
        }
    }
}

pub struct StreamMessageId {
    req_id: Uuid,
}
