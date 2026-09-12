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
//! searches the vocabulary by `tag` and the refusal messages quote the
//! same table, so the two directions cannot disagree about a spelling
//! and a message cannot go stale.
//!
//! - **The kernel**: that the search domain holds every class. The
//!   vocabulary here is [`ContactClass::ALL`], the enum's own slice —
//!   not a literal restated downstream. `#[non_exhaustive]` means this
//!   crate could not enumerate the variants correctly if it tried, and
//!   the kernel's own doc records the measurement that a downstream
//!   `[Rest, Tangent]` literal stays GREEN under a planted third
//!   variant. Reading `ALL` is what puts this module behind the
//!   exhaustive matches beside it, which is where the fence is.
//! - **NOT the compiler**: that a class reaches [`tag`]. A kernel
//!   newer than this writer can present a class this build has no
//!   spelling for, which is why `tag` refuses instead of guessing.
//! - **Therefore, at run time and fail-loud**: every write direction
//!   checks the round trip through [`spelling`] and REFUSES if the
//!   class cannot be read back, naming the omission. The unreadable
//!   file is never created. With the domain taken from the kernel this
//!   is belt-and-braces rather than the only guard, and it stays: it
//!   costs a lookup and it catches a `tag` that answers for a class
//!   the search cannot then find. That refusal has no test row and can
//!   have none — in a build whose read table is complete, nothing
//!   constructs the state it guards — exactly as the sibling
//!   `boolean_op` module's does; what the suite reaches is the admit
//!   path, through the schema round trip.

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

/// The inverse of [`tag`], DERIVED from it rather than restated, over
/// the kernel's own enumeration of the vocabulary.
pub(crate) fn untag(s: &str) -> Option<ContactClass> {
    ContactClass::ALL
        .iter()
        .copied()
        .find(|c| tag(*c) == Some(s))
}

/// The spellings this build can read, for a refusal to quote.
///
/// A class the kernel names and this build cannot spell is absent
/// here, which is the honest answer: the list is what a document may
/// contain, not what the kernel has.
fn known() -> String {
    ContactClass::ALL
        .iter()
        .filter_map(|c| tag(*c).map(|t| format!("`{t}`")))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The spelling to write for `class`, or the reason there is none —
/// the ONE write-side door, so no caller can skip the round trip.
///
/// The check is belt-and-braces over [`ContactClass::ALL`]: a class
/// this build spells but cannot read back is refused at the write
/// rather than committed to a file.
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

/// The read-side refusal, in one place: both entry points quote the
/// same table and the same deferral, so neither can go stale while the
/// other is corrected.
fn unknown(spelling: &str) -> String {
    format!(
        "unknown contact class '{spelling}' — a document spells one of {}; {}",
        known(),
        topo::FIT_DEFERRAL
    )
}

/// Reads a stable lowercase spelling back.
///
/// # Errors
///
/// An unknown spelling refuses typed, naming the deferral.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ContactClass, D::Error> {
    let spelling = String::deserialize(de)?;
    untag(&spelling).ok_or_else(|| D::Error::custom(unknown(&spelling)))
}

/// The declare payload's `((a, b), class)` list, spelling its classes
/// with the same table as the single-class form above.
pub(crate) mod pairs {
    use super::{ContactClass, StableName, spelling, unknown, untag};
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
    /// An unknown spelling refuses typed, quoting the same table and
    /// the same deferral as the single-class form.
    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Pairs, D::Error> {
        let raw: Vec<((StableName, StableName), String)> = Vec::deserialize(de)?;
        raw.into_iter()
            .map(|(pair, t)| {
                untag(&t)
                    .map(|class| (pair, class))
                    .ok_or_else(|| D::Error::custom(unknown(&t)))
            })
            .collect()
    }
}
