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
//! Layering (spec D1): the only dependency is `geom-core`, for the
//! scalar-generic [`geom_core::Real`] the expression evaluator is
//! parameterized over. Profiles are carried opaquely (a type parameter,
//! never a re-model); the kernel op crates join behind PR 2's
//! evaluation service.

pub mod diff;
pub mod doc;
pub mod edit;
pub mod expr;
pub mod node;

pub use diff::{DocDiff, NodeChange};
pub use doc::{Doc, DocParam, ParamName};
pub use edit::{Applied, DocEdit, EditError, EditRecord, apply};
pub use expr::{
    Dimension, DimensionError, EvalError, Expr, ExprPath, ParamEnv, ParamValue, eval, eval_count,
};
pub use node::{
    Axis3, BooleanOp, CapEnd, Datum, EntityKind, Node, PatternKind, RecipeNodeId, RoleSeg, SlotId,
    StableName,
};
