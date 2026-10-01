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

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};

/// **How a duplicate-key refusal says its key.** The refusal is raised
/// at parse, before any document exists, so a node id says itself as
/// [`crate::SpokenNode::absent`] (`node <tag>`) and a stable name as
/// [`crate::SpokenName::absent`]; a text key says itself as the file
/// spells it, a JSON string.
pub(crate) trait SaidKey {
    /// Writes the key as the refusal says it.
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl SaidKey for crate::node::RecipeNodeId {
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::SpokenNode::absent(*self))
    }
}

impl SaidKey for crate::names::StableName {
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crate::SpokenName::absent(self.clone()))
    }
}

/// `text` as a JSON string, escapes and quotes included.
fn json_string(f: &mut fmt::Formatter<'_>, text: &str) -> fmt::Result {
    write!(f, "{}", serde_json::Value::String(text.to_owned()))
}

impl SaidKey for String {
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_string(f, self)
    }
}

impl SaidKey for crate::doc::ParamName {
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        json_string(f, self.as_str())
    }
}

impl SaidKey for crate::appearance::AttrKind {
    fn say(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A unit variant serializes as its name, a JSON string; the
        // `Debug` arm is that same name, for a serializer that refuses.
        match serde_json::to_value(self) {
            Ok(spelled) => write!(f, "{spelled}"),
            Err(_) => write!(f, "{self:?}"),
        }
    }
}

/// A key as [`SaidKey`] says it.
struct Said<'a, K>(&'a K);

impl<K: SaidKey> fmt::Display for Said<'_, K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.say(f)
    }
}

/// **The one duplicate-key refusal** of the format, for the strict
/// maps below and the appearance store's pair list
/// ([`super::pairs`]): the section and the key, as [`SaidKey`] says it.
pub(crate) fn duplicate_key<E: serde::de::Error>(section: &str, key: &impl SaidKey) -> E {
    E::custom(format!(
        "duplicate {section} key {} — refused, no silent last-wins",
        Said(key)
    ))
}

/// The shared strict-map visitor: refuses the first repeated key with
/// a typed message carrying the section label and the key as
/// [`SaidKey`] says it.
pub(crate) fn strict_map<'de, K, V, D>(
    de: D,
    section: &'static str,
) -> Result<BTreeMap<K, V>, D::Error>
where
    K: Deserialize<'de> + Ord + SaidKey,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct Vis<K, V> {
        section: &'static str,
        marker: PhantomData<(K, V)>,
    }
    impl<'de, K, V> Visitor<'de> for Vis<K, V>
    where
        K: Deserialize<'de> + Ord + SaidKey,
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
                    return Err(duplicate_key(self.section, &key));
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
                K: Deserialize<'de> + Ord + SaidKey,
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
    /// The A11 cluster-placement registry.
    placements,
    "placement node"
);
strict_map_section!(
    /// The node-label store.
    labels,
    "label node"
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

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use crate::appearance::AttrKind;
    use crate::node::RecipeNodeId;

    /// What `strict_map` refuses `text` with, in `section`.
    fn refusal<K>(text: &str, section: &'static str) -> String
    where
        K: serde::de::DeserializeOwned + Ord + super::SaidKey,
    {
        let mut de = serde_json::Deserializer::from_str(text);
        match super::strict_map::<K, u8, _>(&mut de, section) {
            Ok(_) => panic!("a repeated key refuses: {text}"),
            Err(e) => e.to_string(),
        }
    }

    /// A node key is said by its tag, as no document is at hand; a text
    /// key as the file spells it, escapes and all.
    #[test]
    fn a_duplicate_key_is_said_as_a_parse_can_say_it() {
        let id = RecipeNodeId(0x3fa9_c1d2_a0b1_0042);
        let said = refusal::<RecipeNodeId>(
            &format!("{{\"{0}\": 1, \"{0}\": 2}}", id.0),
            "snapshot node",
        );
        assert!(
            said.starts_with(
                "duplicate snapshot node key node 3fa9c1d2a0b1 — refused, no silent last-wins"
            ),
            "{said}"
        );
        let said = refusal::<String>(r#"{"a\"b": 1, "a\"b": 2}"#, "document metadata");
        assert!(
            said.starts_with(r#"duplicate document metadata key "a\"b" — refused"#),
            "{said}"
        );
        let said = refusal::<AttrKind>(r#"{"Color": 1, "Color": 2}"#, "appearance attribute");
        assert!(
            said.starts_with(r#"duplicate appearance attribute key "Color" — refused"#),
            "{said}"
        );
    }
}
