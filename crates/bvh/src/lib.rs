//! Deterministic AABB bounding-volume hierarchy (C10, PERF-PLAN §2.1).
//!
//! One tree, several duties: the boolean edge×face sweep's candidate
//! generation (first consumer, M5 PR 8), SSI seeding and the C3
//! exhaustiveness subdivision (PR 7's wiring), viewport picking later.
//!
//! # The conservative-superset contract (load-bearing)
//!
//! A query against this tree may only PRUNE pairs whose padded boxes
//! definitely do not interact; every pair the exact predicates would
//! accept MUST survive candidate generation. The tree therefore
//! **decides nothing semantic**: no Q1 predicate runs inside it, box
//! tests are raw `f64` comparisons on bounds that are conservative *by
//! construction* (each [`Aabb`] certifies containment of its entity's
//! true locus — see [`Aabb`]'s containment contract), and a poisoned
//! (NaN) box can never prove disjointness, so it is never pruned.
//! Results downstream remain a function of exact tests only (D9): the
//! tree changes *which pairs are examined*, never what any predicate
//! answers, and the idealized/realized differential suite (PERF-PLAN
//! §4.4) pins exactly that — realized candidates ⊇ idealized accepted
//! pairs, final results bit-equal.
//!
//! # Determinism (D9)
//!
//! - **Arena-order build**: input order is the iteration order; the
//!   item index handed back by queries is the input index. No hash
//!   iteration anywhere; no parallel build in v1.
//! - **Fixed split rule with total tie-breaks**, documented at
//!   [`Bvh::build`]: median split on the longest axis of the centroid
//!   bounds; axis ties break to the lower axis index (X < Y < Z);
//!   centroid ties break to the lower input index. Every comparison is
//!   IEEE `total_cmp` — total even on NaN.
//! - **Fixed leaf constant** [`LEAF_SIZE`].
//! - Queries return candidate indices in **ascending input order** — a
//!   subsequence of the arena order, independent of tree shape.
//!
//! # The SSI-cell seam (PR 7, wiring deferred — API must not preclude)
//!
//! Items are addressed by dense input index, so any payload (entity
//! keys today; C3 subdivision cells carrying C9 enclosures at PR 7)
//! rides in a caller-side parallel array indexed the same way. Cells
//! with payloads need no change here: the box tree *is* the
//! subdivision structure, the payloads live beside it.

pub mod aabb;
pub mod tree;

pub use aabb::{Aabb, Axis};
pub use tree::{Bvh, LEAF_SIZE};
