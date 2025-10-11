use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::UserContextKind;

/// [`SyncNewAnim`] represente an Animation. It is used when syncing an animation with distant db.
/// As for all animation representation, x and u pos are normalized. (0.0 > 1.0)
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
    x_pos: f32,
    y_pos: f32,
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
        x_pos: f32,
        y_pos: f32,
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
            x_pos,
            y_pos,
        }
    }

    pub fn anim_id(&self) -> Uuid {
        self.anim_id
    }
    pub fn pos(&self) -> (f32, f32) {
        (self.x_pos, self.y_pos)
    }
    pub fn author_id(&self) -> Uuid {
        self.author_id
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..]
    }
    pub fn context_id(&self) -> Uuid {
        self.animation_context_id
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
