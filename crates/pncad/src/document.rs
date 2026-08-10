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

// The recipe and its edits. `Applied` is `apply`'s return (the new
// document plus its `EditRecord`) — re-exported so a caller can STORE
// one in a typed field rather than only destructure it (LIB-DOORS F4;
// the #234 usable-but-unnameable residue class).
pub use editor_core::{Applied, Doc, DocEdit, EditError, EditRecord, apply};

// Node vocabulary. `BooleanOp` is the DOCUMENT layer's — the recipe
// node's operation, distinct from `topo::BooleanOp`, the kernel's.
// The prelude carries the kernel's and cannot carry both under one
// name, so this module is where document-layer code spells it.
pub use editor_core::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};

// Expressions and their text door.
// `ParamEnv` joins them for LIB-SEL1: `select_where` takes one, so a
// caller who cannot spell the type cannot call the door.
// `DimensionError` is the refusal `Expr`'s constructor doors return
// (`literal`, the operator builders) — re-exported so a caller can
// MATCH on it rather than pre-check the conditions it refuses
// (LIB-DOORS F5).
pub use editor_core::{Dimension, DimensionError, Expr, ParamEnv, ParseError, parse_expr};

// Evaluation: the service, its options, its results, and the payloads
// a result can carry. `NodeResult`/`NodeValue`/`EvalOutcome` complete
// the result vocabulary (LIB-DOORS F3/F4): `Evaluation::result` and
// `Evaluation::node_error` answer in these types, so failed and
// poisoned nodes are typed data, not a collapsed `None`. LIB-SEL2
// leans on the same door: the boolean's undeclared-coincidence
// REFUSAL is the detect/declare protocol's trigger, and
// `NodeError`/`NodeErrorKind` were unreachable without the result
// enum that carries them.
pub use editor_core::{
    BooleanValue, CancelToken, DatumValue, EvalOptions, EvalOutcome, Evaluation, NodeError,
    NodeErrorKind, NodeResult, NodeValue, SplitSide, ValuePayload, evaluate,
};

// Persistence (LIB-DOORS F1): the schema-v4 doors, verbatim.
// `save`/`load` speak `ProfileDoc` + `DocEdit` — exactly this module's
// vocabulary — and every refusal is a typed `PersistError`, whose
// payload types ride along so each arm is matchable from here
// (`MigrationError` lives one path deeper in `editor_core`; the others
// are crate-root re-exports there).
//
// Deliberately ABSENT: `MigrationStep`, the migration-table entry
// type. Its signature speaks `serde_json::Value`, which does not cross
// the curated surface (the U9S backlog measurement); the migration
// TABLE is persist's interior, and a consumer never installs a step.
pub use editor_core::persist::MigrationError;
pub use editor_core::{
    Loaded, NonFiniteSite, PersistError, ProgramFault, REGENERATE_RECOURSE, SCHEMA_VERSION,
    SnapshotError, load, save,
};

// The content-hashing trait a scalar must satisfy to be evaluated
// through the document layer (the memo currency's substrate).
pub use editor_core::ContentBits;

// The profile description node type and its document alias.
pub use editor_core::{
    LoopProgram, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, StepArg,
};
