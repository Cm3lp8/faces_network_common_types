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
    animation_delta: Vec<[u8; 16]>,
}
impl PulledContextVersionWithAnimationDelta {
    pub fn new(context_id: Uuid, animations_delta: &[Uuid]) -> Self {
        Self {
            context_id: context_id.into_bytes(),
            animation_delta: animations_delta.iter().map(|it| it.into_bytes()).collect(),
        }
    }

    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }

    pub fn get_animation_delta_ids_coll(&self) -> Vec<Uuid> {
        self.animation_delta
            .iter()
            .map(|i| Uuid::from_bytes(*i))
            .collect()
    }
}
