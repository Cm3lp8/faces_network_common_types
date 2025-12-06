use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{RessourcesDescriptors, ressources_descriptors};

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Clone, Eq)]
/// [`ServerContextVersion`] represents two version counter states from the server.
/// It shows the actual server session version for the user and a specific context version
pub struct ServerContextVersion {
    context_id: Uuid,
    context_version: u64,
    user_session_version: u64,
}

impl ServerContextVersion {
    pub fn new(context_id: Uuid, context_version: u64, user_session_version: u64) -> Self {
        Self {
            context_id,

            context_version,
            user_session_version,
        }
    }
    pub fn context_id(&self) -> Uuid {
        self.context_id
    }
    pub fn context_version(&self) -> u64 {
        self.context_version
    }
    pub fn user_session_version(&self) -> u64 {
        self.user_session_version
    }
}

// Fetched current server version for this user
#[derive(Debug, Serialize, Encode, Decode, Deserialize, Hash, PartialEq, Clone, Eq)]
pub struct PushedUserSessionDeltasWithRessourceDescriptors {
    user_id: [u8; 16],
    current_user_session_version: u64,
    current_user_context_versions: Vec<([u8; 16], u64)>,
    ressources_descriptors: RessourcesDescriptors,
    has_more_ressources: bool,
}

impl PushedUserSessionDeltasWithRessourceDescriptors {
    pub fn new(
        user_id: Uuid,
        current_user_session_version: u64,
        current_user_context_versions: Vec<(Uuid, u64)>,
        ressources_descriptors: RessourcesDescriptors,
        has_more_ressources: bool,
    ) -> Self {
        Self {
            user_id: user_id.into_bytes(),
            current_user_session_version,
            current_user_context_versions: current_user_context_versions
                .iter()
                .map(|it| (it.0.into_bytes(), it.1))
                .collect(),
            ressources_descriptors,
            has_more_ressources,
        }
    }
    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.user_id)
    }
    pub fn current_user_session_version(&self) -> u64 {
        self.current_user_session_version
    }
    pub fn current_user_context_versions(&self) -> Vec<(Uuid, u64)> {
        self.current_user_context_versions
            .iter()
            .map(|it| (Uuid::from_bytes(it.0), it.1))
            .collect()
    }
    pub fn ressources_descriptors(&self) -> &RessourcesDescriptors {
        &self.ressources_descriptors
    }
}

#[derive(Debug, Serialize, Encode, Decode, Deserialize, Hash, PartialEq, Clone, Eq)]
pub struct LastPulledUserSessionVersionAndContextVersions {
    user_id: [u8; 16],
    last_pulled_user_session_version: u64,
    contexts: Vec<([u8; 16], u64)>,
    max_descriptor_amount: usize,
}
impl LastPulledUserSessionVersionAndContextVersions {
    pub fn new(
        user_id: Uuid,
        last_pulled_user_session_version: u64,
        contexts: Vec<(Uuid, u64)>,
        max_descriptor_amount: usize,
    ) -> Self {
        Self {
            user_id: user_id.into_bytes(),
            last_pulled_user_session_version,
            contexts: contexts
                .into_iter()
                .map(|it| (it.0.into_bytes(), it.1))
                .collect(),
            max_descriptor_amount,
        }
    }
    pub fn user_id(&self) -> Uuid {
        Uuid::from_bytes(self.user_id)
    }
    pub fn last_pulled_user_session_version(&self) -> u64 {
        self.last_pulled_user_session_version
    }

    pub fn max_descriptor_amount(&self) -> usize {
        self.max_descriptor_amount
    }
    pub fn context_with_last_version(&self) -> Vec<(Uuid, u64)> {
        self.contexts
            .iter()
            .map(|it| (Uuid::from_bytes(it.0), it.1))
            .collect()
    }
}
