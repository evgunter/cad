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
//! **No CI grep watches these reads, and that is worth knowing rather
//! than assuming otherwise.** They are SOLE `T: Bounds` bounds
//! (`Aabb::from_points`), which is the form the scope rule prescribes and
//! the form `scripts/gates/bounds-allowlist.sh` deliberately does not
//! fire on — a sole bracket bound is its planted must-not-fire case,
//! because firing on it would red every certification file in the tree.
//! So this crate appears nowhere in that gate's filters and cannot be
//! added to them; a new sole-bound door here is invisible to CI, and the
//! ratification above is what licenses these reads, not an instrument.
//!
//! What the gate does see here is the COMPOUND form: `crates/bvh/` is not
//! on its file allowlist, so a `T: Decide + Bounds` written in this crate
//! fires today. The 2026-07-29 amendment names this crate, but that
//! ratification lives in the rule and not in the allowlist — writing a
//! compound bound here means adding the file to the gate first.
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
