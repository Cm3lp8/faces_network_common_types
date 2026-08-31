use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{CompositionData, FragmentTransform2DData};

pub const COMPOSITION_DATA_V2_SCHEMA_VERSION: u16 = 2;
pub const COLLECTION_GRAPH_DATA_V1_SCHEMA_VERSION: u16 = 1;
pub const PUBLISH_COMPOSITION_V2_REQUEST_VERSION: u16 = 1;
pub const COMPOSITION_V2_DELTA_PROTOCOL_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub enum CompositionDataV2ContractError {
    UnsupportedCompositionVersion {
        actual: u16,
    },
    UnsupportedCollectionGraphVersion {
        actual: u16,
    },
    NilUuid {
        field: &'static str,
    },
    DuplicateInstanceId {
        instance_id: Uuid,
    },
    DuplicateCollectionId {
        collection_id: Uuid,
    },
    UnknownInstanceTarget {
        instance_id: Uuid,
    },
    UnknownCollectionTarget {
        collection_id: Uuid,
    },
    InstanceHasMultipleParents {
        instance_id: Uuid,
    },
    CollectionHasMultipleParents {
        collection_id: Uuid,
    },
    CollectionCycle {
        collection_id: Uuid,
    },
    NonFiniteTransform {
        owner_id: Uuid,
    },
    InvalidTransformDimensions {
        owner_id: Uuid,
    },
    InvalidTransformScale {
        owner_id: Uuid,
    },
    InconsistentTransformZ {
        owner_id: Uuid,
        position_z: f32,
        authoritative_z: f32,
    },
    InvalidBounds {
        reason: &'static str,
    },
    UnsupportedPublicationVersion {
        actual: u16,
    },
    PublicationSourceMatchesPublishedComposition,
    PublicationRevisionMismatch {
        source_revision: u64,
        snapshot_revision: u64,
    },
    UnsupportedV2DeltaVersion {
        actual: u16,
    },
    DuplicateDeltaContextId {
        context_id: Uuid,
    },
    DuplicateDeltaCompositionId {
        composition_id: Uuid,
    },
}

impl fmt::Display for CompositionDataV2ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCompositionVersion { actual } => write!(
                formatter,
                "unsupported composition data schema version {actual}; expected {COMPOSITION_DATA_V2_SCHEMA_VERSION}"
            ),
            Self::UnsupportedCollectionGraphVersion { actual } => write!(
                formatter,
                "unsupported collection graph schema version {actual}; expected {COLLECTION_GRAPH_DATA_V1_SCHEMA_VERSION}"
            ),
            Self::NilUuid { field } => write!(formatter, "composition field {field} is nil"),
            Self::DuplicateInstanceId { instance_id } => {
                write!(formatter, "duplicate fragment instance id {instance_id}")
            }
            Self::DuplicateCollectionId { collection_id } => {
                write!(formatter, "duplicate collection id {collection_id}")
            }
            Self::UnknownInstanceTarget { instance_id } => {
                write!(
                    formatter,
                    "collection references unknown instance {instance_id}"
                )
            }
            Self::UnknownCollectionTarget { collection_id } => {
                write!(
                    formatter,
                    "collection references unknown collection {collection_id}"
                )
            }
            Self::InstanceHasMultipleParents { instance_id } => {
                write!(
                    formatter,
                    "instance {instance_id} has multiple collection parents"
                )
            }
            Self::CollectionHasMultipleParents { collection_id } => write!(
                formatter,
                "collection {collection_id} has multiple collection parents"
            ),
            Self::CollectionCycle { collection_id } => {
                write!(
                    formatter,
                    "collection graph contains a cycle at {collection_id}"
                )
            }
            Self::NonFiniteTransform { owner_id } => {
                write!(
                    formatter,
                    "transform for {owner_id} contains a non-finite value"
                )
            }
            Self::InvalidTransformDimensions { owner_id } => {
                write!(formatter, "transform for {owner_id} has invalid dimensions")
            }
            Self::InvalidTransformScale { owner_id } => {
                write!(formatter, "transform for {owner_id} has invalid scale")
            }
            Self::InconsistentTransformZ {
                owner_id,
                position_z,
                authoritative_z,
            } => write!(
                formatter,
                "transform for {owner_id} has position z {position_z} but authoritative z {authoritative_z}"
            ),
            Self::InvalidBounds { reason } => {
                write!(formatter, "composition bounds are invalid: {reason}")
            }
            Self::UnsupportedPublicationVersion { actual } => write!(
                formatter,
                "unsupported composition publication version {actual}; expected {PUBLISH_COMPOSITION_V2_REQUEST_VERSION}"
            ),
            Self::PublicationSourceMatchesPublishedComposition => write!(
                formatter,
                "a publication must copy the draft into a distinct composition identity"
            ),
            Self::PublicationRevisionMismatch {
                source_revision,
                snapshot_revision,
            } => write!(
                formatter,
                "publication source revision {source_revision} does not match snapshot revision {snapshot_revision}"
            ),
            Self::UnsupportedV2DeltaVersion { actual } => write!(
                formatter,
                "unsupported composition V2 delta version {actual}; expected {COMPOSITION_V2_DELTA_PROTOCOL_VERSION}"
            ),
            Self::DuplicateDeltaContextId { context_id } => {
                write!(formatter, "duplicate V2 delta context {context_id}")
            }
            Self::DuplicateDeltaCompositionId { composition_id } => {
                write!(formatter, "duplicate V2 delta composition {composition_id}")
            }
        }
    }
}

impl Error for CompositionDataV2ContractError {}

fn validate_uuid(value: Uuid, field: &'static str) -> Result<(), CompositionDataV2ContractError> {
    if value.is_nil() {
        return Err(CompositionDataV2ContractError::NilUuid { field });
    }
    Ok(())
}

fn validate_transform(
    transform: &FragmentTransform2DData,
    owner_id: Uuid,
    dimensions_may_be_zero: bool,
) -> Result<(), CompositionDataV2ContractError> {
    let pos = transform.pos();
    let dimensions = transform.dimensions();
    let scale = transform.scale();
    if !pos.into_iter().all(f32::is_finite)
        || !dimensions.into_iter().all(f32::is_finite)
        || !scale.into_iter().all(f32::is_finite)
        || !transform.rot().is_finite()
        || !transform.z().is_finite()
    {
        return Err(CompositionDataV2ContractError::NonFiniteTransform { owner_id });
    }

    let dimensions_are_valid = if dimensions_may_be_zero {
        dimensions.into_iter().all(|value| value >= 0.0)
    } else {
        dimensions.into_iter().all(|value| value > 0.0)
    };
    if !dimensions_are_valid {
        return Err(CompositionDataV2ContractError::InvalidTransformDimensions { owner_id });
    }
    if !scale.into_iter().all(|value| value > 0.0) {
        return Err(CompositionDataV2ContractError::InvalidTransformScale { owner_id });
    }
    if !nearly_equal(pos[2], transform.z()) {
        return Err(CompositionDataV2ContractError::InconsistentTransformZ {
            owner_id,
            position_z: pos[2],
            authoritative_z: transform.z(),
        });
    }
    Ok(())
}

fn nearly_equal(left: f32, right: f32) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= f32::EPSILON * 16.0 * scale
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionBoundsDataV1 {
    width: f32,
    height: f32,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl CompositionBoundsDataV1 {
    pub fn new(
        width: f32,
        height: f32,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let bounds = Self {
            width,
            height,
            min_x,
            max_x,
            min_y,
            max_y,
        };
        bounds.validate()?;
        Ok(bounds)
    }

    pub const fn empty() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
        }
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        if ![
            self.width,
            self.height,
            self.min_x,
            self.max_x,
            self.min_y,
            self.max_y,
        ]
        .into_iter()
        .all(f32::is_finite)
        {
            return Err(CompositionDataV2ContractError::InvalidBounds {
                reason: "values must be finite",
            });
        }
        if self.width < 0.0 || self.height < 0.0 {
            return Err(CompositionDataV2ContractError::InvalidBounds {
                reason: "width and height must be non-negative",
            });
        }
        if self.min_x > self.max_x || self.min_y > self.max_y {
            return Err(CompositionDataV2ContractError::InvalidBounds {
                reason: "minimum must not exceed maximum",
            });
        }
        if !nearly_equal(self.width, self.max_x - self.min_x)
            || !nearly_equal(self.height, self.max_y - self.min_y)
        {
            return Err(CompositionDataV2ContractError::InvalidBounds {
                reason: "extent must match min/max coordinates",
            });
        }
        Ok(())
    }

    pub const fn width(self) -> f32 {
        self.width
    }

    pub const fn height(self) -> f32 {
        self.height
    }

    pub const fn min_x(self) -> f32 {
        self.min_x
    }

    pub const fn max_x(self) -> f32 {
        self.max_x
    }

    pub const fn min_y(self) -> f32 {
        self.min_y
    }

    pub const fn max_y(self) -> f32 {
        self.max_y
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct FragmentInstanceDataV1 {
    instance_id: [u8; 16],
    resource_id: [u8; 16],
    transform: FragmentTransform2DData,
}

impl FragmentInstanceDataV1 {
    pub fn new(
        instance_id: Uuid,
        resource_id: Uuid,
        transform: FragmentTransform2DData,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let instance = Self {
            instance_id: instance_id.into_bytes(),
            resource_id: resource_id.into_bytes(),
            transform,
        };
        instance.validate()?;
        Ok(instance)
    }

    pub fn instance_id(&self) -> Uuid {
        Uuid::from_bytes(self.instance_id)
    }

    pub fn resource_id(&self) -> Uuid {
        Uuid::from_bytes(self.resource_id)
    }

    pub const fn transform(&self) -> &FragmentTransform2DData {
        &self.transform
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        let instance_id = self.instance_id();
        validate_uuid(instance_id, "instance_id")?;
        validate_uuid(self.resource_id(), "resource_id")?;
        validate_transform(&self.transform, instance_id, false)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub enum CollectionMemberTargetDataV1 {
    Instance([u8; 16]),
    Collection([u8; 16]),
}

impl CollectionMemberTargetDataV1 {
    pub fn instance(instance_id: Uuid) -> Result<Self, CompositionDataV2ContractError> {
        validate_uuid(instance_id, "target_instance_id")?;
        Ok(Self::Instance(instance_id.into_bytes()))
    }

    pub fn collection(collection_id: Uuid) -> Result<Self, CompositionDataV2ContractError> {
        validate_uuid(collection_id, "target_collection_id")?;
        Ok(Self::Collection(collection_id.into_bytes()))
    }

    pub fn instance_id(&self) -> Option<Uuid> {
        match self {
            Self::Instance(value) => Some(Uuid::from_bytes(*value)),
            Self::Collection(_) => None,
        }
    }

    pub fn collection_id(&self) -> Option<Uuid> {
        match self {
            Self::Collection(value) => Some(Uuid::from_bytes(*value)),
            Self::Instance(_) => None,
        }
    }

    fn id(&self) -> Uuid {
        match self {
            Self::Instance(value) | Self::Collection(value) => Uuid::from_bytes(*value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CollectionMemberDataV1 {
    target: CollectionMemberTargetDataV1,
    local_transform: FragmentTransform2DData,
}

impl CollectionMemberDataV1 {
    pub fn new(
        target: CollectionMemberTargetDataV1,
        local_transform: FragmentTransform2DData,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let member = Self {
            target,
            local_transform,
        };
        validate_transform(&member.local_transform, member.target.id(), true)?;
        Ok(member)
    }

    pub const fn target(&self) -> &CollectionMemberTargetDataV1 {
        &self.target
    }

    pub const fn local_transform(&self) -> &FragmentTransform2DData {
        &self.local_transform
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CollectionNodeDataV1 {
    collection_id: [u8; 16],
    label: String,
    ordered_members: Vec<CollectionMemberDataV1>,
}

impl CollectionNodeDataV1 {
    pub fn new(
        collection_id: Uuid,
        label: String,
        ordered_members: Vec<CollectionMemberDataV1>,
    ) -> Result<Self, CompositionDataV2ContractError> {
        validate_uuid(collection_id, "collection_id")?;
        Ok(Self {
            collection_id: collection_id.into_bytes(),
            label,
            ordered_members,
        })
    }

    pub fn collection_id(&self) -> Uuid {
        Uuid::from_bytes(self.collection_id)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn ordered_members(&self) -> &[CollectionMemberDataV1] {
        &self.ordered_members
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CollectionGraphDataV1 {
    schema_version: u16,
    collections: Vec<CollectionNodeDataV1>,
}

impl CollectionGraphDataV1 {
    pub fn new(
        collections: Vec<CollectionNodeDataV1>,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let graph = Self {
            schema_version: COLLECTION_GRAPH_DATA_V1_SCHEMA_VERSION,
            collections,
        };
        graph.validate_intrinsic()?;
        Ok(graph)
    }

    pub const fn empty() -> Self {
        Self {
            schema_version: COLLECTION_GRAPH_DATA_V1_SCHEMA_VERSION,
            collections: Vec::new(),
        }
    }

    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn collections(&self) -> &[CollectionNodeDataV1] {
        &self.collections
    }

    fn validate_intrinsic(&self) -> Result<(), CompositionDataV2ContractError> {
        if self.schema_version != COLLECTION_GRAPH_DATA_V1_SCHEMA_VERSION {
            return Err(
                CompositionDataV2ContractError::UnsupportedCollectionGraphVersion {
                    actual: self.schema_version,
                },
            );
        }

        let mut collection_ids = HashSet::with_capacity(self.collections.len());
        for collection in &self.collections {
            let collection_id = collection.collection_id();
            validate_uuid(collection_id, "collection_id")?;
            if !collection_ids.insert(collection_id) {
                return Err(CompositionDataV2ContractError::DuplicateCollectionId {
                    collection_id,
                });
            }
            for member in collection.ordered_members() {
                validate_transform(member.local_transform(), member.target().id(), true)?;
            }
        }

        let mut instance_parents = HashSet::new();
        let mut collection_parents = HashMap::new();
        for collection in &self.collections {
            let parent_id = collection.collection_id();
            for member in collection.ordered_members() {
                match member.target() {
                    CollectionMemberTargetDataV1::Instance(raw_instance_id) => {
                        let instance_id = Uuid::from_bytes(*raw_instance_id);
                        validate_uuid(instance_id, "target_instance_id")?;
                        if !instance_parents.insert(instance_id) {
                            return Err(
                                CompositionDataV2ContractError::InstanceHasMultipleParents {
                                    instance_id,
                                },
                            );
                        }
                    }
                    CollectionMemberTargetDataV1::Collection(raw_collection_id) => {
                        let collection_id = Uuid::from_bytes(*raw_collection_id);
                        validate_uuid(collection_id, "target_collection_id")?;
                        if !collection_ids.contains(&collection_id) {
                            return Err(CompositionDataV2ContractError::UnknownCollectionTarget {
                                collection_id,
                            });
                        }
                        if collection_parents
                            .insert(collection_id, parent_id)
                            .is_some()
                        {
                            return Err(
                                CompositionDataV2ContractError::CollectionHasMultipleParents {
                                    collection_id,
                                },
                            );
                        }
                    }
                }
            }
        }

        for start in &collection_ids {
            let mut path = HashSet::new();
            let mut current = *start;
            while let Some(parent) = collection_parents.get(&current).copied() {
                if !path.insert(current) {
                    return Err(CompositionDataV2ContractError::CollectionCycle {
                        collection_id: current,
                    });
                }
                current = parent;
            }
        }
        Ok(())
    }

    fn validate_with_instances(
        &self,
        instance_ids: &HashSet<Uuid>,
    ) -> Result<(), CompositionDataV2ContractError> {
        self.validate_intrinsic()?;
        for collection in &self.collections {
            for member in collection.ordered_members() {
                if let Some(instance_id) = member.target().instance_id()
                    && !instance_ids.contains(&instance_id)
                {
                    return Err(CompositionDataV2ContractError::UnknownInstanceTarget {
                        instance_id,
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionDataV2 {
    schema_version: u16,
    composition_id: [u8; 16],
    author_id: [u8; 16],
    revision: u64,
    instances: Vec<FragmentInstanceDataV1>,
    collection_graph: CollectionGraphDataV1,
    bounds: CompositionBoundsDataV1,
}

impl CompositionDataV2 {
    pub fn new(
        composition_id: Uuid,
        author_id: Uuid,
        revision: u64,
        instances: Vec<FragmentInstanceDataV1>,
        collection_graph: CollectionGraphDataV1,
        bounds: CompositionBoundsDataV1,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let composition = Self {
            schema_version: COMPOSITION_DATA_V2_SCHEMA_VERSION,
            composition_id: composition_id.into_bytes(),
            author_id: author_id.into_bytes(),
            revision,
            instances,
            collection_graph,
            bounds,
        };
        composition.validate()?;
        Ok(composition)
    }

    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn composition_id(&self) -> Uuid {
        Uuid::from_bytes(self.composition_id)
    }

    pub fn author_id(&self) -> Uuid {
        Uuid::from_bytes(self.author_id)
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn instances(&self) -> &[FragmentInstanceDataV1] {
        &self.instances
    }

    pub const fn collection_graph(&self) -> &CollectionGraphDataV1 {
        &self.collection_graph
    }

    pub const fn bounds(&self) -> CompositionBoundsDataV1 {
        self.bounds
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        if self.schema_version != COMPOSITION_DATA_V2_SCHEMA_VERSION {
            return Err(
                CompositionDataV2ContractError::UnsupportedCompositionVersion {
                    actual: self.schema_version,
                },
            );
        }
        validate_uuid(self.composition_id(), "composition_id")?;
        validate_uuid(self.author_id(), "author_id")?;
        self.bounds.validate()?;

        let mut instance_ids = HashSet::with_capacity(self.instances.len());
        for instance in &self.instances {
            instance.validate()?;
            let instance_id = instance.instance_id();
            if !instance_ids.insert(instance_id) {
                return Err(CompositionDataV2ContractError::DuplicateInstanceId { instance_id });
            }
        }
        self.collection_graph.validate_with_instances(&instance_ids)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Decode, Encode)]
pub enum CompositionWireEnvelope {
    V1(CompositionData),
    V2(CompositionDataV2),
}

impl CompositionWireEnvelope {
    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        match self {
            Self::V1(_) => Ok(()),
            Self::V2(composition) => composition.validate(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct PublishCompositionV2Request {
    protocol_version: u16,
    request_id: [u8; 16],
    context_id: [u8; 16],
    source_composition_id: [u8; 16],
    source_revision: u64,
    composition: CompositionDataV2,
}

impl PublishCompositionV2Request {
    pub fn new(
        request_id: Uuid,
        context_id: Uuid,
        source_composition_id: Uuid,
        source_revision: u64,
        composition: CompositionDataV2,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let request = Self {
            protocol_version: PUBLISH_COMPOSITION_V2_REQUEST_VERSION,
            request_id: request_id.into_bytes(),
            context_id: context_id.into_bytes(),
            source_composition_id: source_composition_id.into_bytes(),
            source_revision,
            composition,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn request_id(&self) -> Uuid {
        Uuid::from_bytes(self.request_id)
    }

    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }

    pub fn source_composition_id(&self) -> Uuid {
        Uuid::from_bytes(self.source_composition_id)
    }

    pub const fn source_revision(&self) -> u64 {
        self.source_revision
    }

    pub const fn composition(&self) -> &CompositionDataV2 {
        &self.composition
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        if self.protocol_version != PUBLISH_COMPOSITION_V2_REQUEST_VERSION {
            return Err(
                CompositionDataV2ContractError::UnsupportedPublicationVersion {
                    actual: self.protocol_version,
                },
            );
        }
        validate_uuid(self.request_id(), "publication.request_id")?;
        validate_uuid(self.context_id(), "publication.context_id")?;
        validate_uuid(
            self.source_composition_id(),
            "publication.source_composition_id",
        )?;
        self.composition.validate()?;
        if self.source_composition_id() == self.composition.composition_id() {
            return Err(
                CompositionDataV2ContractError::PublicationSourceMatchesPublishedComposition,
            );
        }
        if self.source_revision != self.composition.revision() {
            return Err(
                CompositionDataV2ContractError::PublicationRevisionMismatch {
                    source_revision: self.source_revision,
                    snapshot_revision: self.composition.revision(),
                },
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct PublishCompositionV2Response {
    published_composition_id: [u8; 16],
    user_session_version: i64,
    newly_published: bool,
}

impl PublishCompositionV2Response {
    pub fn new(
        published_composition_id: Uuid,
        user_session_version: i64,
        newly_published: bool,
    ) -> Self {
        Self {
            published_composition_id: published_composition_id.into_bytes(),
            user_session_version,
            newly_published,
        }
    }

    pub fn published_composition_id(&self) -> Uuid {
        Uuid::from_bytes(self.published_composition_id)
    }

    pub const fn user_session_version(&self) -> i64 {
        self.user_session_version
    }

    pub const fn newly_published(&self) -> bool {
        self.newly_published
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct ServerCapabilitiesV1 {
    composition_v2: bool,
}

impl ServerCapabilitiesV1 {
    pub const fn new(composition_v2: bool) -> Self {
        Self { composition_v2 }
    }

    pub const fn composition_v2(self) -> bool {
        self.composition_v2
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct FetchCompositionsV2DeltaRequest {
    protocol_version: u16,
    last_user_session_version: u64,
}

impl FetchCompositionsV2DeltaRequest {
    pub const fn new(last_user_session_version: u64) -> Self {
        Self {
            protocol_version: COMPOSITION_V2_DELTA_PROTOCOL_VERSION,
            last_user_session_version,
        }
    }

    pub const fn last_user_session_version(self) -> u64 {
        self.last_user_session_version
    }

    pub fn validate(self) -> Result<(), CompositionDataV2ContractError> {
        if self.protocol_version != COMPOSITION_V2_DELTA_PROTOCOL_VERSION {
            return Err(CompositionDataV2ContractError::UnsupportedV2DeltaVersion {
                actual: self.protocol_version,
            });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct CompositionContextV2Delta {
    context_id: [u8; 16],
    context_version: u64,
    compositions: Vec<CompositionDataV2>,
}

impl CompositionContextV2Delta {
    pub fn new(
        context_id: Uuid,
        context_version: u64,
        compositions: Vec<CompositionDataV2>,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let delta = Self {
            context_id: context_id.into_bytes(),
            context_version,
            compositions,
        };
        delta.validate()?;
        Ok(delta)
    }

    pub fn context_id(&self) -> Uuid {
        Uuid::from_bytes(self.context_id)
    }

    pub const fn context_version(&self) -> u64 {
        self.context_version
    }

    pub fn compositions(&self) -> &[CompositionDataV2] {
        &self.compositions
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        validate_uuid(self.context_id(), "delta.context_id")?;
        let mut composition_ids = HashSet::with_capacity(self.compositions.len());
        for composition in &self.compositions {
            composition.validate()?;
            let composition_id = composition.composition_id();
            if !composition_ids.insert(composition_id) {
                return Err(
                    CompositionDataV2ContractError::DuplicateDeltaCompositionId { composition_id },
                );
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct FetchCompositionsV2DeltaResponse {
    protocol_version: u16,
    user_session_version: u64,
    contexts: Vec<CompositionContextV2Delta>,
}

impl FetchCompositionsV2DeltaResponse {
    pub fn new(
        user_session_version: u64,
        contexts: Vec<CompositionContextV2Delta>,
    ) -> Result<Self, CompositionDataV2ContractError> {
        let response = Self {
            protocol_version: COMPOSITION_V2_DELTA_PROTOCOL_VERSION,
            user_session_version,
            contexts,
        };
        response.validate()?;
        Ok(response)
    }

    pub const fn user_session_version(&self) -> u64 {
        self.user_session_version
    }

    pub fn contexts(&self) -> &[CompositionContextV2Delta] {
        &self.contexts
    }

    pub fn validate(&self) -> Result<(), CompositionDataV2ContractError> {
        if self.protocol_version != COMPOSITION_V2_DELTA_PROTOCOL_VERSION {
            return Err(CompositionDataV2ContractError::UnsupportedV2DeltaVersion {
                actual: self.protocol_version,
            });
        }
        let mut context_ids = HashSet::with_capacity(self.contexts.len());
        let mut composition_ids = HashSet::new();
        for context in &self.contexts {
            context.validate()?;
            let context_id = context.context_id();
            if !context_ids.insert(context_id) {
                return Err(CompositionDataV2ContractError::DuplicateDeltaContextId { context_id });
            }
            for composition in context.compositions() {
                let composition_id = composition.composition_id();
                if !composition_ids.insert(composition_id) {
                    return Err(
                        CompositionDataV2ContractError::DuplicateDeltaCompositionId {
                            composition_id,
                        },
                    );
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u128) -> Uuid {
        Uuid::from_u128(value)
    }

    fn transform(x: f32) -> FragmentTransform2DData {
        FragmentTransform2DData::new([x, 0.0, x], [2.0, 3.0], [1.0, 1.0], 0.0, x)
    }

    fn local_transform() -> FragmentTransform2DData {
        FragmentTransform2DData::new([0.0, 0.0, 0.0], [0.0, 0.0], [1.0, 1.0], 0.0, 0.0)
    }

    fn sample_v2() -> CompositionDataV2 {
        let first_instance =
            FragmentInstanceDataV1::new(id(10), id(100), transform(-1.0)).expect("first instance");
        let second_instance =
            FragmentInstanceDataV1::new(id(11), id(100), transform(1.0)).expect("second instance");
        let child = CollectionNodeDataV1::new(
            id(21),
            "child".to_owned(),
            vec![
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::instance(id(11)).expect("instance target"),
                    local_transform(),
                )
                .expect("member"),
            ],
        )
        .expect("child collection");
        let root = CollectionNodeDataV1::new(
            id(20),
            "root".to_owned(),
            vec![
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::instance(id(10)).expect("instance target"),
                    local_transform(),
                )
                .expect("member"),
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::collection(id(21)).expect("collection target"),
                    local_transform(),
                )
                .expect("member"),
            ],
        )
        .expect("root collection");

        CompositionDataV2::new(
            id(1),
            id(2),
            7,
            vec![first_instance, second_instance],
            CollectionGraphDataV1::new(vec![root, child]).expect("graph"),
            CompositionBoundsDataV1::new(4.0, 3.0, -2.0, 2.0, -1.5, 1.5).expect("bounds"),
        )
        .expect("composition")
    }

    #[test]
    fn v2_round_trips_through_json_and_bincode() {
        let composition = sample_v2();

        let json = serde_json::to_string(&composition).expect("serialize json");
        let from_json: CompositionDataV2 = serde_json::from_str(&json).expect("deserialize json");
        from_json.validate().expect("validate json round-trip");
        assert_eq!(from_json, composition);

        let bytes = bincode::encode_to_vec(&composition, bincode::config::standard())
            .expect("encode bincode");
        let (from_bincode, consumed): (CompositionDataV2, usize) =
            bincode::decode_from_slice(&bytes, bincode::config::standard())
                .expect("decode bincode");
        assert_eq!(consumed, bytes.len());
        from_bincode
            .validate()
            .expect("validate bincode round-trip");
        assert_eq!(from_bincode, composition);
    }

    #[test]
    fn publication_request_round_trips_and_preserves_draft_provenance() {
        let request = PublishCompositionV2Request::new(id(30), id(31), id(32), 7, sample_v2())
            .expect("publication request");

        let bytes = bincode::encode_to_vec(&request, bincode::config::standard())
            .expect("encode publication request");
        let (decoded, consumed): (PublishCompositionV2Request, usize) =
            bincode::decode_from_slice(&bytes, bincode::config::standard())
                .expect("decode publication request");

        assert_eq!(consumed, bytes.len());
        decoded.validate().expect("validate publication request");
        assert_eq!(decoded, request);
        assert_eq!(decoded.source_composition_id(), id(32));
        assert_eq!(decoded.source_revision(), 7);
        assert_eq!(decoded.composition().composition_id(), id(1));
    }

    #[test]
    fn publication_must_copy_the_draft_into_a_new_composition_identity() {
        let composition = sample_v2();
        let error = PublishCompositionV2Request::new(
            id(30),
            id(31),
            composition.composition_id(),
            composition.revision(),
            composition,
        )
        .expect_err("published composition must have a distinct identity");

        assert_eq!(
            error,
            CompositionDataV2ContractError::PublicationSourceMatchesPublishedComposition
        );
    }

    #[test]
    fn v2_delta_round_trips_without_using_the_v1_projection() {
        let response = FetchCompositionsV2DeltaResponse::new(
            12,
            vec![
                CompositionContextV2Delta::new(id(40), 5, vec![sample_v2()])
                    .expect("context delta"),
            ],
        )
        .expect("delta response");

        let bytes =
            bincode::encode_to_vec(&response, bincode::config::standard()).expect("encode delta");
        let (decoded, consumed): (FetchCompositionsV2DeltaResponse, usize) =
            bincode::decode_from_slice(&bytes, bincode::config::standard()).expect("decode delta");

        assert_eq!(consumed, bytes.len());
        decoded.validate().expect("validate delta");
        assert_eq!(decoded, response);
        assert_eq!(decoded.contexts()[0].compositions()[0].instances().len(), 2);
    }

    #[test]
    fn two_instances_may_share_one_resource_and_keep_distinct_transforms() {
        let composition = sample_v2();
        assert_eq!(composition.instances()[0].resource_id(), id(100));
        assert_eq!(composition.instances()[1].resource_id(), id(100));
        assert_ne!(
            composition.instances()[0].instance_id(),
            composition.instances()[1].instance_id()
        );
        assert_ne!(
            composition.instances()[0].transform(),
            composition.instances()[1].transform()
        );
    }

    #[test]
    fn collection_and_member_order_survives_round_trip() {
        let composition = sample_v2();
        let bytes = bincode::encode_to_vec(&composition, bincode::config::standard())
            .expect("encode bincode");
        let (decoded, _): (CompositionDataV2, usize) =
            bincode::decode_from_slice(&bytes, bincode::config::standard())
                .expect("decode bincode");

        assert_eq!(decoded.collection_graph().collections()[0].label(), "root");
        assert_eq!(decoded.collection_graph().collections()[1].label(), "child");
        let members = decoded.collection_graph().collections()[0].ordered_members();
        assert_eq!(members[0].target().instance_id(), Some(id(10)));
        assert_eq!(members[1].target().collection_id(), Some(id(21)));
    }

    #[test]
    fn unknown_instance_target_is_rejected() {
        let collection = CollectionNodeDataV1::new(
            id(20),
            "root".to_owned(),
            vec![
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::instance(id(99)).expect("instance target"),
                    local_transform(),
                )
                .expect("member"),
            ],
        )
        .expect("collection");
        let error = CompositionDataV2::new(
            id(1),
            id(2),
            0,
            vec![FragmentInstanceDataV1::new(id(10), id(100), transform(0.0)).expect("instance")],
            CollectionGraphDataV1::new(vec![collection]).expect("graph"),
            CompositionBoundsDataV1::new(2.0, 3.0, -1.0, 1.0, -1.5, 1.5).expect("bounds"),
        )
        .expect_err("unknown target must fail");
        assert_eq!(
            error,
            CompositionDataV2ContractError::UnknownInstanceTarget {
                instance_id: id(99)
            }
        );
    }

    #[test]
    fn multiple_parents_and_cycles_are_rejected() {
        let shared_member = || {
            CollectionMemberDataV1::new(
                CollectionMemberTargetDataV1::instance(id(10)).expect("instance target"),
                local_transform(),
            )
            .expect("member")
        };
        let first = CollectionNodeDataV1::new(id(20), "a".to_owned(), vec![shared_member()])
            .expect("first collection");
        let second = CollectionNodeDataV1::new(id(21), "b".to_owned(), vec![shared_member()])
            .expect("second collection");
        assert_eq!(
            CollectionGraphDataV1::new(vec![first, second]),
            Err(CompositionDataV2ContractError::InstanceHasMultipleParents {
                instance_id: id(10)
            })
        );

        let first = CollectionNodeDataV1::new(
            id(20),
            "a".to_owned(),
            vec![
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::collection(id(21)).expect("collection target"),
                    local_transform(),
                )
                .expect("member"),
            ],
        )
        .expect("first collection");
        let second = CollectionNodeDataV1::new(
            id(21),
            "b".to_owned(),
            vec![
                CollectionMemberDataV1::new(
                    CollectionMemberTargetDataV1::collection(id(20)).expect("collection target"),
                    local_transform(),
                )
                .expect("member"),
            ],
        )
        .expect("second collection");
        assert!(matches!(
            CollectionGraphDataV1::new(vec![first, second]),
            Err(CompositionDataV2ContractError::CollectionCycle { .. })
        ));
    }

    #[test]
    fn invalid_transform_values_are_rejected() {
        assert_eq!(
            FragmentInstanceDataV1::new(
                id(10),
                id(100),
                FragmentTransform2DData::new(
                    [f32::NAN, 0.0, 0.0],
                    [1.0, 1.0],
                    [1.0, 1.0],
                    0.0,
                    0.0,
                )
            ),
            Err(CompositionDataV2ContractError::NonFiniteTransform { owner_id: id(10) })
        );
        assert_eq!(
            FragmentInstanceDataV1::new(
                id(10),
                id(100),
                FragmentTransform2DData::new([0.0, 0.0, 0.0], [0.0, 1.0], [1.0, 1.0], 0.0, 0.0,)
            ),
            Err(CompositionDataV2ContractError::InvalidTransformDimensions { owner_id: id(10) })
        );
        assert_eq!(
            FragmentInstanceDataV1::new(
                id(10),
                id(100),
                FragmentTransform2DData::new([0.0, 0.0, 2.0], [1.0, 1.0], [1.0, 1.0], 0.0, 3.0,)
            ),
            Err(CompositionDataV2ContractError::InconsistentTransformZ {
                owner_id: id(10),
                position_z: 2.0,
                authoritative_z: 3.0,
            })
        );
    }
}
