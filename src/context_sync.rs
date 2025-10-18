use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct LastClientContextVersion {
    context_id: [u8; 16], // UUid
    context_version: u64,
}
impl LastClientContextVersion {
    pub fn new(context_id: Uuid, context_version: u64) -> Self {
        Self {
            context_id: context_id.into_bytes(),
            context_version,
        }
    }

    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }

    pub fn context_version(&self) -> u64 {
        self.context_version
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub struct PulledContextVersionWithAnimationDelta {
    context_id: [u8; 16], // UUid
    animation_delta: AnimationDeltaCollection,
}
impl PulledContextVersionWithAnimationDelta {
    pub fn new(context_id: Uuid, animations_delta: Option<&[Uuid]>) -> Self {
        let animation_delta: AnimationDeltaCollection = match animations_delta {
            Some(coll) => AnimationDeltaCollection::AnimationDelta {
                ids_collection: coll.iter().map(|it| it.into_bytes()).collect(),
            },
            None => AnimationDeltaCollection::UptoDate,
        };
        Self {
            context_id: context_id.into_bytes(),
            animation_delta,
        }
    }

    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }

    pub fn get_animation_delta_ids_coll(&self) -> Option<Vec<Uuid>> {
        let AnimationDeltaCollection::AnimationDelta { ids_collection } = &self.animation_delta
        else {
            return None;
        };
        Some(
            ids_collection
                .iter()
                .map(|i| Uuid::from_bytes(*i))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Decode, Encode)]
pub enum AnimationDeltaCollection {
    UptoDate,
    AnimationDelta { ids_collection: Vec<[u8; 16]> },
}
