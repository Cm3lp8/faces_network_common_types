use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context_ressources::ContextRessourcesMetaDelta;

/// [`DisplayContext`] represents a context for the client. It reflects the state of the db.
/// [`participants`] field represents who is authorized to access the content of this context.
#[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq)]
pub struct DisplayContext {
    id: [u8; 16],
    participants: Vec<[u8; 16]>,
    ressources_delta: Option<ContextRessourcesMetaDelta>,
    user_session_version: u64,
    context_version: u64,
    created_at: i64,
    updated_at: i64,
    kind: DisplayContextKind,
}

impl DisplayContext {
    pub fn new_multiple_participants(
        id: Uuid,
        participants: Vec<Uuid>,
        user_session_version: u64,
        context_version: u64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into_bytes(),
            participants: participants.into_iter().map(|it| it.into_bytes()).collect(),
            ressources_delta: None,
            user_session_version,
            context_version,
            created_at: created_at.timestamp(),
            updated_at: updated_at.timestamp(),
            kind: DisplayContextKind::Conversation,
        }
    }
    pub fn new(
        id: Uuid,
        kind: DisplayContextKind,
        participants: Vec<Uuid>,
        user_session_version: u64,
        context_version: u64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into_bytes(),
            participants: participants.into_iter().map(|it| it.into_bytes()).collect(),
            ressources_delta: None,
            user_session_version,
            context_version,
            created_at: created_at.timestamp(),
            updated_at: updated_at.timestamp(),
            kind,
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
    pub fn user_session_version(&self) -> u64 {
        self.user_session_version
    }
    pub fn context_version(&self) -> u64 {
        self.context_version
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
    use std::collections::{HashMap, HashSet};

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    type CompositionId = [u8; 16];
    /// [`ContextRessourcesMeta`] represents the collection of ressources attached within the
    /// current version of a context. It does not contain the ressources's data.
    #[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq)]
    pub struct ContextRessourcesMetaDelta {
        composition_id_set: HashSet<CompositionId>,
        ressources_by_composition: HashMap<CompositionId, Vec<RessourceItem>>,
    }
    impl ContextRessourcesMetaDelta {
        pub fn get_compositions_set(&self) -> Vec<Uuid> {
            self.composition_id_set
                .iter()
                .map(|it| Uuid::from_bytes(*it))
                .collect()
        }
        /// Flatten the map
        pub fn get_all_resources_items(&self) -> Vec<&RessourceItem> {
            self.ressources_by_composition.values().flatten().collect()
        }
        pub fn extend_ressources(&mut self, composition_id: Uuid, ressources: &[RessourceItem]) {
            self.composition_id_set.insert(composition_id.into_bytes());
            self.ressources_by_composition
                .entry(composition_id.into_bytes())
                .or_insert_with(Vec::new)
                .extend_from_slice(ressources);
        }
        pub fn iter_by_composition(
            &self,
            composition_id: Uuid,
        ) -> Option<ContextRessourceIterator<'_>> {
            let Some(r) = self
                .ressources_by_composition
                .get(&composition_id.into_bytes())
            else {
                return None;
            };
            Some(ContextRessourceIterator {
                ressources: &r,
                index: 0,
            })
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
            Self {
                composition_id_set: HashSet::new(),
                ressources_by_composition: HashMap::new(),
            }
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
