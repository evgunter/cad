//! The wire form of a contact class — the mate node's `class` field
//! and, through [`pairs`], a [`Node::Declare`](crate::Node) payload's
//! classes. ONE vocabulary, one wire spelling of it, one table
//! (ASM-R2a D-1).
//!
//! Lowercase because the class vocabulary was minted straight into a
//! `with` module at the v11 break and never had a derive to match; see
//! this module's parent.
//!
//! Both directions refuse typed on a spelling this build does not
//! know — including the SERIALIZE direction, which `ContactClass`
//! being `#[non_exhaustive]` makes reachable: a class added in a newer
//! kernel than this writer must never be written out under a guessed
//! tag.

use crate::names::StableName;
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serializer};
use topo::ContactClass;

/// The wire spelling of a class, or `None` if this build has no name
/// for it.
pub(crate) fn tag(class: ContactClass) -> Option<&'static str> {
    match class {
        ContactClass::Rest => Some("rest"),
        ContactClass::Tangent => Some("tangent"),
        // `ContactClass` is `#[non_exhaustive]`: a kernel newer than
        // this writer can present a class with no spelling here.
        // Refusing is the only honest answer — see the module docs.
        _ => None,
    }
}

/// The inverse of [`tag`].
pub(crate) fn untag(s: &str) -> Option<ContactClass> {
    match s {
        "rest" => Some(ContactClass::Rest),
        "tangent" => Some(ContactClass::Tangent),
        _ => None,
    }
}

/// Serializes the class as its stable lowercase spelling.
///
/// # Errors
///
/// A class with no spelling in this build refuses rather than
/// inventing one.
pub(crate) fn serialize<S: Serializer>(class: &ContactClass, ser: S) -> Result<S::Ok, S::Error> {
    let spelling = tag(*class).ok_or_else(|| {
        S::Error::custom(format!(
            "persist: this build has no wire spelling for the mate's contact class (a newer \
             kernel's vocabulary) — refusing to write a guessed tag. {}",
            topo::FIT_DEFERRAL
        ))
    })?;
    ser.serialize_str(spelling)
}

/// Reads a stable lowercase spelling back.
///
/// # Errors
///
/// An unknown spelling refuses typed, naming the deferral.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ContactClass, D::Error> {
    let spelling = String::deserialize(de)?;
    untag(&spelling).ok_or_else(|| {
        D::Error::custom(format!(
            "unknown contact class '{spelling}' — v1 mates spell `rest` and `tangent`; {}",
            topo::FIT_DEFERRAL
        ))
    })
}

/// The declare payload's `((a, b), class)` list, spelling its classes
/// with the same table as the single-class form above.
pub(crate) mod pairs {
    use super::{ContactClass, StableName, tag, untag};
    use serde::de::Error as _;
    use serde::ser::Error as _;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type Pairs = Vec<((StableName, StableName), ContactClass)>;

    /// # Errors
    ///
    /// A class with no spelling in this build refuses rather than
    /// inventing one.
    pub(crate) fn serialize<S: Serializer>(pairs: &Pairs, ser: S) -> Result<S::Ok, S::Error> {
        let mut out = Vec::with_capacity(pairs.len());
        for ((a, b), class) in pairs {
            let t = tag(*class).ok_or_else(|| {
                S::Error::custom(
                    "persist: this build has no wire spelling for the declared contact class \
                     (a newer kernel's vocabulary) — refusing to write a guessed tag",
                )
            })?;
            out.push(((a, b), t));
        }
        out.serialize(ser)
    }

    /// # Errors
    ///
    /// An unknown spelling refuses typed.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Pairs, D::Error> {
        let raw: Vec<((StableName, StableName), String)> = Vec::deserialize(de)?;
        raw.into_iter()
            .map(|(pair, t)| {
                untag(&t)
                    .map(|class| (pair, class))
                    .ok_or_else(|| D::Error::custom(format!("unknown contact class '{t}'")))
            })
            .collect()
    }
}
