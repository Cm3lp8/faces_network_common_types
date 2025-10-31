use bincode::{Decode, Encode};
use uuid::Uuid;

#[derive(Decode, Encode)]
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
