use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionResourceData {
    id: [u8; 16],
    pos_screen_coord: [f32; 2],
    width: f32,
    height: f32,
}
impl CompositionResourceData {
    pub fn new(id: Uuid, pos_screen_coord: [f32; 2], width: f32, height: f32) -> Self {
        Self {
            id: id.into_bytes(),
            pos_screen_coord,
            width,
            height,
        }
    }
    pub fn id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn pos_screen_coord(&self) -> [f32; 2] {
        self.pos_screen_coord
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionData {
    id: [u8; 16],
    author_id: [u8; 16],
    resource_collection: Vec<CompositionResourceData>,
    width: u32,
    height: u32,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl PartialEq for CompositionData {
    fn eq(&self, other: &Self) -> bool {
        if self.id == other.id { true } else { false }
    }
}
impl Eq for CompositionData {}

impl CompositionData {
    pub fn new(
        id: Uuid,
        author_id: Uuid,
        resource_collection: Vec<CompositionResourceData>,
        width: u32,
        height: u32,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
    ) -> Self {
        Self {
            id: id.into_bytes(),
            author_id: author_id.into_bytes(),
            resource_collection,
            width,
            height,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
    pub fn author_id(&self) -> Uuid {
        Uuid::from_bytes(self.author_id)
    }
    pub fn resources_collection(&self) -> &[CompositionResourceData] {
        self.resource_collection.as_slice()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn min_x(&self) -> f32 {
        self.min_x
    }
    pub fn max_x(&self) -> f32 {
        self.max_x
    }
    pub fn min_y(&self) -> f32 {
        self.min_y
    }
    pub fn max_y(&self) -> f32 {
        self.max_y
    }
}
