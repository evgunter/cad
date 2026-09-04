//! **The boolean's correspondence** — the first two-operand verb, in
//! the same per-instance-data pattern as the blends'.
//!
//! # What it declares, and what stays upstairs
//!
//! The boolean's document semantics are RICHER than a blend's, and the
//! semantic content stays where it is authored — in the lowering, not
//! restated here:
//!
//! - **Two operand inputs** (`a`, `b`), each a node whose evaluated
//!   body AND name table the lowering reads — the arity the kernel
//!   declaration states (`verbs::VerbKind::arity`).
//! - **The `declare` input**: an optional `Node::Declare` operand
//!   whose name pairs resolve through the OPERANDS' name tables into
//!   the kernel's `BooleanDeclarations` (`resolve_declarations`, the
//!   N5 ladder — resolution failures are the typed trio, never a
//!   silent drop). Name resolution is the document's semantics, so it
//!   runs upstairs; what this correspondence's `build` receives is the
//!   already-lowered arena-key form.
//! - **The declared-contact carry**: the surviving contacts come back
//!   in the verb's record channel and the lowering carries them into
//!   the boolean VALUE (`BooleanValue::Body { contacts, .. }`), the
//!   tier-3′ currency downstream ops re-enter.
//!
//! What is declared HERE is the correspondence proper: how the node's
//! structural payload becomes the kernel verb (`build`), and which
//! emitter mints names from the birth record (`emitter`). There is no
//! slot↔parameter row because the boolean has no scalar slots — the
//! same fact its empty `param_flow` declares kernel-side.
//!
//! # Every field is direct per-instance data
//!
//! As in the blends' module: function pointers and literals per
//! instance, no match over a verb vocabulary anywhere in this file, so
//! a future verb never has to open it.

use std::sync::Arc;

use geom_core::{Decide, Tol};
use topo::{Body, BooleanDeclarations, BooleanNaming, BooleanOp};
use verbs::Verb;

use crate::names::{self, NameTable, NamingError};
use crate::node::RecipeNodeId;

/// A two-operand verb's naming emitter: this node's id, the result
/// body, the birth record, and the two operands' naming contexts, in.
pub(crate) type PairEmitter<T> = fn(
    RecipeNodeId,
    &Body<T>,
    &BooleanNaming,
    &names::OperandCtx<'_, T>,
    &names::OperandCtx<'_, T>,
    Tol,
) -> Result<Arc<NameTable>, NamingError>;

/// **The boolean's correspondence**, as data — everything the
/// two-operand lowering needs to turn a `Node::Boolean` into a
/// [`Verb`] and its result into a name table. Adding a field here is
/// how a pair verb declares something the lowering must know.
pub(crate) struct PairVerb<T: Decide> {
    /// **Structural payload + resolved declarations → the kernel
    /// verb.** The one place a document's op selector and lowered
    /// declaration set become a [`Verb`] payload, per instance.
    pub(crate) build: fn(BooleanOp, BooleanDeclarations) -> Verb<T>,
    /// This verb's naming emitter.
    pub(crate) emitter: PairEmitter<T>,
    /// What a WRONG-FAMILY record is called when this verb's result
    /// arrives carrying another family's channel — the blends'
    /// `foreign_record` class exactly: a kernel bug surfaced typed,
    /// unreachable while the doors and the correspondences agree.
    pub(crate) foreign_record: &'static str,
}

/// The boolean's kernel payload. A named function rather than a
/// closure so it can be a plain `fn` pointer in the struct above.
fn build_boolean<T>(op: BooleanOp, declare: BooleanDeclarations) -> Verb<T> {
    Verb::Boolean { op, declare }
}

/// The boolean's correspondence.
///
/// A function rather than a `const` for the same reason the blends'
/// are: the struct is generic in the lane scalar and a module-level
/// const cannot be; the call is monomorphized and inlined.
pub(crate) fn boolean<T: Decide>() -> PairVerb<T> {
    PairVerb {
        build: build_boolean,
        emitter: names::name_boolean,
        foreign_record: "the boolean returned a record that is not a boolean's",
    }
}

#[cfg(test)]
mod tests {
    use verbs::VerbKind;

    use super::*;

    /// The correspondence builds the verb its name claims, for every
    /// regularized op — the structural check the blends' module makes
    /// for its pair.
    #[test]
    fn the_correspondence_builds_the_boolean() {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let v: Verb<f64> = (boolean::<f64>().build)(op, BooleanDeclarations::none());
            assert_eq!(
                v.kind(),
                VerbKind::Boolean(op),
                "the boolean correspondence built a {v:?}"
            );
        }
    }

    /// The wrong-family sentence names its door (the blends' module
    /// pins the same property for its pair by distinctness; with one
    /// instance here, naming the door IS the property — it is what
    /// makes the refusal attributable when the unreachable-by-
    /// construction case ever fires).
    #[test]
    fn the_foreign_record_sentence_names_the_door() {
        assert!(
            boolean::<f64>().foreign_record.contains("boolean"),
            "the wrong-family sentence no longer names the boolean"
        );
    }
}
