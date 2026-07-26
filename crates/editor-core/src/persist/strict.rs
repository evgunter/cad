//! Duplicate-refusing map deserialization (review MAJOR-2, D6.3):
//! serde's default map visitor silently keeps the LAST value when a
//! JSON object repeats a key — a duplicate-key file is a corrupt
//! file, and no corrupt file may load silently. Every serde-derived
//! `BTreeMap` in the format deserializes through one of the
//! section-labeled modules below (the pair-list appearance store has
//! the same rule in [`super::pairs`]); the refusal is typed at parse,
//! naming the key and the section.
//!
//! Serialization is untouched (a `BTreeMap` cannot hold duplicates —
//! the modules forward to the plain impl so `#[serde(with)]` stays
//! symmetric).

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserializer, Error as _, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};

/// The shared strict-map visitor: refuses the first repeated key with
/// a typed message carrying the section label and the key's Debug.
pub(crate) fn strict_map<'de, K, V, D>(
    de: D,
    section: &'static str,
) -> Result<BTreeMap<K, V>, D::Error>
where
    K: Deserialize<'de> + Ord + fmt::Debug,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct Vis<K, V> {
        section: &'static str,
        marker: PhantomData<(K, V)>,
    }
    impl<'de, K, V> Visitor<'de> for Vis<K, V>
    where
        K: Deserialize<'de> + Ord + fmt::Debug,
        V: Deserialize<'de>,
    {
        type Value = BTreeMap<K, V>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "a map ({}) without duplicate keys", self.section)
        }
        fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
            let mut out = BTreeMap::new();
            while let Some(key) = access.next_key::<K>()? {
                if out.contains_key(&key) {
                    return Err(A::Error::custom(format!(
                        "duplicate {} key {key:?} — refused, no silent last-wins",
                        self.section
                    )));
                }
                let value = access.next_value::<V>()?;
                out.insert(key, value);
            }
            Ok(out)
        }
    }
    de.deserialize_map(Vis {
        section,
        marker: PhantomData,
    })
}

macro_rules! strict_map_section {
    ($(#[$doc:meta])* $name:ident, $section:literal) => {
        $(#[$doc])*
        pub(crate) mod $name {
            use super::*;

            pub(crate) fn serialize<K, V, S>(
                map: &BTreeMap<K, V>,
                ser: S,
            ) -> Result<S::Ok, S::Error>
            where
                K: Serialize + Ord,
                V: Serialize,
                S: Serializer,
            {
                map.serialize(ser)
            }

            pub(crate) fn deserialize<'de, K, V, D>(de: D) -> Result<BTreeMap<K, V>, D::Error>
            where
                K: Deserialize<'de> + Ord + fmt::Debug,
                V: Deserialize<'de>,
                D: Deserializer<'de>,
            {
                strict_map(de, $section)
            }
        }
    };
}

strict_map_section!(
    /// The snapshot's node map (id → node).
    nodes,
    "snapshot node"
);
strict_map_section!(
    /// The document parameter table.
    params,
    "document parameter"
);
strict_map_section!(
    /// The per-node witness store.
    witnesses,
    "witness node"
);
strict_map_section!(
    /// The document's free-form metadata map.
    doc_metadata,
    "document metadata"
);
strict_map_section!(
    /// An appearance record's attribute map.
    attrs,
    "appearance attribute"
);
strict_map_section!(
    /// An appearance record's D7 metadata map.
    record_metadata,
    "appearance metadata"
);
strict_map_section!(
    /// A `MetaValue::Map`'s entries.
    meta_map,
    "metadata map"
);
strict_map_section!(
    /// A verdict summary's node map (ε-audit interchange).
    summary_nodes,
    "verdict summary node"
);
strict_map_section!(
    /// A node's per-predicate verdict populations (ε-audit
    /// interchange).
    populations,
    "verdict population"
);
