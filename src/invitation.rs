use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::StreamMessage;

#[derive(Decode, Encode, Debug)]
pub struct InvitationResponse {
    invitation_id: [u8; 16],
    responder_user_id: [u8; 16],
    kind: InvitationResponseKind,
    ts_utc: i64,
}

impl InvitationResponse {
    pub fn new(invitation_uuid: Uuid, user_id: Uuid, kind: InvitationResponseKind) -> Self {
        Self {
            invitation_id: invitation_uuid.into_bytes(),
            responder_user_id: user_id.into_bytes(),
            kind,
            ts_utc: Utc::now().timestamp(),
        }
    }
    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.responder_user_id)
    }
    pub fn invitation_id(&self) -> Uuid {
        Uuid::from_bytes(self.invitation_id)
    }
    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.ts_utc, 0)
    }
    pub fn invitation_response_kind(&self) -> InvitationResponseKind {
        self.kind
    }
}

#[derive(Decode, Encode, Debug, Clone, Copy)]
pub enum InvitationResponseKind {
    Accepted,
    Refused,
}

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

#[derive(Serialize, Debug, Deserialize, Encode, Decode, Clone, PartialEq, Eq)]
pub enum InvitationOrientation {
    AsSender,
    AsReceiver,
}
pub struct InvitationMessage {
    invitation_orientation: InvitationOrientation,
    invitation_id: [u8; 16],
    emitter_id: [u8; 16],
    emitter_name: String,
    receiver_id: [u8; 16],
    ts: DateTime<Utc>,
}
impl InvitationMessage {
    pub fn new_for_sender(
        invitation_id: Uuid,
        emitter_id: Uuid,
        emitter_name: &str,
        receiver_id: Uuid,
    ) -> Self {
        Self {
            invitation_orientation: InvitationOrientation::AsSender,
            invitation_id: invitation_id.into_bytes(),
            emitter_id: emitter_id.into_bytes(),
            emitter_name: emitter_name.to_string(),
            receiver_id: receiver_id.into_bytes(),
            ts: Utc::now(),
        }
    }
    pub fn new_for_receiver(
        invitation_id: Uuid,
        emitter_id: Uuid,
        emitter_name: &str,
        receiver_id: Uuid,
    ) -> Self {
        Self {
            invitation_orientation: InvitationOrientation::AsSender,
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
            invitation_orientation: value.invitation_orientation,
            req_id: Uuid::now_v7().into_bytes(),
            invitation_message_id: value.invitation_id,
            emitter_id: value.emitter_id,
            emitter_name: value.emitter_name,
            dest_id: value.receiver_id,
            ts: value.ts.timestamp(),
        }
    }
}

#[cfg(test)]
mod invitation_test {
    use bincode::{Decode, config::standard};
    use uuid::Uuid;

    use crate::{InvitationResponse, InvitationResponseKind};

    #[test]
    fn invitation_response_end_to_end() {
        let user = Uuid::now_v7();
        let invitation_uuid = Uuid::now_v7();
        let new_invitation =
            InvitationResponse::new(invitation_uuid, user, InvitationResponseKind::Accepted);

        let Ok(encoded) = bincode::encode_to_vec(new_invitation, bincode::config::standard())
        else {
            assert!(false);
            return;
        };

        let Ok((decoded, _)) = bincode::decode_from_slice::<InvitationResponse, _>(
            &encoded,
            bincode::config::standard(),
        ) else {
            assert!(false);
            return;
        };

        let decoded_uuid = decoded.user_id();

        assert!(user == decoded_uuid)
    }
}
