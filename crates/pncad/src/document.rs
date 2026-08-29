//! **The document layer's curated surface**.
//!
//! The façade re-exports the KERNEL crates wholesale (`pncad::topo`,
//! `pncad::sweep`, …): they are geometry, and a consumer that reaches
//! past the prelude into them finds nothing that outlives the value it
//! is holding. The document layer is different — its arena keys
//! (`EntityRef`, `EntityKey`, and the `topo` keys they wrap) are
//! body-lineage-scoped, meaningful only against the evaluation that
//! minted them, and `editor-core`'s own rule is that they never
//! leave that crate. A whole-crate re-export of `editor_core` would
//! hand them out anyway, one hop past the seal.
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
// one in a typed field rather than only destructure it.
pub use editor_core::{Applied, Doc, DocEdit, EditError, EditRecord, apply};

// Node vocabulary. `BooleanOp` is the KERNEL's, which the recipe node
// carries directly; it is re-exported here so document-layer code can
// spell the whole node vocabulary through one module.
pub use editor_core::{
    Axis3, BooleanOp, Datum, Node, PatternKind, PlacementRuleFault, RecipeNodeId, SlotId,
};

// Expressions and their text door.
// `ParamEnv` joins them because `select_where` takes one, so a
// caller who cannot spell the type cannot call the door.
// `DimensionError` is the refusal `Expr`'s constructor doors return
// (`literal`, the operator builders) — re-exported so a caller can
// MATCH on it rather than pre-check the conditions it refuses.
pub use editor_core::{Dimension, DimensionError, Expr, ParamEnv, ParseError, parse_expr};

// The expression READ side: an expression's current value under a
// document's parameter environment (`Doc::param_env`). A panel that
// shows a slot before editing it needs this — `Expr::literal_value`
// answers only for a bare literal, and a slot driven by
// `width/2 - margin` has a value the consumer otherwise cannot obtain
// without re-implementing the evaluator. `EvalError` rides along so a
// slot whose value cannot be computed says which parameter is missing
// rather than displaying a blank.
//
// Reached through the `expr` module path rather than through
// `editor_core::eval`, which names BOTH the evaluation module and this
// function: a bare `pub use editor_core::eval` would re-export the
// module too, opening a second door onto the layer this list exists to
// curate.
pub use editor_core::expr::{EvalError, eval, eval_count};

// Named document parameters.
// `ParamName` is a parameter's name — a plain string newtype — and
// `DocParam` its declared dimension plus exact stored value: recipe
// vocabulary, plain values, no arena key anywhere in either. They
// complete doors this module already carried: `DocEdit::SetDocParam`
// takes both and `Expr::param` takes a `ParamName`, so without them
// the parametric flagship (`plate_param`, guide §3.2) could not be
// authored façade-only.
// `DocParamValue` is the value half of one, and the reason it is
// curated is the door it opens: `DocEdit::SetDocParamValue` writes a
// new number into an already-declared parameter and carries the whole
// declaration — dimension AND distribution — forward. Rebuilding a
// `DocParam` from `(dim, value)` to move a value is the natural
// spelling and it silently DELETES an annotation, because
// `SetDocParam` is create-or-replace; a façade that curated only the
// deleting door would be handing every caller that trap.
pub use editor_core::{DocParam, DocParamValue, ParamName};

// A parameter's optional uncertainty (ERROR-DESIGN E1/E2), and the
// typed refusals its invariants raise at the edit and persistence
// doors. It rides on `DocParam::Continuous`, so a façade that can
// author a parameter but not annotate one could not express an
// error-analysis document at all; `DistributionFault` is what
// `EditError::InvalidDistribution` and `PersistError::Distribution`
// carry, so a caller diagnosing a refusal needs it too. Reading a
// distribution back is `analysis`'s door, not this one.
pub use editor_core::{Distribution, DistributionFault, DistributionField};

// Evaluation: the service, its options, its results, and the payloads
// a result can carry. `NodeResult`/`NodeValue`/`EvalOutcome` complete
// the result vocabulary: `Evaluation::result` and
// `Evaluation::node_error` answer in these types, so failed and
// poisoned nodes are typed data, not a collapsed `None`. The
// detect/declare protocol leans on the same door: the boolean's
// undeclared-coincidence
// REFUSAL is the detect/declare protocol's trigger, and
// `NodeError`/`NodeErrorKind` were unreachable without the result
// enum that carries them.
pub use editor_core::{
    BooleanValue, CancelToken, DatumValue, EvalOptions, EvalOutcome, Evaluation, NodeError,
    NodeErrorKind, NodeResult, NodeValue, SplitSide, ValuePayload, evaluate,
};

// Persistence: the schema-v4 doors, verbatim.
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

// Document identity and content pins.
// `DocumentId` answers "which part" (authored at construction —
// `DocumentId::derive` for deterministic callers, this crate's
// `workspace::random_document_id` for interactive authoring);
// `ContentPin` answers "which version" (SHA-256 of the canonical
// semantic bytes); `DocRef` pairs them — the value cross-document
// references carry. `canonical_bytes`/`content_pin` are the pin
// doors; `header_document_id` is the workspace scan's cheap header
// read. The store itself lives in [`crate::workspace`].
pub use editor_core::{
    ContentPin, DocRef, DocumentId, canonical_bytes, content_pin, header_document_id,
};

// The content-hashing trait a scalar must satisfy to be evaluated
// through the document layer (the memo currency's substrate).
pub use editor_core::ContentBits;

// Explicit product roots: the ordered root list is read through
// `Doc::roots` and set through
// `DocEdit::SetRoots`; `product` is the whole-document gather those
// roots name, and `RootFault` is the shared invariant refusal both
// the edit and persistence doors carry.
pub use editor_core::{ProductError, RootFault, product};

// Instantiated parts. `Frame` is the cluster placement a document
// records per instantiate node
// (read through `Doc::placement`, written by `DocEdit::SetPlacement`);
// `PartResolver` is the document seam evaluation crosses to reach a
// referenced document, `ResolveFailure`/`ResolveFault` its classified
// refusal, and `PartFault` the evaluation-side cause an
// `InstantiatePart` node reports. `product_named` is the gather that
// carries the product's stable names — what an instance's own names
// are minted from.
pub use editor_core::{
    Frame, PartFault, PartResolver, ResolveFailure, ResolveFault, product_named,
};

// Mates: the declaration node's
// authored payload (`Alignment` over two `MateFrame`s, a
// `MatePrimitive`, an `AxisSense`), the solve's per-node outcome
// (`SolvedPoses`, `MateRole`, the residual `Subgroup`), the recorded
// cluster-record maintenance (`ClusterMaintenance`), and `MateFault`
// — the typed refusal every door carries, the way `RootFault` is
// carried above.
pub use editor_core::{
    Alignment, AxisSense, ClusterMaintenance, MateFault, MateFrame, MatePrimitive, MateRole,
    MateSide, SolvedPoses, Subgroup, UNDER_RECOURSE, clusters, gauge_of, reading_edges,
    relative_freedom_components, solve_document,
};

// The class-admission table (`ClassAdmission`, read through
// `class_admission`, with `CLASS_DEFERRAL` as the deferral sentence its
// refusals cite): HOW FAR each contact class gets in v1, as one value
// both enforcement doors read. A mate-authoring consumer needs it
// BEFORE committing — the table is what says a class will refuse at
// the solve or mint door, so exposing it here is what lets a tool
// offer only what the vocabulary can execute instead of discovering
// the refusal after the edit lands.
pub use editor_core::{CLASS_DEFERRAL, ClassAdmission, class_admission};

// **The assembly at-rest gate** (A5): `assemble` gathers a document's
// product, mints every solved mate's declaration into its contact
// record set, and runs the kernel's own tier-3′ door over the two
// together — the answer to "is this assembly valid at rest", which the
// authoring vocabulary above can otherwise construct and not check.
// `Assembly` is the validated result (body, names, certified records,
// and one `MintedDeclaration` per mate); `AssemblyError` is the typed
// refusal, and its arms are not interchangeable — a caller must tell a
// verdict AGAINST the document (`AtRest`) from the declared
// direction's frontier (`Uncertified`), which is what `AtRestFinding`
// and `Attribution` carry per finding. `RefusedRef` says why a mate
// reference named no product face.
pub use editor_core::{
    Assembly, AssemblyError, AtRestFinding, Attribution, MintedDeclaration, RefusedRef, assemble,
};

// Split and inline: the first-class
// recorded refactorings. `split` cuts a closed node set out into a new
// document (identity supplied by the caller — `DocumentId::derive` or
// `workspace::random_document_id`) and leaves an instance behind;
// `inline` splices a referenced document back in through a
// `PartResolver`. Both return the new document VALUES plus the
// ordinary recorded edits producing them; persistence of the results
// is the workspace write side (`workspace::Workspace::create` /
// `resave`). `InterfaceRecord`/`InterfaceCrossing` are the split
// seam's crossing-declaration record, inhabited by
// `InterfaceCrossing::Mate`.
pub use editor_core::{
    InlineError, InlineOutcome, InterfaceCrossing, InterfaceRecord, NodeMap, SplitError,
    SplitOutcome, inline, split,
};

// The pin-update door. `DocEdit`'s
// `UpdateReference` arm is the per-reference primitive;
// `update_references` is the whole-document ELABORATION over it,
// returning the ordinary edits and applying none of them (purity =
// atomicity), and `UpdateError` is its typed refusal. `mixed_pins`
// is the multiplicity LINT — a report, never a gate:
// one entry per referenced id carrying more than one pin
// (`PinMultiplicity`), each pin listed with the nodes holding it
// (`PinSites`). The store-facing convenience that computes the new
// pin from disk is `workspace::update_to_store`.
pub use editor_core::{PinMultiplicity, PinSites, UpdateError, mixed_pins, update_references};

// The advisory-check registry (DISCIPLINES-DESIGN DS6) and its first
// resident, the connectedness check. `run_checks` REPORTS, never
// gates (the `mixed_pins` posture: nothing calls it from apply, load,
// or evaluation); `enforce_checks` is the one refusing path, and the
// CALLER chooses where to gate on it. Deliberately NOT in the prelude
// (prelude membership is corpus-measured).
// `subject_body` resolves a finding's (root, output_ix) attribution
// back to the flagged body in the same evaluation.
pub use editor_core::{
    Advisory, CheckEvidence, CheckFinding, CheckId, CheckKind, CheckRefusal, ChecksConfig,
    ChecksError, ChecksReport, Severity, enforce_checks, run_checks, subject_body,
};

// The profile description node type and its document alias.
pub use editor_core::{
    LoopProgram, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecordedProgramError, StepArg,
};
