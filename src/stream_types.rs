use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserStreamSessionInfo {
    user_id: Uuid,
}

// an Id field to make possible request confirmation when client responds
pub enum StreamMessage {
    ContactRequest { req_id: Uuid, peer_id: Uuid },
}

impl StreamMessage {
    pub fn new_contact_request(peer_id: Uuid) -> (StreamMessageId, Self) {
        let req_id = Uuid::now_v7();
        (
            StreamMessageId { req_id },
            Self::ContactRequest {
                req_id,
                peer_id: peer_id,
            },
        )
    }
}

pub struct StreamMessageId {
    req_id: Uuid,
}
