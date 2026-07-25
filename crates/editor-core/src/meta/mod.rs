//! `MetaValue` — the format's own self-describing value tree (M4 PR 6
//! spec D7, Evan's #92 ask banked at the schema-v1 freeze).
//!
//! The appearance record carries `metadata: BTreeMap<String,
//! MetaValue>`; the kernel NEVER interprets it (black-box for
//! GUI/tooling), and any loader round-trips unknown metadata
//! structurally — pass-through interop. That is exactly why the tree
//! is the format's own vocabulary (null/bool/int/float/string/bytes/
//! list/map) and not a generic `M` parameter or a dyn registry: serde
//! needs the concrete type at decode time, so either of those would
//! make one tool's types part of the file format.
//!
//! Producer ergonomics are serde-native (D7 as RULED 2026-07-25,
//! superseding the earlier bytes ruling): a producer type derives
//! `Serialize`/`Deserialize` and converts at the store boundary with
//! [`to_value`]/[`from_value`] — typed where the type is known, erased
//! at the format boundary. Producer convention REQUIRED (enforced at
//! the edit door, [`MetaValue::require_versioned`]): each stored value
//! is a map carrying a `"v": <integer>` version field (the
//! `WitnessDatum.schema` discipline); typed views live in the layer
//! owning the key namespace.
//!
//! Equality is STRUCTURAL on the canonical tree ([`PartialEq`] below
//! compares floats BY BITS — F3 `bit_eq`); floats obey D2 (persisted
//! Ryu-canonical; NaN/inf refused at the doors, `-0.0` is data);
//! `BTreeMap` gives canonical key order.

mod de;
mod ser;

pub use de::from_value;
pub use ser::to_value;

use std::collections::BTreeMap;

/// The self-describing metadata value tree (spec D7). See the module
/// docs for the contract; construct directly or via [`to_value`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum MetaValue {
    /// Absence-as-data.
    Null,
    /// A boolean.
    Bool(bool),
    /// An exact integer.
    Int(i64),
    /// A float (D2 semantics: bit-exact, `-0.0` preserved; NaN/inf
    /// refused at the edit and persist doors — see
    /// [`MetaValue::first_non_finite`]).
    Float(f64),
    /// A UTF-8 string.
    Str(String),
    /// Opaque bytes; persists as a hex string.
    Bytes(#[serde(with = "crate::persist::hexbytes")] Vec<u8>),
    /// An ordered list.
    List(Vec<MetaValue>),
    /// A string-keyed map (canonical key order by construction).
    Map(BTreeMap<String, MetaValue>),
}

impl PartialEq for MetaValue {
    /// Structural equality on the canonical tree; floats compare BY
    /// BITS (`0.0 ≠ -0.0` — F3 `bit_eq`, spec D7).
    fn eq(&self, other: &Self) -> bool {
        use MetaValue::*;
        match (self, other) {
            (Null, Null) => true,
            (Bool(a), Bool(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (Float(a), Float(b)) => a.to_bits() == b.to_bits(),
            (Str(a), Str(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Map(a), Map(b)) => a == b,
            _ => false,
        }
    }
}

/// Bit-equality on floats is a true equivalence (NaN never compares —
/// by bits a NaN equals itself), so `Eq` holds.
impl Eq for MetaValue {}

impl MetaValue {
    /// The path (dot/index notation from the value root) of the first
    /// non-finite float in the tree, or `None` when every float is
    /// finite — the D2 refusal door's diagnostic.
    pub fn first_non_finite(&self) -> Option<String> {
        fn walk(v: &MetaValue, path: &mut String) -> bool {
            match v {
                MetaValue::Float(f) => !f.is_finite(),
                MetaValue::List(items) => items.iter().enumerate().any(|(i, item)| {
                    let mark = path.len();
                    path.push_str(&format!("[{i}]"));
                    walk(item, path) || {
                        path.truncate(mark);
                        false
                    }
                }),
                MetaValue::Map(entries) => entries.iter().any(|(k, item)| {
                    let mark = path.len();
                    path.push_str(&format!(".{k}"));
                    walk(item, path) || {
                        path.truncate(mark);
                        false
                    }
                }),
                _ => false,
            }
        }
        let mut path = String::from("$");
        walk(self, &mut path).then_some(path)
    }

    /// Enforces the D7 producer convention at the store boundary: the
    /// stored value must be a map carrying an integer `"v"` field.
    /// Structural enforcement only — the kernel checks the SHAPE and
    /// never reads the version's meaning.
    pub fn require_versioned(&self) -> Result<(), MetaVersionError> {
        let MetaValue::Map(entries) = self else {
            return Err(MetaVersionError::NotAMap);
        };
        match entries.get("v") {
            Some(MetaValue::Int(_)) => Ok(()),
            Some(_) => Err(MetaVersionError::VersionNotInt),
            None => Err(MetaVersionError::MissingVersion),
        }
    }
}

/// Typed refusal of the D7 producer convention (`"v"` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetaVersionError {
    /// The stored value is not a map.
    NotAMap,
    /// The map has no `"v"` entry.
    MissingVersion,
    /// The `"v"` entry is not an integer.
    VersionNotInt,
}

/// Typed refusal from the producer boundary ([`to_value`] /
/// [`from_value`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaError {
    /// The producer value serialized an integer outside `i64`
    /// (`u64`/`u128`/`i128` overflow) — `Int` is exact `i64`.
    IntOutOfRange,
    /// The producer value contained a non-finite float (D2: NaN/inf
    /// refused at the boundary, never stored).
    NonFinite,
    /// A map key was not a string — `Map` keys are strings.
    NonStringKey,
    /// A serde-reported error (producer `Serialize`/`Deserialize`
    /// impls surface their own messages here).
    Message(String),
}

impl std::fmt::Display for MetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntOutOfRange => write!(f, "integer out of i64 range for MetaValue::Int"),
            Self::NonFinite => write!(f, "non-finite float refused at the metadata boundary"),
            Self::NonStringKey => write!(f, "non-string map key refused at the metadata boundary"),
            Self::Message(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for MetaError {}

impl serde::ser::Error for MetaError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl serde::de::Error for MetaError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
