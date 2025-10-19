use bincode::{
    BorrowDecode, Decode, Encode,
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MyUuid(Uuid);
use crate::UserContextKind;
/*
impl Encode for MyUuid {
    fn encode<E: Encoder>(&self, enc: &mut E) -> Result<(), EncodeError> {
        self.0.as_bytes().encode(enc)
    }
}
impl<Context> Decode<Context> for MyUuid {
    fn decode<D: Decoder<Context = Context>>(dec: &mut D) -> Result<Self, DecodeError> {
        let bytes: [u8; 16] = <[u8; 16] as Decode<Context>>::decode(dec)?;
        Ok(MyUuid(Uuid::from_bytes(bytes)))
    }
}
impl BorrowDecode<'de, Context>: Sized for MyUuid {

}

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
/// [`SyncNewAnim`] represente an Animation. It is used when syncing an animation with distant db.
*/
/// As for all animation representation, x and u pos are normalized. (0.0 > 1.0)

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct FetchAnimationToSyncWithItsContext {
    anim_raw_bytes: Vec<u8>,
    anim_variable_context: AnimVariableContext,
}

impl FetchAnimationToSyncWithItsContext {
    pub fn new(anim_raw_bytes: Vec<u8>, anim_variable_context: AnimVariableContext) -> Self {
        Self {
            anim_raw_bytes,
            anim_variable_context,
        }
    }
    pub fn animation_raw_bytes(&self) -> &[u8] {
        &self.anim_raw_bytes
    }
    pub fn animation_variable_context(&self) -> AnimVariableContext {
        self.anim_variable_context.clone()
    }
}

// [u8;16 are uuid]
#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct SyncNewAnim {
    anim_id: [u8; 16],
    author_id: [u8; 16],
    data: Vec<u8>,
    fps: usize,
    frame_width: u32,
    frame_height: u32,
    jwt: String, // TODO not the place of the jwt
    animation_context_id: [u8; 16],
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
            anim_id: anim_id.into_bytes(),
            author_id: author_id.into_bytes(),
            data,
            fps,
            frame_width,
            frame_height,
            jwt,
            animation_context_id: animation_context_id.into_bytes(),
            creation_context_kind,
            x_pos,
            y_pos,
        }
    }

    pub fn anim_id(&self) -> Uuid {
        Uuid::from_bytes(self.anim_id)
    }
    pub fn pos(&self) -> (f32, f32) {
        (self.x_pos, self.y_pos)
    }
    pub fn author_id(&self) -> Uuid {
        Uuid::from_bytes(self.author_id)
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..]
    }
    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.animation_context_id)
    }
    pub fn frame_width(&self) -> u32 {
        self.frame_width
    }
    pub fn frame_height(&self) -> u32 {
        self.frame_height
    }
    pub fn fps(&self) -> usize {
        self.fps
    }
    pub fn take_data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AnimVariableContext {
    anim_id: [u8; 16],
    context_id: [u8; 16],
    context_version: u64,
    variable_context_version: u64,
    //context_kind:
    x_pos: f32,
    y_pos: f32,
}
impl AnimVariableContext {
    pub fn new(
        anim_id: Uuid,
        context_id: Uuid,
        context_version: u64,
        variable_context_version: u64,
        x_pos: f32,
        y_pos: f32,
    ) -> Self {
        Self {
            anim_id: anim_id.into_bytes(),
            context_id: context_id.into_bytes(),
            context_version,
            variable_context_version,
            x_pos,
            y_pos,
        }
    }
    pub fn pos(&self) -> [f32; 2] {
        [self.x_pos, self.y_pos]
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

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct AnimationRefToFetch {
    anim_id: [u8; 16],
    context_id: [u8; 16],
    user_id: [u8; 16],
}
impl AnimationRefToFetch {
    pub fn new(anim_id: Uuid, context_id: Uuid, user_id: Uuid) -> Self {
        Self {
            anim_id: anim_id.into_bytes(),
            context_id: context_id.into_bytes(),
            user_id: user_id.into_bytes(),
        }
    }
    pub fn anim_id(&self) -> Uuid {
        Uuid::from_bytes(self.anim_id)
    }
    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }
    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.user_id)
    }
}
