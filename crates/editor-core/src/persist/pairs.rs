//! Structural-key map encoding (spec D3: appearance keys are
//! [`StableName`]s "serialized structurally"). JSON maps require
//! string keys, so name-keyed maps persist as a LIST of `[key, value]`
//! pairs — the key stays a structural tree. Strict on load: a
//! duplicate key refuses typed (no silent last-wins, D6.3), and order
//! is canonicalized by the `BTreeMap` on rebuild (save order is
//! already canonical because the source IS a `BTreeMap`).

use std::collections::BTreeMap;

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
/// A typed refusal on a duplicated key, the format's one
/// ([`super::strict::duplicate_key`]).
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<BTreeMap<StableName, AppearanceRecord>, D::Error> {
    let pairs: Vec<(StableName, AppearanceRecord)> = Vec::deserialize(de)?;
    let mut map = BTreeMap::new();
    for (key, value) in pairs {
        if map.insert(key.clone(), value).is_some() {
            return Err(super::strict::duplicate_key("appearance", &key));
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::expect_used)]

    use crate::appearance::AppearanceRecord;
    use crate::names::{EntityKind, StableName};
    use crate::node::RecipeNodeId;

    /// A repeated appearance key refuses in the format's one
    /// duplicate-key sentence, its name's minting node by tag.
    #[test]
    fn a_duplicate_appearance_key_is_said_by_its_minting_node() {
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(0x3fa9_c1d2_a0b1_0042),
            path: Vec::new(),
        };
        let record = AppearanceRecord::default();
        let text = serde_json::to_string(&[(&name, &record), (&name, &record)])
            .expect("a pair list writes");
        let mut de = serde_json::Deserializer::from_str(&text);
        let said = match super::deserialize(&mut de) {
            Ok(_) => panic!("a repeated key refuses: {text}"),
            Err(e) => e.to_string(),
        };
        assert!(
            said.starts_with(
                "duplicate appearance key face name minted by node 3fa9c1d2a0b1 — refused, no \
                 silent last-wins"
            ),
            "{said}"
        );
    }
}
