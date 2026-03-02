use uuid::Uuid;

use crate::context_ressources::RessourceItem;

pub struct CompositionData {
    id: [u8; 16],
    author_id: [u8; 16],
    width: u32,
    height: u32,
    min_x: f32,
    max_x: f32,

    min_y: f32,
    max_y: f32,
}

impl CompositionData {
    pub fn new(
        id: Uuid,
        author_id: Uuid,
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
