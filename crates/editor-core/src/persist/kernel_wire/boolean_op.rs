//! The wire form of a [`Node::Boolean`](crate::Node)'s operation.
//!
//! The spellings are written down exactly ONCE, in [`tag`]. Everything
//! else here is derived from it: [`untag`] searches [`ALL`] by `tag`,
//! and the refusal message lists the same table — so the two
//! directions cannot disagree about a spelling, and the message cannot
//! go stale.
//!
//! # What is enforced, and what is not
//!
//! - **The compiler**: every operation has a spelling. `tag`'s match
//!   is exhaustive over a closed enum, so an operation added to the
//!   kernel breaks this build until it is given one.
//! - **The compiler**: the read direction cannot drift from the write
//!   direction, because it does not restate it — it calls it.
//! - **NOT the compiler**: that a new operation reaches `ALL`. Safe
//!   Rust has no way to tie an array literal to a variant list without
//!   a proc macro, and the workspace has none. The gap is real and it
//!   is the dangerous one: an operation absent from `ALL` would
//!   serialize fine and refuse on READ, so this build would write a
//!   file it could not open.
//! - **Therefore, at run time and fail-loud**: [`serialize`] checks the
//!   round trip before writing and REFUSES if the operation is missing
//!   from `ALL`, naming the omission. The unreadable file is never
//!   created. Pinned by test.

use super::super::super::node::BooleanOp;
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serializer};

/// Every operation, listed once — the table [`untag`] searches and the
/// refusal message quotes. A new operation belongs here as well as in
/// [`tag`]; `serialize` refuses loudly if it reaches only one of them.
const ALL: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// The stable wire spelling of an operation — the ONE place a spelling
/// is written down.
///
/// Capitalised because these bytes replaced a `#[derive(Serialize)]`
/// and must reproduce it exactly; see this module's parent.
fn tag(op: BooleanOp) -> &'static str {
    match op {
        BooleanOp::Union => "Union",
        BooleanOp::Intersect => "Intersect",
        BooleanOp::Subtract => "Subtract",
    }
}

/// The inverse of [`tag`], DERIVED from it rather than restated.
fn untag(spelling: &str) -> Option<BooleanOp> {
    ALL.into_iter().find(|op| tag(*op) == spelling)
}

/// The vocabulary this build can read, for a refusal to quote.
fn known() -> String {
    ALL.map(|op| format!("`{}`", tag(op))).join(", ")
}

/// # Errors
///
/// An operation that [`untag`] cannot read back refuses rather than
/// writing a file this build could not open — see the module docs.
pub(crate) fn serialize<S: Serializer>(op: &BooleanOp, ser: S) -> Result<S::Ok, S::Error> {
    let spelling = tag(*op);
    if untag(spelling) != Some(*op) {
        return Err(S::Error::custom(format!(
            "persist: the boolean operation spelled '{spelling}' is missing from this build's \
             read table ({}) — refusing to write a file this build could not open",
            known()
        )));
    }
    ser.serialize_str(spelling)
}

/// # Errors
///
/// A spelling this build has no operation for refuses TYPED rather
/// than resolving to some other operation.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<BooleanOp, D::Error> {
    let spelling = String::deserialize(de)?;
    untag(&spelling).ok_or_else(|| {
        D::Error::custom(format!(
            "unknown boolean operation '{spelling}' — a document spells one of {}",
            known()
        ))
    })
}
