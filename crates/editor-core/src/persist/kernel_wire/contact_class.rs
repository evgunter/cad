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
//!
//! # What is enforced, and what is not
//!
//! The spellings are written down exactly ONCE, in [`tag`]; [`untag`]
//! searches [`ALL`] by `tag` and the refusal messages quote the same
//! table, so the two directions cannot disagree about a spelling and a
//! message cannot go stale.
//!
//! - **NOT the compiler**: that a class reaches [`tag`]. The kernel's
//!   enum is `#[non_exhaustive]`, so this build cannot even name every
//!   class — which is why `tag` refuses instead of guessing.
//! - **NOT the compiler**: that a class reaches [`ALL`]. Safe Rust
//!   cannot tie an array literal to a variant list without a proc
//!   macro, and the workspace has none. The gap is the dangerous one:
//!   a class spelled by `tag` but absent from `ALL` would serialize
//!   fine and refuse on READ, so this build would write a file it
//!   could not open.
//! - **Therefore, at run time and fail-loud**: every write direction
//!   checks the round trip through [`spelling`] and REFUSES if the
//!   class is missing from `ALL`, naming the omission. The unreadable
//!   file is never created. That refusal has no test row and can have
//!   none — in a build whose read table is complete, nothing
//!   constructs the state it guards — exactly as the sibling
//!   `boolean_op` module's does; what the suite reaches is the admit
//!   path, through the schema round trip.

use crate::names::StableName;
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serializer};
use topo::ContactClass;

/// Every class this build can read, listed once — the table [`untag`]
/// searches and the refusals quote. A new class belongs here as well
/// as in [`tag`]; [`spelling`] refuses loudly if it reaches only one
/// of them.
const ALL: [ContactClass; 2] = [ContactClass::Rest, ContactClass::Tangent];

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

/// The inverse of [`tag`], DERIVED from it rather than restated.
pub(crate) fn untag(s: &str) -> Option<ContactClass> {
    ALL.into_iter().find(|c| tag(*c) == Some(s))
}

/// The vocabulary this build can read, for a refusal to quote.
fn known() -> String {
    ALL.into_iter()
        .filter_map(|c| tag(c).map(|t| format!("`{t}`")))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The spelling to write for `class`, or the reason there is none —
/// the ONE write-side door, so no caller can skip the round trip.
///
/// The check is what makes the `ALL`/`tag` gap fail loud: a class this
/// build spells but cannot read back is refused at the write rather
/// than committed to a file.
fn spelling(class: ContactClass) -> Result<&'static str, String> {
    let Some(t) = tag(class) else {
        return Err(format!(
            "this build has no wire spelling for the contact class (a newer kernel's \
             vocabulary) — refusing to write a guessed tag. {}",
            topo::FIT_DEFERRAL
        ));
    };
    if untag(t) != Some(class) {
        return Err(format!(
            "the contact class spelled '{t}' is missing from this build's read table ({}) — \
             refusing to write a file this build could not open",
            known()
        ));
    }
    Ok(t)
}

/// Serializes the class as its stable lowercase spelling.
///
/// # Errors
///
/// A class with no spelling in this build, or none this build could
/// read back, refuses rather than inventing one.
pub(crate) fn serialize<S: Serializer>(class: &ContactClass, ser: S) -> Result<S::Ok, S::Error> {
    let t = spelling(*class).map_err(|why| S::Error::custom(format!("persist: {why}")))?;
    ser.serialize_str(t)
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
            "unknown contact class '{spelling}' — a document spells one of {}; {}",
            known(),
            topo::FIT_DEFERRAL
        ))
    })
}

/// The declare payload's `((a, b), class)` list, spelling its classes
/// with the same table as the single-class form above.
pub(crate) mod pairs {
    use super::{ContactClass, StableName, spelling, untag};
    use serde::de::Error as _;
    use serde::ser::Error as _;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    type Pairs = Vec<((StableName, StableName), ContactClass)>;

    /// # Errors
    ///
    /// A class with no spelling in this build, or none this build
    /// could read back, refuses rather than inventing one — the same
    /// write-side door the single-class form uses.
    pub(crate) fn serialize<S: Serializer>(pairs: &Pairs, ser: S) -> Result<S::Ok, S::Error> {
        let mut out = Vec::with_capacity(pairs.len());
        for ((a, b), class) in pairs {
            let t = spelling(*class).map_err(|why| {
                S::Error::custom(format!("persist: declared contact class: {why}"))
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
