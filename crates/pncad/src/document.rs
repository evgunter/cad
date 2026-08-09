//! **The document layer's curated surface** (LB13).
//!
//! The façade re-exports the KERNEL crates wholesale (`pncad::topo`,
//! `pncad::sweep`, …): they are geometry, and a consumer that reaches
//! past the prelude into them finds nothing that outlives the value it
//! is holding. The document layer is different — its arena keys
//! (`EntityRef`, `EntityKey`, and the `topo` keys they wrap) are
//! body-lineage-scoped, meaningful only against the evaluation that
//! minted them, and `editor-core`'s own rule (G1) is that they never
//! leave that crate. A whole-crate re-export of `editor_core` handed
//! them out anyway, one hop past the seal LIB-U5 had just built.
//!
//! **So the document layer is exposed through THIS list and nothing
//! else.** What is here is what the façade chose to expose; what is
//! absent is absent on purpose. In particular there is no
//! `EntityRef`, no `EntityKey`, and no `Entry` — the way to a named
//! entity's geometry is [`crate::select`]'s doors (`face_frame`,
//! `edge_frame`, `vertex_position`, `denotation`), which speak names
//! and answer with values.
//!
//! The naming vocabulary proper (selectors, `StableName`, the
//! materializers, the geometry doors) lives in [`crate::select`],
//! beside the worked examples; most of the authoring types here are
//! also in [`crate::prelude`], which draws from this list so there is
//! ONE curated surface rather than two that can drift.

// The recipe and its edits.
pub use editor_core::{Doc, DocEdit, EditError, apply};

// Node vocabulary. `BooleanOp` is the DOCUMENT layer's — the recipe
// node's operation, distinct from `topo::BooleanOp`, the kernel's.
// The prelude carries the kernel's and cannot carry both under one
// name, so this module is where document-layer code spells it.
pub use editor_core::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};

// Expressions and their text door.
pub use editor_core::{Dimension, Expr, ParseError, parse_expr};

// Evaluation: the service, its options, its results, and the payloads
// a result can carry.
pub use editor_core::{
    BooleanValue, CancelToken, DatumValue, EvalOptions, Evaluation, NodeError, NodeErrorKind,
    SplitSide, ValuePayload, evaluate,
};

// The content-hashing trait a scalar must satisfy to be evaluated
// through the document layer (the memo currency's substrate).
pub use editor_core::ContentBits;

// The profile description node type and its document alias.
pub use editor_core::{DescToken, ProfileDesc, ProfileDoc};
