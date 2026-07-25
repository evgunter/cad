//! `editor-core` — the recipe substrate: `Doc` as a plain value, the v1
//! feature-node vocabulary as data, the dimension-checked expression
//! sublanguage, and the `DocEdit` vocabulary with a pure `apply`.
//!
//! Born in M4 PR 1 under the ratified M4-PLAN forks: F1 (restrictive
//! dimension lattice), F4 (node vocabulary), F7 (expression AST with no
//! conditionals — total by construction) — see `docs/M4-PR1-SPEC.md`
//! (D1–D9, binding). This crate holds NO geometry evaluation (PR 2), NO
//! persistence (PR 6), NO name resolution (PR 3/4).
//!
//! Layering (M4 PR 2 spec D1, G1): editor-core sits ABOVE the kernel —
//! the evaluation service ([`eval`]) depends on the op crates it wires
//! (`profile`, `sweep`, `topo`); the kernel crates gain no editor-core
//! dependency. Profiles are carried opaquely in the document (a type
//! parameter, never a re-model); [`ProfileDoc`] is the canonical
//! instantiation at the profile crate's public description type.

pub mod appearance;
pub mod diff;
pub mod doc;
pub mod edit;
pub mod eval;
pub mod expr;
pub mod names;
pub mod node;
pub mod profile_desc;
pub mod resolve;
pub mod witness;

pub use appearance::{
    AppearanceLoss, AppearanceLossCause, AppearanceMap, AppearanceResolution, Attr, AttrKind,
    AttrSet, Rgba8,
};
pub use diff::{DocDiff, NodeChange};
pub use doc::{Doc, DocParam, ParamName};
pub use edit::{Applied, DocEdit, EditError, EditRecord, apply};
pub use eval::{
    BooleanValue, CancelToken, ContentBits, ContentKey, DatumValue, Epoch, EvalOptions,
    EvalOutcome, Evaluation, NodeError, NodeErrorKind, NodeResult, NodeValue, SplitSide,
    ValuePayload, evaluate,
};
pub use expr::{
    Dimension, DimensionError, EvalError, Expr, ExprPath, ParamEnv, ParamValue, eval, eval_count,
};
pub use names::{
    CapEnd, EntityKey, EntityKind, EntityRef, Entry, MeridianEnd, NameTable, NamingError,
    ProfileEdgeRef, ProfileVertexRef, Qualifier, RolePath, RoleSeg, SideVerdict, SplitHalf,
    StableName,
};
pub use node::{Axis3, BooleanOp, Datum, Node, PatternKind, RecipeNodeId, SlotId};
pub use profile_desc::{ProfileDesc, ProfileDoc};
pub use resolve::{
    Diagnosis, FlipSet, HitTestError, MeshPatchKey, NodeVerdictDelta, PredicateDivergence,
    RecipeEditRef, Resolution, ResolutionFailure, ResolveError, ResolveIndeterminate, Resolved,
    RunCtx, RunStatus, TieWitness, Tombstone, VerdictFlip, appearance_rebind_suggestions,
    apply_with_names, body_name, derivation_nodes, diff_verdicts, edge_name,
    enrich_appearance_loss, enrich_appearance_loss_with_prior, entity_name, face_name,
    rebind_suggestions, resolve, resolve_with_prior, vertex_name,
};
pub use witness::{
    BifurcationKind, BranchCertification, BranchMarginEvidence, Implicated, WitnessAge,
    WitnessBifurcation, WitnessDatum,
};
