//! Serde adapter for `jigsaw_core::Weight`.
//!
//! jigsaw-core's `Weight` is a newtype over `i64` without serde support.
//! This module provides a bridge: serialized as an integer, deserialized
//! back into `Weight`.

use jigsaw_core::Weight;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(weight: &Weight, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    weight.milli().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Weight, D::Error>
where
    D: Deserializer<'de>,
{
    let milli = i64::deserialize(deserializer)?;
    Ok(Weight(milli))
}

/// Serialize an `Option<Weight>`.
pub fn serialize_opt<S>(weight: &Option<Weight>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match weight {
        Some(w) => serialize(w, serializer),
        None => serializer.serialize_none(),
    }
}

/// Deserialize an `Option<Weight>`.
pub fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<Weight>, D::Error>
where
    D: Deserializer<'de>,
{
    let milli: Option<i64> = Option::deserialize(deserializer)?;
    Ok(milli.map(Weight))
}
