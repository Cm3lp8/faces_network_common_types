use std::ops::Deref;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ressources_descriptors::ressources_descriptors_kind::AnimationRessource;

/// [`RessourcesDescritors`] represents a collection of ressources a client needs to fetch from the
/// server
#[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RessourcesDescriptors {
    ressources_collection: Vec<RessourcesDescriptorsKind>,
}
impl RessourcesDescriptors {
    pub fn new_empty() -> Self {
        Self {
            ressources_collection: vec![],
        }
    }
    pub fn add_ressource_descriptor(&mut self, descriptors: RessourcesDescriptorsKind) {
        self.ressources_collection.push(descriptors);
    }
    pub fn extend_ressource_descriptor(&mut self, descriptors: &[RessourcesDescriptorsKind]) {
        self.ressources_collection.extend_from_slice(descriptors);
    }
    pub fn iter(&self) -> RessourcesDescriptorsIterator<'_> {
        RessourcesDescriptorsIterator {
            items: &self.ressources_collection,
            index: 0,
        }
    }
}
impl<'a> From<BorrowedRessourcesDescriptorsKind<'a>> for RessourcesDescriptorsKind {
    fn from(value: BorrowedRessourcesDescriptorsKind<'a>) -> Self {
        match value.0 {
            RessourcesDescriptorsKind::Animation(anim_resources) => {
                Self::Animation(anim_resources.clone())
            }
        }
    }
}

pub struct RessourcesDescriptorsIterator<'a> {
    items: &'a Vec<RessourcesDescriptorsKind>,
    index: usize,
}

impl<'a> Iterator for RessourcesDescriptorsIterator<'a> {
    type Item = BorrowedRessourcesDescriptorsKind<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.items.get(self.index) {
            self.index += 1;
            Some(BorrowedRessourcesDescriptorsKind(item))
        } else {
            None
        }
    }
}
impl<'a> Deref for BorrowedRessourcesDescriptorsKind<'a> {
    type Target = RessourcesDescriptorsKind;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
pub struct BorrowedRessourcesDescriptorsKind<'a>(&'a RessourcesDescriptorsKind);
#[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RessourcesDescriptorsKind {
    Animation(AnimationRessource),
}
impl RessourcesDescriptorsKind {
    pub fn new_animation_descriptor(animation_id: Uuid) -> Self {
        Self::Animation(AnimationRessource::new(animation_id))
    }
    pub fn get_id(&self) -> Uuid {
        match self {
            Self::Animation(animation_desc) => animation_desc.get_id(),
        }
    }
}

mod ressources_descriptors_kind {
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AnimationRessource {
        ressource_id: [u8; 16],
    }
    impl AnimationRessource {
        pub fn new(ressource_id: Uuid) -> Self {
            Self {
                ressource_id: ressource_id.into_bytes(),
            }
        }
        pub fn get_id(&self) -> Uuid {
            Uuid::from_bytes(self.ressource_id)
        }
    }
}
