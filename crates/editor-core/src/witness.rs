//! Solver CONTRACTS (M4 PR 4 spec D5; SOLVER-DESIGN W1–W9 ratified
//! #79): the opaque per-node witness datum, the certified-same-branch
//! bulk-ReWitness certification payload, and the `WitnessBifurcation`
//! typed refusal. **Types and document semantics only** — no solver
//! implementation, no constraint evaluation, no certification logic:
//! the M6 solver fills these shapes without a schema change.
//!
//! Opacity discipline (W1/W8, spec D5): the witness datum is
//! per-node and OPAQUE at the document layer — serialized bytes plus
//! a schema tag. editor-core stores it, replays it (content keys,
//! persistence), and never interprets it; a sketch node stores an ℝⁿ
//! assignment, a future mate node stores points of ∏SE(3), and the
//! document layer cannot tell the difference by construction.

use crate::names::StableName;

/// The opaque per-node witness datum (W1: the committed solved
/// assignment, recorded at the last committed sketch edit together
/// with the params it solved under — all inside `bytes`, under
/// `schema`'s vocabulary). Bytes are exact data: bit-exact
/// persistence and content-key feeding need no float policy here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WitnessDatum {
    /// The serialization vocabulary tag (versioned with the solver's
    /// contract; the M6 solver is its first author).
    pub schema: u32,
    /// The serialized witness — opaque to editor-core.
    pub bytes: Vec<u8>,
}

/// The certification payload a bulk certified-same-branch ReWitness
/// carries (W4, ratified amendment): rewitnessing is automatable in
/// bulk exactly when the W2 certificate proves old witness and new
/// solution share one uniqueness region — semantically invisible,
/// recorded, never silent. v1 carries the obligation AS DATA;
/// **enforcement arrives with the M6 solver** (a checker that
/// consumes this payload — an additive function, not a schema
/// change).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BranchCertification {
    /// The certification evidence's vocabulary tag (M6's checker owns
    /// it).
    pub schema: u32,
    /// Serialized certified-same-branch evidence for every entry of
    /// the bulk edit — opaque to editor-core.
    pub bytes: Vec<u8>,
}

/// Which way the W2 certificate refused (W3; layer-2 vocabulary of
/// the two-layer DOF diagnosis — NEVER "over/under-constrained",
/// never "didn't converge").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BifurcationKind {
    /// The interval Jacobian's regularity margin entered the sliver
    /// band: genuine bifurcation proximity (the elbow near straight).
    FoldProximity,
    /// Box inflation swallowed multiple roots before containment
    /// fired: the witness sits between well-separated basins and the
    /// selection is genuinely ambiguous.
    AmbiguousBasin,
    /// The committed assignment's residuals failed ε-certification.
    ResidualFailure,
}

/// The `solver_branch_margin` evidence (W2.3/W3): the certificate
/// outcome's certified bound against its classification band. The
/// margin and the certificate are the same computation; this is its
/// recorded shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BranchMarginEvidence {
    /// The certified regularity margin.
    pub margin: f64,
    /// The band's coincidence threshold at classification.
    pub band_zero: f64,
    /// The band's escalation threshold at classification.
    pub band_escalate: f64,
}

/// One implicated element from the near-nullspace of the interval
/// Jacobian (W3: highlightable in an editor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Implicated {
    /// A sketch entity, by stable name.
    Entity(StableName),
    /// A constraint, by its index in the owning sketch node's
    /// recipe-recorded constraint list (M6's constraint vocabulary is
    /// recipe data on the sketch node; the index is D8-structural).
    Constraint(u32),
}

/// The witness's age evidence (W3: "params the witness solved under
/// vs. params now"). Both sides are serialized under the witness
/// datum's own schema — the document layer compares them for the
/// display only bytewise, never interprets them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessAge {
    /// The params the recorded witness solved under (opaque, the
    /// witness schema's vocabulary).
    pub solved_under: Vec<u8>,
    /// The params at the refusing solve (same vocabulary).
    pub at_solve: Vec<u8>,
}

/// The typed branch-selection refusal (W3 verbatim): certificate
/// refusal is data, never a retry loop and never a panic. Reaches
/// the result DAG as a per-node failure poisoning descendants only
/// (GQ2), and NAMING-DESIGN N5's `Diagnosis::WitnessBifurcation` arm
/// carries exactly this payload when a stable-name resolution fails
/// through a sketch node. Constructed by the M6 solver; v1 pins the
/// shape and the document semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct WitnessBifurcation {
    /// Which way the certificate refused.
    pub kind: BifurcationKind,
    /// The `solver_branch_margin` evidence (certified bound).
    pub margin: BranchMarginEvidence,
    /// The implicated constraint/entity set (near-nullspace of the
    /// interval Jacobian).
    pub implicated: Vec<Implicated>,
    /// The stale-witness evidence.
    pub witness_age: WitnessAge,
}
