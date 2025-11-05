use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::StreamMessage;

#[derive(Decode, Encode, Debug)]
pub struct PeerInvitationByTextHandle {
    emitting_user_id: [u8; 16],
    peer_username_handle: String,
}
impl PeerInvitationByTextHandle {
    pub fn new(emitting_user_id: Uuid, peer_username_handle: &str) -> Self {
        Self {
            emitting_user_id: emitting_user_id.into_bytes(),
            peer_username_handle: peer_username_handle.to_string(),
        }
    }
    pub fn get_emitting_user_id(&self) -> Uuid {
        Uuid::from_bytes(self.emitting_user_id)
    }
    pub fn get_peer_username_handle(&self) -> &str {
        &self.peer_username_handle
    }
}
pub struct InvitationMessage {
    invitation_id: [u8; 16],
    emitter_id: [u8; 16],
    emitter_name: String,
    receiver_id: [u8; 16],
    ts: DateTime<Utc>,
}
impl InvitationMessage {
    pub fn new(
        invitation_id: Uuid,
        emitter_id: Uuid,
        emitter_name: &str,
        receiver_id: Uuid,
    ) -> Self {
        Self {
            invitation_id: invitation_id.into_bytes(),
            emitter_id: emitter_id.into_bytes(),
            emitter_name: emitter_name.to_string(),
            receiver_id: receiver_id.into_bytes(),
            ts: Utc::now(),
        }
    }
    pub fn dest_id(&self) -> Uuid {
        Uuid::from_bytes(self.receiver_id)
    }
    pub fn emitter_id(&self) -> Uuid {
        Uuid::from_bytes(self.emitter_id)
    }
}

impl From<InvitationMessage> for StreamMessage {
    fn from(value: InvitationMessage) -> Self {
        StreamMessage::InvitationRequest {
            req_id: Uuid::now_v7().into_bytes(),
            invitation_message_id: value.invitation_id,
            emitter_id: value.emitter_id,
            emitter_name: value.emitter_name,
            dest_id: value.receiver_id,
            ts: value.ts.timestamp(),
        }
    }
}
