use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::InvitationOrientation;

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
        inviter_id: [u8; 16],
        inviter_name: String,
        invitee_name: String,
        invitee_id: [u8; 16],
        ts: i64, // utc timestamp
    },
    InvitationResponse {
        req_id: [u8; 16],
        invitation_message_id: [u8; 16],
        inviter_id: [u8; 16],
        invited_id: [u8; 16],
        invitation_accepted: bool,
        ts: i64, // utc timestamp
    },
    ContactRequest {
        req_id: [u8; 16],
        peer_id: [u8; 16],
        ts: i64,
    },
}

impl StreamMessage {
    pub fn new_invitation_response_confirmation(
        invitation_message_id: Uuid,
        inviter_id: Uuid,
        invited_id: Uuid,
    ) -> Self {
        let req_id = Uuid::now_v7();
        Self::InvitationResponse {
            req_id: req_id.into_bytes(),
            invitation_message_id: invitation_message_id.into_bytes(),
            inviter_id: inviter_id.into_bytes(),
            invited_id: invited_id.into_bytes(),
            invitation_accepted: true,
            ts: Utc::now().timestamp(),
        }
    }
    pub fn new_invitation_response_refused(
        invitation_message_id: Uuid,
        inviter_id: Uuid,
        invited_id: Uuid,
    ) -> Self {
        let req_id = Uuid::now_v7();
        Self::InvitationResponse {
            req_id: req_id.into_bytes(),
            invitation_message_id: invitation_message_id.into_bytes(),
            inviter_id: inviter_id.into_bytes(),
            invited_id: invited_id.into_bytes(),
            invitation_accepted: false,
            ts: Utc::now().timestamp(),
        }
    }
    pub fn new_contact_request(peer_id: Uuid) -> (StreamMessageId, Self) {
        let req_id = Uuid::now_v7();
        (
            StreamMessageId { req_id },
            Self::ContactRequest {
                req_id: req_id.into_bytes(),
                peer_id: peer_id.into_bytes(),
                ts: Utc::now().timestamp(),
            },
        )
    }
    pub fn new_invitation_request_for_sender(
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
                inviter_id: emitter_id.into_bytes(),
                inviter_name: emitter_name.to_string(),
                invitee_name: emitter_name.to_string(),
                invitee_id: dest_id.into_bytes(),
                ts: Utc::now().timestamp(),
            },
        )
    }
    pub fn new_invitation_request_for_receiver(
        invitation_message_id: Uuid,
        inviter_id: Uuid,
        inviter_name: &str,
        invitee_name: &str,
        invitee_id: Uuid,
    ) -> (StreamMessageId, Self) {
        let req_id = Uuid::now_v7();
        (
            StreamMessageId { req_id },
            Self::InvitationRequest {
                req_id: req_id.into_bytes(),
                invitation_message_id: invitation_message_id.into_bytes(),
                inviter_id: inviter_id.into_bytes(),
                inviter_name: inviter_name.to_string(),
                invitee_name: invitee_name.to_string(),
                invitee_id: invitee_id.into_bytes(),
                ts: Utc::now().timestamp(),
            },
        )
    }
    pub fn get_request_id(&self) -> Uuid {
        match self {
            Self::ContactRequest {
                req_id,
                peer_id: _,
                ts: _,
            }
            | Self::InvitationRequest {
                req_id,
                invitation_message_id: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts: _,
            } => Uuid::from_bytes(*req_id),
            Self::InvitationResponse {
                req_id,
                invitation_message_id: _,
                inviter_id: _,
                invited_id: _,
                ts: _,
                invitation_accepted: _,
            } => Uuid::from_bytes(*req_id),
        }
    }
    /// Should be not the current installed user
    pub fn get_peer_id(&self) -> Uuid {
        match self {
            Self::ContactRequest {
                req_id: _,
                peer_id,
                ts: _,
            } => Uuid::from_bytes(*peer_id),
            Self::InvitationRequest {
                req_id: _,
                invitation_message_id: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id,
                ts: _,
            } => Uuid::from_bytes(*invitee_id),
            Self::InvitationResponse {
                req_id: _,
                invitation_message_id: _,
                inviter_id,
                invited_id: _,
                ts: _,
                invitation_accepted: _,
            } => Uuid::from_bytes(*inviter_id),
        }
    }
}

pub struct StreamMessageId {
    req_id: Uuid,
}
