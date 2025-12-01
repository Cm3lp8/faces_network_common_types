use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DisplayContext, InvitationOrientation};

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
        inviter_friend_reg_version: Option<u64>,
        invitee_friend_reg_version: Option<u64>,
        invitation_accepted: bool,
        first_common_display_context: Option<DisplayContext>,
        ts: i64, // utc timestamp
    },

    RemovedNotification {
        req_id: [u8; 16],
        peer_id: [u8; 16],
        removed_notification_id: [u8; 16],
    },
    ContactRequest {
        req_id: [u8; 16],
        peer_id: [u8; 16],
        ts: i64,
    },
    NewSessionVersionAvailable {
        req_id: [u8; 16],
        peer_id: [u8; 16],
    },
}

impl StreamMessage {
    pub fn new_session_version_available(peer_id: Uuid) -> Self {
        Self::NewSessionVersionAvailable {
            req_id: Uuid::now_v7().into_bytes(),
            peer_id: peer_id.into_bytes(),
        }
    }
    pub fn new_removed_notification(peer_id: Uuid, removed_notification_id: Uuid) -> Self {
        Self::RemovedNotification {
            req_id: Uuid::now_v7().into_bytes(),
            peer_id: peer_id.into_bytes(),
            removed_notification_id: removed_notification_id.into_bytes(),
        }
    }
    pub fn new_invitation_response_confirmation(
        invitation_message_id: Uuid,
        inviter_id: Uuid,
        invited_id: Uuid,
        first_common_display_context: DisplayContext,
        invitee_friend_reg_version: Option<u64>,
        inviter_friend_reg_version: Option<u64>,
    ) -> Self {
        let req_id = Uuid::now_v7();
        Self::InvitationResponse {
            req_id: req_id.into_bytes(),
            invitation_message_id: invitation_message_id.into_bytes(),
            inviter_id: inviter_id.into_bytes(),
            invited_id: invited_id.into_bytes(),
            inviter_friend_reg_version,
            invitee_friend_reg_version,
            invitation_accepted: true,
            first_common_display_context: Some(first_common_display_context),

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
            inviter_friend_reg_version: None,
            invitee_friend_reg_version: None,
            invitation_accepted: false,
            first_common_display_context: None,
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
            Self::ContactRequest { req_id, .. } | Self::InvitationRequest { req_id, .. } => {
                Uuid::from_bytes(*req_id)
            }
            Self::InvitationResponse { req_id, .. } => Uuid::from_bytes(*req_id),
            Self::RemovedNotification { req_id, .. } => Uuid::from_bytes(*req_id),
            Self::NewSessionVersionAvailable { req_id, .. } => Uuid::from_bytes(*req_id),
        }
    }
    /// Should be not the current installed user
    pub fn get_peer_id(&self) -> Uuid {
        match self {
            Self::RemovedNotification { peer_id, .. } => Uuid::from_bytes(*peer_id),
            Self::ContactRequest { peer_id, .. } => Uuid::from_bytes(*peer_id),

            Self::InvitationRequest { invitee_id, .. } => Uuid::from_bytes(*invitee_id),
            Self::InvitationResponse { inviter_id, .. } => Uuid::from_bytes(*inviter_id),
            Self::NewSessionVersionAvailable { peer_id, .. } => Uuid::from_bytes(*peer_id),
        }
    }
}

pub struct StreamMessageId {
    req_id: Uuid,
}
