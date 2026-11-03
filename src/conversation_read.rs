//! Exact account consultation, independent of transport acknowledgements and media cursors.
use std::{collections::HashSet, error::Error, fmt};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const CONVERSATION_READ_VERSION_V1: u16 = 1;
pub const CONVERSATION_READ_MAX_IDS: usize = 256;
/// Both encoded requests and responses fit within this limit. Apply it before decoding too.
pub const CONVERSATION_READ_MAX_BODY_BYTES: usize = 8192;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationReadContractError {
    UnsupportedVersion,
    NilUuid,
    InvalidListSize,
    DuplicateId,
    ContextMismatch,
    UnexpectedIds,
    IncompleteAcknowledgement,
}

impl fmt::Display for ConversationReadContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid conversation read contract: {self:?}")
    }
}
impl Error for ConversationReadContractError {}

fn validate_header(version: u16, context: [u8; 16]) -> Result<(), ConversationReadContractError> {
    if version != CONVERSATION_READ_VERSION_V1 {
        return Err(ConversationReadContractError::UnsupportedVersion);
    }
    if Uuid::from_bytes(context).is_nil() {
        return Err(ConversationReadContractError::NilUuid);
    }
    Ok(())
}

fn id_set(ids: &[[u8; 16]]) -> Result<HashSet<[u8; 16]>, ConversationReadContractError> {
    if ids.len() > CONVERSATION_READ_MAX_IDS {
        return Err(ConversationReadContractError::InvalidListSize);
    }
    let mut set = HashSet::with_capacity(ids.len());
    for id in ids {
        if Uuid::from_bytes(*id).is_nil() {
            return Err(ConversationReadContractError::NilUuid);
        }
        if !set.insert(*id) {
            return Err(ConversationReadContractError::DuplicateId);
        }
    }
    Ok(set)
}

fn raw_ids(ids: Vec<Uuid>) -> Vec<[u8; 16]> {
    ids.into_iter().map(Uuid::into_bytes).collect()
}
fn uuids(ids: &[[u8; 16]]) -> Vec<Uuid> {
    ids.iter().copied().map(Uuid::from_bytes).collect()
}

/// The account is supplied by authenticated transport, never by this body.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct ConversationReadRequestV1 {
    contract_version: u16,
    context_id: [u8; 16],
    composition_ids: Vec<[u8; 16]>,
}

impl ConversationReadRequestV1 {
    pub fn new(
        context_id: Uuid,
        composition_ids: Vec<Uuid>,
    ) -> Result<Self, ConversationReadContractError> {
        let request = Self {
            contract_version: CONVERSATION_READ_VERSION_V1,
            context_id: context_id.into_bytes(),
            composition_ids: raw_ids(composition_ids),
        };
        request.validate()?;
        Ok(request)
    }
    pub fn validate(&self) -> Result<(), ConversationReadContractError> {
        validate_header(self.contract_version, self.context_id)?;
        if self.composition_ids.is_empty() {
            return Err(ConversationReadContractError::InvalidListSize);
        }
        id_set(&self.composition_ids)?;
        Ok(())
    }
    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }
    pub fn composition_ids(&self) -> Vec<Uuid> {
        uuids(&self.composition_ids)
    }
}

/// A committed result partitions the exact request into durable reads and absent content.
/// Absence completes the retry; it does not assert a server read row exists.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct MarkConversationReadResponseV1 {
    contract_version: u16,
    context_id: [u8; 16],
    seen_ids: Vec<[u8; 16]>,
    absent_ids: Vec<[u8; 16]>,
}

impl MarkConversationReadResponseV1 {
    pub fn new(
        request: &ConversationReadRequestV1,
        seen_ids: Vec<Uuid>,
        absent_ids: Vec<Uuid>,
    ) -> Result<Self, ConversationReadContractError> {
        let response = Self {
            contract_version: CONVERSATION_READ_VERSION_V1,
            context_id: request.context_id,
            seen_ids: raw_ids(seen_ids),
            absent_ids: raw_ids(absent_ids),
        };
        response.validate_for(request)?;
        Ok(response)
    }
    pub fn validate_for(
        &self,
        request: &ConversationReadRequestV1,
    ) -> Result<(), ConversationReadContractError> {
        request.validate()?;
        validate_header(self.contract_version, self.context_id)?;
        if self.context_id != request.context_id {
            return Err(ConversationReadContractError::ContextMismatch);
        }
        let seen = id_set(&self.seen_ids)?;
        let absent = id_set(&self.absent_ids)?;
        if !seen.is_disjoint(&absent) {
            return Err(ConversationReadContractError::DuplicateId);
        }
        let expected = id_set(&request.composition_ids)?;
        let actual: HashSet<_> = seen.union(&absent).copied().collect();
        if !actual.is_subset(&expected) {
            return Err(ConversationReadContractError::UnexpectedIds);
        }
        if actual != expected {
            return Err(ConversationReadContractError::IncompleteAcknowledgement);
        }
        Ok(())
    }
    pub fn seen_ids(&self) -> Vec<Uuid> {
        uuids(&self.seen_ids)
    }
    pub fn absent_ids(&self) -> Vec<Uuid> {
        uuids(&self.absent_ids)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct QueryConversationReadResponseV1 {
    contract_version: u16,
    context_id: [u8; 16],
    seen_ids: Vec<[u8; 16]>,
}

impl QueryConversationReadResponseV1 {
    pub fn new(
        request: &ConversationReadRequestV1,
        seen_ids: Vec<Uuid>,
    ) -> Result<Self, ConversationReadContractError> {
        let response = Self {
            contract_version: CONVERSATION_READ_VERSION_V1,
            context_id: request.context_id,
            seen_ids: raw_ids(seen_ids),
        };
        response.validate_for(request)?;
        Ok(response)
    }
    pub fn validate_for(
        &self,
        request: &ConversationReadRequestV1,
    ) -> Result<(), ConversationReadContractError> {
        request.validate()?;
        validate_header(self.contract_version, self.context_id)?;
        if self.context_id != request.context_id {
            return Err(ConversationReadContractError::ContextMismatch);
        }
        if !id_set(&self.seen_ids)?.is_subset(&id_set(&request.composition_ids)?) {
            return Err(ConversationReadContractError::UnexpectedIds);
        }
        Ok(())
    }
    pub fn seen_ids(&self) -> Vec<Uuid> {
        uuids(&self.seen_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }
    fn request() -> ConversationReadRequestV1 {
        ConversationReadRequestV1::new(id(1), vec![id(2), id(3)]).unwrap()
    }

    #[test]
    fn exact_acknowledgement_allows_lost_response_retry_and_deleted_content() {
        let request = request();
        let ack = MarkConversationReadResponseV1::new(&request, vec![id(2)], vec![id(3)]).unwrap();
        assert!(ack.validate_for(&request).is_ok());
        assert_eq!(ack.seen_ids(), vec![id(2)]);
        assert_eq!(ack.absent_ids(), vec![id(3)]);
        // Ordering is not a consultation boundary.
        let reordered = ConversationReadRequestV1::new(id(1), vec![id(3), id(2)]).unwrap();
        assert!(ack.validate_for(&reordered).is_ok());
        assert!(MarkConversationReadResponseV1::new(&request, vec![id(2)], vec![]).is_err());
        assert!(
            MarkConversationReadResponseV1::new(&request, vec![id(2), id(3)], vec![id(3)]).is_err()
        );
        assert!(MarkConversationReadResponseV1::new(&request, vec![id(2), id(4)], vec![]).is_err());
    }

    #[test]
    fn query_cannot_consume_an_arrival_outside_the_captured_revision() {
        let request = request();
        assert!(QueryConversationReadResponseV1::new(&request, vec![]).is_ok());
        assert!(QueryConversationReadResponseV1::new(&request, vec![id(3)]).is_ok());
        assert!(QueryConversationReadResponseV1::new(&request, vec![id(4)]).is_err());
        assert!(QueryConversationReadResponseV1::new(&request, vec![id(2), id(2)]).is_err());
        let mut response = QueryConversationReadResponseV1::new(&request, vec![id(2)]).unwrap();
        response.context_id = id(9).into_bytes();
        assert_eq!(
            response.validate_for(&request),
            Err(ConversationReadContractError::ContextMismatch)
        );
    }

    #[test]
    fn malformed_or_unbounded_requests_are_rejected() {
        for ids in [
            vec![],
            vec![Uuid::nil()],
            vec![id(2), id(2)],
            (1..=257).map(id).collect(),
        ] {
            assert!(ConversationReadRequestV1::new(id(1), ids).is_err());
        }
        assert!(ConversationReadRequestV1::new(Uuid::nil(), vec![id(2)]).is_err());
        let mut request = request();
        request.contract_version = 2;
        assert_eq!(
            request.validate(),
            Err(ConversationReadContractError::UnsupportedVersion)
        );
    }

    #[test]
    fn bounded_wire_roundtrip_preserves_exact_ids_and_fits_decode_limit() {
        let request = ConversationReadRequestV1::new(id(300), (1..=256).map(id).collect()).unwrap();
        let ack = MarkConversationReadResponseV1::new(&request, request.composition_ids(), vec![])
            .unwrap();
        let query =
            QueryConversationReadResponseV1::new(&request, request.composition_ids()).unwrap();
        let bytes = bincode::encode_to_vec(&request, bincode::config::standard()).unwrap();
        assert!(bytes.len() <= CONVERSATION_READ_MAX_BODY_BYTES);
        let (decoded, consumed): (ConversationReadRequestV1, _) = bincode::decode_from_slice(
            &bytes,
            bincode::config::standard().with_limit::<CONVERSATION_READ_MAX_BODY_BYTES>(),
        )
        .unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded, request);
        let bytes = bincode::encode_to_vec(&ack, bincode::config::standard()).unwrap();
        assert!(bytes.len() <= CONVERSATION_READ_MAX_BODY_BYTES);
        let (decoded, consumed): (MarkConversationReadResponseV1, _) = bincode::decode_from_slice(
            &bytes,
            bincode::config::standard().with_limit::<CONVERSATION_READ_MAX_BODY_BYTES>(),
        )
        .unwrap();
        assert_eq!(consumed, bytes.len());
        assert!(decoded.validate_for(&request).is_ok());
        let bytes = bincode::encode_to_vec(&query, bincode::config::standard()).unwrap();
        let (decoded, consumed): (QueryConversationReadResponseV1, _) = bincode::decode_from_slice(
            &bytes,
            bincode::config::standard().with_limit::<CONVERSATION_READ_MAX_BODY_BYTES>(),
        )
        .unwrap();
        assert_eq!(consumed, bytes.len());
        assert!(decoded.validate_for(&request).is_ok());
    }
}
