use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncNewAnim {
    anim_id: Uuid,
    author_id: Uuid,
    data: Vec<u8>,
    fps: usize,
    frame_width: u32,
    frame_height: u32,
    jwt: String,
    animation_context_id: Uuid,
    creation_context_kind: String,
}
