use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// [`RessourcesDescritors`] represents a collection of ressources a client needs to fetch from the
/// server
#[derive(Encode, Deserialize, Serialize, Decode, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RessourcesDescritors;
