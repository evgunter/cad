//! `MetaValue` — the format's own self-describing value tree (M4 PR 6
//! spec D7, Ev's #92 ask banked at the schema-v1 freeze).
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
//!
//! **A value nests at most [`MAX_NESTING`] levels**, and the bound is
//! the type's own: a list or a map is built only through
//! [`MetaValue::list`] and [`MetaValue::map`] (or [`to_value`], or a
//! deserializer), each of which refuses past it with
//! [`MetaError::NestedTooDeep`]. So every walk over a value that
//! recurses once per level, the derived ones and `Drop` included, fits
//! the smallest stack a door runs on, and the load door reads a body as
//! deep as a value at the bound saves (`persist::nesting`).

mod de;
mod ser;

pub use de::from_value;
pub use ser::to_value;

use std::collections::BTreeMap;

/// **How deep a [`MetaValue`] may nest**: the longest chain of values
/// from the root to a leaf, both ends included (a leaf, or an empty list
/// or map, is 1). Every door that builds a list or a map refuses past it
/// ([`MetaError::NestedTooDeep`]).
pub const MAX_NESTING: usize = 128;

const _: () = assert!(
    MAX_NESTING <= u8::MAX as usize,
    "a value's nesting is stored in one byte"
);

/// The children of a list or a map, with how deep the value holding
/// them nests. Built only by [`MetaValue::list`] and [`MetaValue::map`],
/// which refuse past [`MAX_NESTING`], so the nesting it carries is
/// never above the bound. Read the children through `Deref`.
#[derive(Clone)]
pub struct Nested<C> {
    children: C,
    /// The holding value's nesting, itself included.
    nesting: u8,
}

/// A list's items.
pub type MetaList = Nested<Vec<MetaValue>>;

/// A map's entries.
pub type MetaMap = Nested<BTreeMap<String, MetaValue>>;

impl<C> Nested<C> {
    /// `children` one level below their holder, whose deepest child
    /// nests `below` levels; refused past [`MAX_NESTING`].
    fn over(children: C, below: usize) -> Result<Self, MetaError> {
        match u8::try_from(below + 1) {
            Ok(nesting) if usize::from(nesting) <= MAX_NESTING => Ok(Self { children, nesting }),
            _ => Err(MetaError::NestedTooDeep { bound: MAX_NESTING }),
        }
    }

    /// The children, to change and build again.
    #[must_use]
    pub fn into_inner(self) -> C {
        self.children
    }
}

/// The deepest nesting among `values`, 0 when there are none.
fn deepest<'a>(values: impl Iterator<Item = &'a MetaValue>) -> usize {
    values.map(MetaValue::nesting).max().unwrap_or(0)
}

impl<C> core::ops::Deref for Nested<C> {
    type Target = C;
    fn deref(&self) -> &C {
        &self.children
    }
}

/// The children alone: the nesting is a function of them.
impl<C: core::fmt::Debug> core::fmt::Debug for Nested<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            children,
            nesting: _,
        } = self;
        children.fmt(f)
    }
}

/// The children alone: the nesting is a function of them.
impl<C: PartialEq> PartialEq for Nested<C> {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            children,
            nesting: _,
        } = self;
        *children == other.children
    }
}

impl<C: serde::Serialize> serde::Serialize for Nested<C> {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.children.serialize(ser)
    }
}

/// A reader's refusal of a value past the bound: the problem alone,
/// since the door reading it states its own recourse.
fn read_refusal<E: serde::de::Error>(error: MetaError) -> E {
    match error {
        MetaError::NestedTooDeep { bound } => E::custom(TooDeep(bound)),
        other => E::custom(other),
    }
}

impl<'de> serde::Deserialize<'de> for MetaList {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let items = Vec::<MetaValue>::deserialize(de)?;
        let below = deepest(items.iter());
        Nested::over(items, below).map_err(read_refusal)
    }
}

impl<'de> serde::Deserialize<'de> for MetaMap {
    /// Refuses a repeated key (`persist::strict`) as well as the bound.
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let entries: BTreeMap<String, MetaValue> =
            crate::persist::strict::strict_map(de, "metadata map")?;
        let below = deepest(entries.values());
        Nested::over(entries, below).map_err(read_refusal)
    }
}

/// The self-describing metadata value tree (spec D7). See the module
/// docs for the contract; construct a leaf directly, a list or a map
/// through [`MetaValue::list`] and [`MetaValue::map`], or the whole
/// tree via [`to_value`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    List(MetaList),
    /// A string-keyed map (canonical key order by construction;
    /// duplicate keys refuse typed on load — no silent last-wins).
    Map(MetaMap),
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
            // Different variants are unequal — spelled over the whole
            // vocabulary rather than swept up by a catch-all, so a
            // value kind added to `MetaValue` must be given its own
            // arm above instead of silently comparing unequal to
            // itself, which would break the reflexivity `Eq` below
            // promises.
            (Null | Bool(_) | Int(_) | Float(_) | Str(_) | Bytes(_) | List(_) | Map(_), _) => false,
        }
    }
}

/// Bit-equality on floats is a true equivalence (NaN never compares —
/// by bits a NaN equals itself), so `Eq` holds.
impl Eq for MetaValue {}

impl MetaValue {
    /// A list of `items`.
    ///
    /// # Errors
    ///
    /// [`MetaError::NestedTooDeep`] when the list would nest past
    /// [`MAX_NESTING`].
    pub fn list(items: Vec<MetaValue>) -> Result<Self, MetaError> {
        let below = deepest(items.iter());
        Nested::over(items, below).map(Self::List)
    }

    /// A map of `entries`.
    ///
    /// # Errors
    ///
    /// [`MetaError::NestedTooDeep`] when the map would nest past
    /// [`MAX_NESTING`].
    pub fn map(entries: BTreeMap<String, MetaValue>) -> Result<Self, MetaError> {
        let below = deepest(entries.values());
        Nested::over(entries, below).map(Self::Map)
    }

    /// How many levels the value nests, itself included (a leaf is 1);
    /// never above [`MAX_NESTING`].
    #[must_use]
    pub fn nesting(&self) -> usize {
        match self {
            Self::List(Nested { nesting, .. }) | Self::Map(Nested { nesting, .. }) => {
                usize::from(*nesting)
            }
            Self::Null
            | Self::Bool(_)
            | Self::Int(_)
            | Self::Float(_)
            | Self::Str(_)
            | Self::Bytes(_) => 1,
        }
    }

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
                // The leaves that carry no float and nest no value.
                // Spelled out rather than swept up: a value kind
                // added to `MetaValue` that carries a float, or
                // nests values that might, would otherwise be
                // walked past and the D2 refusal door would admit
                // the non-finite it exists to refuse.
                MetaValue::Null
                | MetaValue::Bool(_)
                | MetaValue::Int(_)
                | MetaValue::Str(_)
                | MetaValue::Bytes(_) => false,
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

impl std::fmt::Display for MetaVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAMap => f.write_str("the stored value is not a map"),
            Self::MissingVersion => f.write_str("the map has no \"v\" entry"),
            Self::VersionNotInt => f.write_str("the \"v\" entry is not an integer"),
        }
    }
}

impl std::error::Error for MetaVersionError {}

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
    /// A producer map serialized the same key twice — refused (the
    /// erased tree is canonical; silent last-wins would drop data).
    DuplicateKey(String),
    /// The value would nest deeper than a value may
    /// ([`MAX_NESTING`]).
    NestedTooDeep {
        /// The deepest a value may nest, in levels.
        bound: usize,
    },
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
            Self::DuplicateKey(k) => {
                write!(
                    f,
                    "duplicate map key {k:?} refused at the metadata boundary"
                )
            }
            Self::NestedTooDeep { bound } => write!(
                f,
                "{}. Recourse: store it flatter, keeping a deep part as a string or as bytes",
                TooDeep(*bound)
            ),
            Self::Message(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for MetaError {}

/// [`MetaError::NestedTooDeep`]'s problem without its recourse: what the
/// load door reads, which states a recourse of its own.
struct TooDeep(usize);

impl std::fmt::Display for TooDeep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the metadata value nests deeper than {} levels of lists and maps",
            self.0
        )
    }
}

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// A stand-in GUI/tooling producer type (D7's serde-native
    /// ergonomics): derives Serialize/Deserialize, converts at the
    /// store boundary, carries its own "v".
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Annotation {
        v: i64,
        label: String,
        offset: [f64; 2],
        pinned: bool,
        color: Option<u8>,
        #[serde(with = "serde_bytes_shim")]
        raw: Vec<u8>,
    }

    /// Minimal serialize_bytes shim (serde derives Vec<u8> as a seq;
    /// the BYTES path needs an explicit call — same as serde_bytes).
    mod serde_bytes_shim {
        pub(super) fn serialize<S: serde::Serializer>(b: &[u8], s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(b)
        }
        pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
            d: D,
        ) -> Result<Vec<u8>, D::Error> {
            struct V;
            impl serde::de::Visitor<'_> for V {
                type Value = Vec<u8>;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("bytes")
                }
                fn visit_bytes<E>(self, v: &[u8]) -> Result<Vec<u8>, E> {
                    Ok(v.to_vec())
                }
            }
            d.deserialize_bytes(V)
        }
    }

    fn ann() -> Annotation {
        Annotation {
            v: 3,
            label: "hole ⌀".into(),
            offset: [-0.0, 1.5e-300],
            pinned: true,
            color: None,
            raw: vec![0, 255, 7],
        }
    }

    #[test]
    fn producer_round_trips_through_the_erased_tree() {
        let tree = to_value(&ann()).expect("to_value");
        // The erased shape is the canonical vocabulary: a Map with
        // exact ints, bit-exact floats, real Bytes, Null for None.
        let MetaValue::Map(m) = &tree else {
            panic!("struct erases to Map")
        };
        assert_eq!(m["v"], MetaValue::Int(3));
        assert_eq!(m["color"], MetaValue::Null);
        assert_eq!(m["raw"], MetaValue::Bytes(vec![0, 255, 7]));
        let MetaValue::List(off) = &m["offset"] else {
            panic!("array erases to List")
        };
        assert_eq!(off[0], MetaValue::Float(-0.0)); // bit-eq: sign kept
        tree.require_versioned().expect("carries v");
        let back: Annotation = from_value(&tree).expect("from_value");
        assert_eq!(back, ann());
    }

    #[test]
    fn boundary_refusals_are_typed() {
        assert_eq!(to_value(&f64::NAN), Err(MetaError::NonFinite));
        assert_eq!(to_value(&u64::MAX), Err(MetaError::IntOutOfRange));
        let int_keys: std::collections::BTreeMap<u32, u32> = [(1, 2)].into();
        assert_eq!(to_value(&int_keys), Err(MetaError::NonStringKey));
        // Duplicate producer map keys refuse (save/load symmetry at
        // the producer boundary — from_value's tree cannot even
        // represent a duplicate).
        let dup_keys: Vec<(&str, u8)> = vec![("k", 1), ("k", 2)];
        struct AsMap(Vec<(&'static str, u8)>);
        impl serde::Serialize for AsMap {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap as _;
                let mut m = s.serialize_map(Some(self.0.len()))?;
                for (k, v) in &self.0 {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
        }
        let dup_keys = AsMap(dup_keys);
        assert_eq!(
            to_value(&dup_keys),
            Err(MetaError::DuplicateKey("k".into()))
        );
        assert!(MetaValue::Int(1).require_versioned().is_err());
        assert!(
            MetaValue::map([("v".to_owned(), MetaValue::Str("x".into()))].into())
                .unwrap()
                .require_versioned()
                .is_err()
        );
    }

    #[test]
    fn enums_erase_with_external_tags_and_come_back() {
        #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        enum Marker {
            Plain,
            At(f64),
            Rect { w: f64, h: f64 },
        }
        for m in [
            Marker::Plain,
            Marker::At(2.5),
            Marker::Rect { w: 1.0, h: -0.0 },
        ] {
            let tree = to_value(&m).expect("to_value");
            let back: Marker = from_value(&tree).expect("from_value");
            assert_eq!(back, m);
        }
        assert_eq!(
            to_value(&Marker::Plain).unwrap(),
            MetaValue::Str("Plain".into())
        );
    }

    #[test]
    fn first_non_finite_names_the_path() {
        let inner =
            MetaValue::map([("b".to_owned(), MetaValue::Float(f64::INFINITY))].into()).unwrap();
        let list = MetaValue::list(vec![MetaValue::Float(1.0), inner]).unwrap();
        let tree = MetaValue::map([("a".to_owned(), list)].into()).unwrap();
        assert_eq!(tree.first_non_finite().as_deref(), Some("$.a[1].b"));
        assert_eq!(MetaValue::Float(1.0).first_non_finite(), None);
    }
}
