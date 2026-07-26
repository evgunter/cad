//! Structural-key map encoding (spec D3: appearance keys are
//! [`StableName`]s "serialized structurally"). JSON maps require
//! string keys, so name-keyed maps persist as a LIST of `[key, value]`
//! pairs — the key stays a structural tree. Strict on load: a
//! duplicate key refuses typed (no silent last-wins, D6.3), and order
//! is canonicalized by the `BTreeMap` on rebuild (save order is
//! already canonical because the source IS a `BTreeMap`).

use std::collections::BTreeMap;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::appearance::AppearanceRecord;
use crate::names::StableName;

/// Serializes the appearance store as a pair list.
///
/// # Errors
///
/// Only the underlying serializer's own errors.
pub(crate) fn serialize<S: Serializer>(
    map: &BTreeMap<StableName, AppearanceRecord>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    let pairs: Vec<(&StableName, &AppearanceRecord)> = map.iter().collect();
    pairs.serialize(ser)
}

/// Deserializes a pair list back into the store, refusing duplicates.
///
/// # Errors
///
/// A typed refusal on a duplicated key.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<BTreeMap<StableName, AppearanceRecord>, D::Error> {
    let pairs: Vec<(StableName, AppearanceRecord)> = Vec::deserialize(de)?;
    let mut map = BTreeMap::new();
    for (key, value) in pairs {
        if map.insert(key.clone(), value).is_some() {
            return Err(D::Error::custom(format!(
                "duplicate appearance key {key:?}"
            )));
        }
    }
    Ok(map)
}
