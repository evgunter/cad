//! **The Python word for a recipe node's KIND**, and the one
//! exhaustive match that mints it.
//!
//! `Doc.node_kind` is the read half of the `Node.*` constructor
//! family: a Python row that authored a recipe can ask what kind of
//! node an id holds without evaluating the document and without
//! reading its saved text.
//!
//! # Why the match is the mechanism
//!
//! The vocabulary below is drawn from ONE `match` over the kernel's
//! [`Node`] with **no wildcard arm**. A node kind added kernel-side
//! and given no Python word therefore does not compile, which is the
//! same device [`crate::surface_census`] uses one layer up: the
//! witness is a match on the kernel tag, never a hand-kept list that
//! compiles green while short.
//!
//! # The words are not the wire's words
//!
//! The saved text spells a node by its Rust variant identifier
//! (`"PlacedUnion"`), because that is serde's rendering of the enum
//! and it belongs to the persistence format's own compatibility
//! contract. The words here are snake_case, the convention every
//! other stable string this crate publishes already follows
//! ([`crate::tags`], `Value.kind`). The two vocabularies are
//! deliberately separate: a wire spelling may not move without a
//! format decision, and a Python word may not move without a public
//! API decision, and neither should be able to drag the other.
//!
//! # A node's kind is not its value's kind
//!
//! An extrude, a transform and a placed union all evaluate to a value
//! whose `kind` is `"body"` — that tag is the PAYLOAD's shape. Telling
//! the recipes apart is what this vocabulary is for.

use pncad::document::{BooleanOp, Node};

/// The stable Python word for `node`'s kind.
///
/// One word per [`Node`] variant, except [`Node::Boolean`], which
/// answers a word per OPERATION: union, intersect and subtract are
/// three kernel operations sharing one payload shape, and a caller
/// asking "does this recipe spend a subtract" is asking about the
/// operation. The unprefixed `union` is the different node — the
/// n-ary one that folds a member list.
///
/// The full vocabulary is pinned by
/// `the_node_kind_vocabulary_matches_its_committed_roster` in
/// `src/tests.rs` and listed for callers in `pncad.pyi`.
pub fn node_kind<P>(node: &Node<P>) -> &'static str {
    match node {
        Node::Datum(_) => "datum",
        Node::Profile(_) => "profile",
        Node::Extrude { .. } => "extrude",
        Node::Revolve { .. } => "revolve",
        Node::Tube { .. } => "tube",
        Node::HollowTube { .. } => "hollow_tube",
        Node::Loft { .. } => "loft",
        Node::Sweep { .. } => "sweep",
        Node::Fillet { .. } => "fillet",
        Node::Chamfer { .. } => "chamfer",
        Node::Split { .. } => "split",
        Node::Boolean { op, .. } => match op {
            BooleanOp::Union => "boolean_union",
            BooleanOp::Intersect => "boolean_intersect",
            BooleanOp::Subtract => "boolean_subtract",
        },
        Node::Union { .. } => "union",
        Node::Transform { .. } => "transform",
        Node::Pattern { .. } => "pattern",
        Node::Part { .. } => "part",
        Node::PlacedUnion { .. } => "placed_union",
        Node::Declare { .. } => "declare",
        Node::InstantiatePart { .. } => "instantiate_part",
        Node::Mate { .. } => "mate",
        Node::Measure { .. } => "measure",
        Node::Assertion { .. } => "assertion",
    }
}
