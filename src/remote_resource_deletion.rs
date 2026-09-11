use std::{collections::HashSet, error::Error, fmt};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wire version for the durable remote-resource deletion protocol.
pub const REMOTE_RESOURCE_DELETION_VERSION_V1: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoteResourceDeletionContractError {
    UnsupportedVersion { expected: u16, actual: u16 },
    NilUuid { field: &'static str },
    EmptyResourceList,
    DuplicateResourceId { resource_id: Uuid },
    EmptyAcknowledgement,
    CorrelationMismatch { field: &'static str },
    AcknowledgedResourceSetMismatch,
}

impl fmt::Display for RemoteResourceDeletionContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion { expected, actual } => write!(
                formatter,
                "unsupported remote-resource deletion version {actual}; expected {expected}"
            ),
            Self::NilUuid { field } => {
                write!(formatter, "remote-resource deletion field {field} is nil")
            }
            Self::EmptyResourceList => {
                write!(formatter, "remote-resource deletion list is empty")
            }
            Self::DuplicateResourceId { resource_id } => write!(
                formatter,
                "remote-resource deletion contains duplicate resource {resource_id}"
            ),
            Self::EmptyAcknowledgement => {
                write!(
                    formatter,
                    "remote-resource deletion acknowledgement is empty"
                )
            }
            Self::CorrelationMismatch { field } => write!(
                formatter,
                "remote-resource deletion acknowledgement mismatches request field {field}"
            ),
            Self::AcknowledgedResourceSetMismatch => write!(
                formatter,
                "remote-resource deletion acknowledgement does not cover the request resource set"
            ),
        }
    }
}

impl Error for RemoteResourceDeletionContractError {}

fn validate_version(version: u16) -> Result<(), RemoteResourceDeletionContractError> {
    if version != REMOTE_RESOURCE_DELETION_VERSION_V1 {
        return Err(RemoteResourceDeletionContractError::UnsupportedVersion {
            expected: REMOTE_RESOURCE_DELETION_VERSION_V1,
            actual: version,
        });
    }
    Ok(())
}

fn validate_uuid(
    value: Uuid,
    field: &'static str,
) -> Result<(), RemoteResourceDeletionContractError> {
    if value.is_nil() {
        return Err(RemoteResourceDeletionContractError::NilUuid { field });
    }
    Ok(())
}

fn decode_uuid(value: [u8; 16]) -> Uuid {
    Uuid::from_bytes(value)
}

fn decode_uuid_list(values: &[[u8; 16]]) -> Vec<Uuid> {
    values.iter().copied().map(decode_uuid).collect()
}

fn validate_resource_ids(
    values: &[[u8; 16]],
    empty_error: RemoteResourceDeletionContractError,
) -> Result<HashSet<Uuid>, RemoteResourceDeletionContractError> {
    if values.is_empty() {
        return Err(empty_error);
    }

    let mut unique = HashSet::with_capacity(values.len());
    for raw_resource_id in values {
        let resource_id = decode_uuid(*raw_resource_id);
        validate_uuid(resource_id, "resource_id")?;
        if !unique.insert(resource_id) {
            return Err(RemoteResourceDeletionContractError::DuplicateResourceId { resource_id });
        }
    }
    Ok(unique)
}

/// Versioned request to unlink and, when no other composition references them, delete resources.
///
/// `user_id` is the durable owner recorded with the local outbox entry. Servers must still derive
/// authorization from the authenticated transport and compare it with this value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct RemoteResourceDeletionRequestV1 {
    version: u16,
    request_id: [u8; 16],
    user_id: [u8; 16],
    composition_id: [u8; 16],
    context_id: [u8; 16],
    resource_ids: Vec<[u8; 16]>,
}

impl RemoteResourceDeletionRequestV1 {
    pub fn new(
        request_id: Uuid,
        user_id: Uuid,
        composition_id: Uuid,
        context_id: Uuid,
        resource_ids: Vec<Uuid>,
    ) -> Result<Self, RemoteResourceDeletionContractError> {
        let request = Self {
            version: REMOTE_RESOURCE_DELETION_VERSION_V1,
            request_id: request_id.into_bytes(),
            user_id: user_id.into_bytes(),
            composition_id: composition_id.into_bytes(),
            context_id: context_id.into_bytes(),
            resource_ids: resource_ids.into_iter().map(Uuid::into_bytes).collect(),
        };
        request.validate()?;
        Ok(request)
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub fn request_id(&self) -> Uuid {
        decode_uuid(self.request_id)
    }

    pub fn user_id(&self) -> Uuid {
        decode_uuid(self.user_id)
    }

    pub fn composition_id(&self) -> Uuid {
        decode_uuid(self.composition_id)
    }

    pub fn context_id(&self) -> Uuid {
        decode_uuid(self.context_id)
    }

    pub fn resource_ids(&self) -> Vec<Uuid> {
        decode_uuid_list(&self.resource_ids)
    }

    pub fn validate(&self) -> Result<(), RemoteResourceDeletionContractError> {
        validate_version(self.version)?;
        validate_uuid(self.request_id(), "request_id")?;
        validate_uuid(self.user_id(), "user_id")?;
        validate_uuid(self.composition_id(), "composition_id")?;
        validate_uuid(self.context_id(), "context_id")?;
        validate_resource_ids(
            &self.resource_ids,
            RemoteResourceDeletionContractError::EmptyResourceList,
        )?;
        Ok(())
    }
}

/// Positive acknowledgement for a [`RemoteResourceDeletionRequestV1`].
///
/// `deleted_or_absent_resource_ids` includes resources deleted by this request and resources that
/// were already absent during an idempotent retry. `retained_shared_resource_ids` includes
/// resources unlinked from the requested composition but retained because another composition
/// still references them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct RemoteResourceDeletionAckV1 {
    version: u16,
    request_id: [u8; 16],
    user_id: [u8; 16],
    composition_id: [u8; 16],
    context_id: [u8; 16],
    deleted_or_absent_resource_ids: Vec<[u8; 16]>,
    retained_shared_resource_ids: Vec<[u8; 16]>,
}

impl RemoteResourceDeletionAckV1 {
    pub fn new(
        request_id: Uuid,
        user_id: Uuid,
        composition_id: Uuid,
        context_id: Uuid,
        deleted_or_absent_resource_ids: Vec<Uuid>,
        retained_shared_resource_ids: Vec<Uuid>,
    ) -> Result<Self, RemoteResourceDeletionContractError> {
        let acknowledgement = Self {
            version: REMOTE_RESOURCE_DELETION_VERSION_V1,
            request_id: request_id.into_bytes(),
            user_id: user_id.into_bytes(),
            composition_id: composition_id.into_bytes(),
            context_id: context_id.into_bytes(),
            deleted_or_absent_resource_ids: deleted_or_absent_resource_ids
                .into_iter()
                .map(Uuid::into_bytes)
                .collect(),
            retained_shared_resource_ids: retained_shared_resource_ids
                .into_iter()
                .map(Uuid::into_bytes)
                .collect(),
        };
        acknowledgement.validate()?;
        Ok(acknowledgement)
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub fn request_id(&self) -> Uuid {
        decode_uuid(self.request_id)
    }

    pub fn user_id(&self) -> Uuid {
        decode_uuid(self.user_id)
    }

    pub fn composition_id(&self) -> Uuid {
        decode_uuid(self.composition_id)
    }

    pub fn context_id(&self) -> Uuid {
        decode_uuid(self.context_id)
    }

    pub fn deleted_or_absent_resource_ids(&self) -> Vec<Uuid> {
        decode_uuid_list(&self.deleted_or_absent_resource_ids)
    }

    pub fn retained_shared_resource_ids(&self) -> Vec<Uuid> {
        decode_uuid_list(&self.retained_shared_resource_ids)
    }

    pub fn processed_resource_ids(&self) -> Vec<Uuid> {
        self.deleted_or_absent_resource_ids()
            .into_iter()
            .chain(self.retained_shared_resource_ids())
            .collect()
    }

    pub fn validate(&self) -> Result<(), RemoteResourceDeletionContractError> {
        validate_version(self.version)?;
        validate_uuid(self.request_id(), "request_id")?;
        validate_uuid(self.user_id(), "user_id")?;
        validate_uuid(self.composition_id(), "composition_id")?;
        validate_uuid(self.context_id(), "context_id")?;

        if self.deleted_or_absent_resource_ids.is_empty()
            && self.retained_shared_resource_ids.is_empty()
        {
            return Err(RemoteResourceDeletionContractError::EmptyAcknowledgement);
        }

        let mut processed = HashSet::with_capacity(
            self.deleted_or_absent_resource_ids.len() + self.retained_shared_resource_ids.len(),
        );
        for raw_resource_id in self
            .deleted_or_absent_resource_ids
            .iter()
            .chain(self.retained_shared_resource_ids.iter())
        {
            let resource_id = decode_uuid(*raw_resource_id);
            validate_uuid(resource_id, "resource_id")?;
            if !processed.insert(resource_id) {
                return Err(RemoteResourceDeletionContractError::DuplicateResourceId {
                    resource_id,
                });
            }
        }
        Ok(())
    }

    pub fn validate_for(
        &self,
        request: &RemoteResourceDeletionRequestV1,
    ) -> Result<(), RemoteResourceDeletionContractError> {
        request.validate()?;
        self.validate()?;

        for (field, matches) in [
            ("version", self.version == request.version),
            ("request_id", self.request_id == request.request_id),
            ("user_id", self.user_id == request.user_id),
            (
                "composition_id",
                self.composition_id == request.composition_id,
            ),
            ("context_id", self.context_id == request.context_id),
        ] {
            if !matches {
                return Err(RemoteResourceDeletionContractError::CorrelationMismatch { field });
            }
        }

        let requested = request.resource_ids().into_iter().collect::<HashSet<_>>();
        let processed = self
            .processed_resource_ids()
            .into_iter()
            .collect::<HashSet<_>>();
        if processed != requested {
            return Err(RemoteResourceDeletionContractError::AcknowledgedResourceSetMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bincode::{config, decode_from_slice, encode_to_vec};
    use uuid::Uuid;

    use super::{
        REMOTE_RESOURCE_DELETION_VERSION_V1, RemoteResourceDeletionAckV1,
        RemoteResourceDeletionContractError, RemoteResourceDeletionRequestV1,
    };

    fn ids() -> (Uuid, Uuid, Uuid, Uuid, Uuid, Uuid) {
        (
            Uuid::now_v7(),
            Uuid::now_v7(),
            Uuid::now_v7(),
            Uuid::now_v7(),
            Uuid::now_v7(),
            Uuid::now_v7(),
        )
    }

    #[test]
    fn request_round_trips_through_bincode_and_validates() {
        let (request_id, user_id, composition_id, context_id, first, second) = ids();
        let request = RemoteResourceDeletionRequestV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![first, second],
        )
        .expect("build request");

        let encoded = encode_to_vec(&request, config::standard()).expect("encode request");
        let (decoded, consumed): (RemoteResourceDeletionRequestV1, usize) =
            decode_from_slice(&encoded, config::standard()).expect("decode request");

        assert_eq!(consumed, encoded.len());
        assert_eq!(decoded, request);
        assert_eq!(decoded.version(), REMOTE_RESOURCE_DELETION_VERSION_V1);
        assert_eq!(decoded.request_id(), request_id);
        assert_eq!(decoded.user_id(), user_id);
        assert_eq!(decoded.composition_id(), composition_id);
        assert_eq!(decoded.context_id(), context_id);
        assert_eq!(decoded.resource_ids(), vec![first, second]);
        decoded.validate().expect("validate decoded request");
    }

    #[test]
    fn acknowledgement_round_trips_and_matches_the_request() {
        let (request_id, user_id, composition_id, context_id, deleted, shared) = ids();
        let request = RemoteResourceDeletionRequestV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![deleted, shared],
        )
        .expect("build request");
        let acknowledgement = RemoteResourceDeletionAckV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![deleted],
            vec![shared],
        )
        .expect("build acknowledgement");

        let encoded =
            encode_to_vec(&acknowledgement, config::standard()).expect("encode acknowledgement");
        let (decoded, consumed): (RemoteResourceDeletionAckV1, usize) =
            decode_from_slice(&encoded, config::standard()).expect("decode acknowledgement");

        assert_eq!(consumed, encoded.len());
        assert_eq!(decoded, acknowledgement);
        assert_eq!(decoded.deleted_or_absent_resource_ids(), vec![deleted]);
        assert_eq!(decoded.retained_shared_resource_ids(), vec![shared]);
        decoded
            .validate_for(&request)
            .expect("acknowledgement matches request");
    }

    #[test]
    fn request_rejects_nil_empty_and_duplicate_values() {
        let (request_id, user_id, composition_id, context_id, resource_id, _) = ids();
        assert!(matches!(
            RemoteResourceDeletionRequestV1::new(
                Uuid::nil(),
                user_id,
                composition_id,
                context_id,
                vec![resource_id]
            ),
            Err(RemoteResourceDeletionContractError::NilUuid {
                field: "request_id"
            })
        ));
        assert!(matches!(
            RemoteResourceDeletionRequestV1::new(
                request_id,
                user_id,
                composition_id,
                context_id,
                vec![]
            ),
            Err(RemoteResourceDeletionContractError::EmptyResourceList)
        ));
        assert!(matches!(
            RemoteResourceDeletionRequestV1::new(
                request_id,
                user_id,
                composition_id,
                context_id,
                vec![resource_id, resource_id]
            ),
            Err(RemoteResourceDeletionContractError::DuplicateResourceId { .. })
        ));
        assert!(matches!(
            RemoteResourceDeletionRequestV1::new(
                request_id,
                user_id,
                composition_id,
                context_id,
                vec![Uuid::nil()]
            ),
            Err(RemoteResourceDeletionContractError::NilUuid {
                field: "resource_id"
            })
        ));
    }

    #[test]
    fn decoded_request_rejects_an_unsupported_version() {
        let (request_id, user_id, composition_id, context_id, resource_id, _) = ids();
        let mut request = RemoteResourceDeletionRequestV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![resource_id],
        )
        .expect("build request");
        request.version = REMOTE_RESOURCE_DELETION_VERSION_V1 + 1;

        assert!(matches!(
            request.validate(),
            Err(RemoteResourceDeletionContractError::UnsupportedVersion { .. })
        ));
    }

    #[test]
    fn acknowledgement_rejects_empty_and_overlapping_outcomes() {
        let (request_id, user_id, composition_id, context_id, resource_id, _) = ids();
        assert!(matches!(
            RemoteResourceDeletionAckV1::new(
                request_id,
                user_id,
                composition_id,
                context_id,
                vec![],
                vec![]
            ),
            Err(RemoteResourceDeletionContractError::EmptyAcknowledgement)
        ));
        assert!(matches!(
            RemoteResourceDeletionAckV1::new(
                request_id,
                user_id,
                composition_id,
                context_id,
                vec![resource_id],
                vec![resource_id]
            ),
            Err(RemoteResourceDeletionContractError::DuplicateResourceId { .. })
        ));
    }

    #[test]
    fn acknowledgement_must_match_correlation_and_the_full_resource_set() {
        let (request_id, user_id, composition_id, context_id, first, second) = ids();
        let request = RemoteResourceDeletionRequestV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![first, second],
        )
        .expect("build request");
        let wrong_request = RemoteResourceDeletionAckV1::new(
            Uuid::now_v7(),
            user_id,
            composition_id,
            context_id,
            vec![first, second],
            vec![],
        )
        .expect("build acknowledgement");
        assert!(matches!(
            wrong_request.validate_for(&request),
            Err(RemoteResourceDeletionContractError::CorrelationMismatch {
                field: "request_id"
            })
        ));

        let partial = RemoteResourceDeletionAckV1::new(
            request_id,
            user_id,
            composition_id,
            context_id,
            vec![first],
            vec![],
        )
        .expect("build partial acknowledgement");
        assert!(matches!(
            partial.validate_for(&request),
            Err(RemoteResourceDeletionContractError::AcknowledgedResourceSetMismatch)
        ));
    }
}
