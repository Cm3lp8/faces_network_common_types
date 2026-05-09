use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct FragmentTransform2DData {
    pos: [f32; 3],
    dimensions: [f32; 2],
    scale: [f32; 2],
    rot: f32,
    z: f32,
}

impl FragmentTransform2DData {
    pub fn new(pos: [f32; 3], dimensions: [f32; 2], scale: [f32; 2], rot: f32, z: f32) -> Self {
        Self {
            pos,
            dimensions,
            scale,
            rot,
            z,
        }
    }
    pub fn from_pos_size(pos: [f32; 2], width: f32, height: f32) -> Self {
        Self {
            pos: [pos[0], pos[1], 0.0],
            dimensions: [width, height],
            scale: [1.0, 1.0],
            rot: 0.0,
            z: 0.0,
        }
    }
    pub fn pos(&self) -> [f32; 3] {
        self.pos
    }
    pub fn dimensions(&self) -> [f32; 2] {
        self.dimensions
    }
    pub fn scale(&self) -> [f32; 2] {
        self.scale
    }
    pub fn rot(&self) -> f32 {
        self.rot
    }
    pub fn z(&self) -> f32 {
        self.z
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionResourceData {
    id: [u8; 16],
    transform: FragmentTransform2DData,
}
impl CompositionResourceData {
    pub fn new(id: Uuid, pos_world_coord: [f32; 2], width: f32, height: f32) -> Self {
        Self {
            id: id.into_bytes(),
            transform: FragmentTransform2DData::from_pos_size(pos_world_coord, width, height),
        }
    }
    pub fn new_with_transform(id: Uuid, transform: FragmentTransform2DData) -> Self {
        Self {
            id: id.into_bytes(),
            transform,
        }
    }
    pub fn id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
    pub fn width(&self) -> f32 {
        self.transform.dimensions[0]
    }
    pub fn height(&self) -> f32 {
        self.transform.dimensions[1]
    }
    pub fn transform(&self) -> &FragmentTransform2DData {
        &self.transform
    }
    pub fn pos_world_coord(&self) -> [f32; 2] {
        [self.transform.pos[0], self.transform.pos[1]]
    }
    pub fn pos_screen_coord(&self) -> [f32; 2] {
        self.pos_world_coord()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionData {
    id: [u8; 16],
    author_id: [u8; 16],
    resource_collection: Vec<CompositionResourceData>,
    width: f32,
    height: f32,
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
        width: f32,
        height: f32,
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
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
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
