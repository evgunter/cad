//! Deterministic AABB bounding-volume hierarchy (C10, PERF-PLAN §2.1).
//!
//! One tree, several duties — **two of them wired so far**:
//!
//! - **Boolean edge×face sweep** candidate generation — LIVE since
//!   M5 PR 8 (`topo::boolean::reduce`).
//! - **Placement separation** — LIVE (`topo::separation`): the
//!   pairwise certificate that no two placed copies of a prototype
//!   can meet.
//! - **SSI seeding / C3 exhaustiveness subdivision** — INTENDED, not
//!   yet wired. `geom_brep::ssi::exhaust` still enumerates cells by
//!   recursive bisection with a linear scan over tubes, and says so
//!   ("Brute force, deliberately, for now"): this tree swaps in under
//!   that module's already-merged differential suite when profiling
//!   asks for it. Nothing in the C3 contract changes when it does.
//! - **Clearance candidate pairs at the certified scalar** — LIVE
//!   (`editor_core::clearance`): [`Bvh::build_bounded`] builds the tree
//!   over item point clouds read at `T: Bounds`, and [`Bvh::within`] and
//!   [`Bvh::pairs_within`] answer the proximity queries the E7 clearance
//!   engine subdivides from. Those three doors are the whole surface the
//!   engine uses, and the crate ships no other proximity door: a form
//!   with no consumer is not kept here on the chance one arrives. At
//!   `T = Interval` an item box encloses every real configuration in the
//!   analysis leaf's parameter box, so the candidate set is conservative
//!   over the whole box.
//!
//!   **The pruning threshold is the consumer's, and it is not the
//!   consumer's own decision threshold.** These queries drop a pair when
//!   [`Aabb::separation_lo`] exceeds the pad, on a raw comparison; a
//!   consumer whose own answers come from a tolerance band must
//!   therefore hand a pad that already carries the band, or it will have
//!   let this crate decide a case its funnel would have called
//!   indeterminate. `editor_core::clearance` pads by the funnel's
//!   escalate threshold for exactly that reason.
//! - **Viewport picking** — LIVE since GUI-1: [`Bvh::ray`], the
//!   conservative ray-slab query the editor-core hit-test service
//!   traverses (candidates ordered by conservative entry parameter;
//!   see the method's contract).
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
//! - Note vs PERF-PLAN §4.4's sketch: the realized form shipped here
//!   is a MEDIAN-SPLIT recursive build with per-node stack traversal —
//!   not §4.4's SAH build / flattened stackless traversal. The spec
//!   licenses the shipped form (C10 names the rule, not SAH); §4.4's
//!   form remains available behind the same differential suite if
//!   profiling ever demands it.
//! - Queries return candidate indices in **ascending input order** — a
//!   subsequence of the arena order, independent of tree shape.
//!
//! # Bounds scope (ratified 2026-07-29)
//!
//! This crate reads coordinate brackets (`geom_core::Bounds`) as
//! spatial-index driver code — ratified 2026-07-29, see geom-core
//! `real.rs`, Bounds scope rule.
//!
//! **`bounds-allowlist.sh` does not watch these reads**: they are SOLE
//! `T: Bounds` bounds, which is that gate's planted must-not-fire case,
//! so this crate is in none of its filters and cannot be. What watches
//! the sole form is `geom-core/tests/bounds_census.rs`, whose roster
//! carries `Aabb::from_points` with its disposition.
//!
//! The gate does see the COMPOUND form here, and `crates/bvh/` is not on
//! its allowlist, so a `T: Decide + Bounds` in this crate fires today —
//! the 2026-07-29 amendment names the crate, but a ratification in the
//! rule is not one in the allowlist.
//!
//! # The SSI-cell seam (wiring deferred, and UNSCHEDULED)
//!
//! The seam is unwired: `geom-brep` does not depend on this crate,
//! and the marcher subdivides with its own boxes. The deferral is
//! live and has no date.
//!
//! Items are addressed by dense input index, so any payload (entity
//! keys today; C3 subdivision cells carrying C9 enclosures if the seam
//! is wired) rides in a caller-side parallel array indexed the same
//! way. Cells
//! with payloads need no change here: the box tree *is* the
//! subdivision structure, the payloads live beside it.

pub mod aabb;
pub mod ray;
pub mod tree;

pub use aabb::{Aabb, Axis};
pub use ray::{Ray, RayCandidate};
pub use tree::{Bvh, LEAF_SIZE};
