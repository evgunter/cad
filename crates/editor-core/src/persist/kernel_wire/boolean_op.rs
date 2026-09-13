//! The wire form of a [`Node::Boolean`](crate::Node)'s operation.
//!
//! The spellings are written down exactly ONCE, in [`tag`]. Everything
//! else here is derived from it: [`untag`] searches `BooleanOp::ALL` by
//! `tag`, and the refusal message lists the same operations — so the
//! two directions cannot disagree about a spelling, and the message
//! cannot go stale.
//!
//! # What is enforced, and what is not
//!
//! - **The compiler**: every operation has a spelling. `tag`'s match
//!   is exhaustive over a closed enum, so an operation added to the
//!   kernel breaks this build until it is given one.
//! - **The compiler**: the read direction cannot drift from the write
//!   direction, because it does not restate it — it calls it.
//! - **The kernel, at the declaration**: which operations exist is
//!   `BooleanOp::ALL`'s to say, not this module's. Safe Rust cannot
//!   tie an array literal to a variant list, so a list here would be a
//!   second census of another crate's enum with nothing reading the
//!   two against each other; the kernel's own list at least sits beside
//!   the declaration, under the census row that forces an author adding
//!   an operation to visit it.
//! - **NOT the compiler**: that two operations do not share a
//!   spelling. `tag`'s strings are hand-written and nothing makes them
//!   distinct; a collision resolves READS to whichever operation
//!   `untag` reaches first, so this build would write a file it opens
//!   as a different operation.
//! - **NOT the compiler, and not that census either**: that every
//!   operation reaches `BooleanOp::ALL`. That row forces the visit, not
//!   the edit (`topo::boolean`'s `all_is_every_operation` says how it
//!   falls short), so an operation absent from the list is still
//!   reachable here — it would serialize fine and refuse on READ.
//! - **Therefore, at run time and fail-loud**: [`serialize`] checks the
//!   round trip before writing and REFUSES when the operation does not
//!   come back as itself, naming what it came back as. The unreadable
//!   file is never created. **This refusal has no test row and can have
//!   none**: in a build whose read table is complete nothing constructs
//!   the state it guards, which `boolean_op_wire.rs`'s header states at
//!   its lines 15-21 — what the suite reaches is the admit path.

use super::super::super::node::BooleanOp;
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serializer};

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

/// The inverse of [`tag`], DERIVED from it rather than restated, over
/// the kernel's own enumeration of the operations.
fn untag(spelling: &str) -> Option<BooleanOp> {
    BooleanOp::ALL
        .iter()
        .copied()
        .find(|op| tag(*op) == spelling)
}

/// The vocabulary this build can read, for a refusal to quote.
fn known() -> String {
    BooleanOp::ALL
        .iter()
        .map(|op| format!("`{}`", tag(*op)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// # Errors
///
/// An operation that [`untag`] does not read back as itself refuses
/// rather than writing a file this build could not open — see the
/// module docs.
pub(crate) fn serialize<S: Serializer>(op: &BooleanOp, ser: S) -> Result<S::Ok, S::Error> {
    let spelling = tag(*op);
    let read_back = untag(spelling);
    if read_back != Some(*op) {
        // Both faults the round trip can catch reach this sentence, so
        // it names what came back rather than assuming which one it
        // was: an operation the read table cannot produce at all, and a
        // spelling shared with another operation, which the table
        // produces as that other one.
        let back = match read_back {
            Some(other) => format!("the operation {other:?}"),
            None => "no operation at all".to_owned(),
        };
        return Err(S::Error::custom(format!(
            "persist: the boolean operation {:?} spells '{spelling}', which this build reads \
             back as {back} — refusing to write a file it could not open as the operation it \
             wrote (this build reads {})",
            *op,
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
