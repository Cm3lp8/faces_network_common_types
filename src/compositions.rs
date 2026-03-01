use uuid::Uuid;

use crate::context_ressources::RessourceItem;

pub struct Composition {
    id: [u8; 16],
    author_id: [u8; 16],
    /// Dimensions in pixel of the bounding box
    size: [f32; 2],
    normalized_pos: [f32; 2],
    ///
    resource_ids_set: Vec<[u8; 16]>,
}

impl Composition {
    pub fn new(id: Uuid, author_id: Uuid) -> CompositionBuilder {
        CompositionBuilder {
            resources_set: vec![]
        }
    }
    pub fn id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
}

pub struct CompositionBuilder {
    resources_set: Vec<RessourceDimData>,
}

pub struct RessourceDimData {
    size: [f32; 2],
    screen_pos: [f32; 2],
}
