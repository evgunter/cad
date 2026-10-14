//! The wire form of a [`Node::Extrude`](crate::Node)'s side.
//!
//! A vocabulary with no bytes before it, so lowercase (the parent's
//! house style). The spellings are written once, in [`tag`]; [`untag`]
//! searches the kernel's own `ExtrudeSide::ALL` by it, and the refusal
//! quotes the same table. `tag`'s match is exhaustive, so a side added
//! to the kernel breaks this build until it is spelled; that two sides
//! do not share a spelling is not the compiler's, so [`serialize`]
//! checks the round trip before writing, as its sibling modules do.

use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serializer};
use sweep::ExtrudeSide;

/// The stable wire spelling of a side — the ONE place it is written.
fn tag(side: ExtrudeSide) -> &'static str {
    match side {
        ExtrudeSide::Along => "along",
        ExtrudeSide::Against => "against",
    }
}

/// The inverse of [`tag`], derived from it over the kernel's sides.
fn untag(spelling: &str) -> Option<ExtrudeSide> {
    ExtrudeSide::ALL
        .into_iter()
        .find(|side| tag(*side) == spelling)
}

/// The vocabulary this build can read, for a refusal to quote.
fn known() -> String {
    ExtrudeSide::ALL
        .iter()
        .map(|side| format!("`{}`", tag(*side)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// # Errors
///
/// A side that [`untag`] does not read back as itself refuses rather
/// than writing a file this build could not open.
pub(crate) fn serialize<S: Serializer>(side: &ExtrudeSide, ser: S) -> Result<S::Ok, S::Error> {
    let spelling = tag(*side);
    if untag(spelling) != Some(*side) {
        return Err(S::Error::custom(format!(
            "persist: the extrude side {side:?} spells '{spelling}', which this build does not \
             read back as that side — refusing to write a file it could not open as the side \
             it wrote (this build reads {})",
            known()
        )));
    }
    ser.serialize_str(spelling)
}

/// # Errors
///
/// A spelling this build has no side for refuses TYPED.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<ExtrudeSide, D::Error> {
    let spelling = String::deserialize(de)?;
    untag(&spelling).ok_or_else(|| {
        D::Error::custom(format!(
            "unknown extrude side '{spelling}' — a document spells one of {}",
            known()
        ))
    })
}
