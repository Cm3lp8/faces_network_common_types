use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context_ressources::ContextRessourcesMetaDelta;

/// [`DisplayContext`] represents a context for the client. It reflects the state of the db.
/// [`participants`] field represents who is authorized to access the content of this context.
#[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisplayContext {
    id: [u8; 16],
    participants: Vec<[u8; 16]>,
    ressources_delta: Option<ContextRessourcesMetaDelta>,
    version: u64,
    created_at: i64,
    updated_at: i64,
    kind: DisplayContextKind,
}

impl DisplayContext {
    pub fn new_multiple_participants(
        id: Uuid,
        participants: Vec<Uuid>,
        version: u64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into_bytes(),
            participants: participants.into_iter().map(|it| it.into_bytes()).collect(),
            ressources_delta: None,
            version,
            created_at: created_at.timestamp(),
            updated_at: updated_at.timestamp(),
            kind: DisplayContextKind::Conversation,
        }
    }
    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.id)
    }
    pub fn participants(&self) -> Vec<Uuid> {
        self.participants
            .iter()
            .map(|it| Uuid::from_bytes(*it))
            .collect()
    }
    pub fn retain_participants_by_ids(&self, user_ids: &[Uuid]) -> Vec<Uuid> {
        self.participants
            .iter()
            .filter_map(|it| {
                let id = Uuid::from_bytes(*it);
                if user_ids.contains(&id) {
                    None
                } else {
                    Some(id)
                }
            })
            .collect()
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.created_at, 0)
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.updated_at, 0)
    }
    pub fn kind(&self) -> DisplayContextKind {
        self.kind
    }
}

#[derive(
    Encode, Deserialize, Serialize, Decode, Debug, Clone, Copy, FromSql, ToSql, Hash, PartialEq, Eq,
)]
#[postgres(name = "display_context_kind")]
pub enum DisplayContextKind {
    #[postgres(name = "conversation")]
    Conversation,
    #[postgres(name = "solo")]
    Solo,
}
pub mod context_ressources {
    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    /// [`ContextRessourcesMeta`] represents the collection of ressources attached within the
    /// current version of a context. It does not contain the ressources's data.
    #[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ContextRessourcesMetaDelta {
        ressources: Vec<RessourceItem>,
    }
    impl ContextRessourcesMetaDelta {
        pub fn extend_ressources(&mut self, ressources: &[RessourceItem]) {
            self.ressources.extend_from_slice(ressources);
        }
        pub fn iter(&self) -> ContextRessourceIterator<'_> {
            ContextRessourceIterator {
                ressources: &self.ressources,
                index: 0,
            }
        }
    }

    pub struct ContextRessourceIterator<'a> {
        ressources: &'a [RessourceItem],
        index: usize,
    }
    impl<'a> Iterator for ContextRessourceIterator<'a> {
        type Item = &'a RessourceItem;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(item) = self.ressources.get(0) {
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    impl Default for ContextRessourcesMetaDelta {
        fn default() -> Self {
            Self { ressources: vec![] }
        }
    }

    #[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum RessourceItem {
        Animation { id: [u8; 16] },
    }

    impl RessourceItem {
        pub fn new_animation_ressource(animation_id: Uuid) -> Self {
            Self::Animation {
                id: animation_id.into_bytes(),
            }
        }
        pub fn get_ressource_id(&self) -> Uuid {
            match self {
                Self::Animation { id, .. } => Uuid::from_bytes(*id),
            }
        }
    }
}
