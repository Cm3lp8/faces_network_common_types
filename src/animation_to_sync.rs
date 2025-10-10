use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::UserContextKind;

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
    creation_context_kind: UserContextKind,
}
impl SyncNewAnim {
    pub fn new(
        anim_id: Uuid,
        author_id: Uuid,
        data: Vec<u8>,
        fps: usize,
        frame_width: u32,
        frame_height: u32,
        jwt: String,
        animation_context_id: Uuid,
        creation_context_kind: UserContextKind,
    ) -> Self {
        Self {
            anim_id,
            author_id,
            data,
            fps,
            frame_width,
            frame_height,
            jwt,
            animation_context_id,
            creation_context_kind,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncNewAnimResponse {
    code: usize,
}
impl SyncNewAnimResponse {
    pub fn new_with_code(code: usize) -> Self {
        Self { code }
    }
}
