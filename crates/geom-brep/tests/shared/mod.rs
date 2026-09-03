//! Helpers that more than one suite of this crate's `all` binary uses.
//!
//! Declared ONCE in `tests/all.rs` as `mod shared;`, so the code below
//! is parsed, resolved, type-checked and codegen'd once for the whole
//! binary; a suite that wants a piece of it says
//! `use crate::shared::<module>;`. Nothing here is a suite — the
//! aggregation guard's directory walk skips a directory carrying a
//! `mod.rs`, which is why this is a directory and not a flat
//! `tests/shared.rs`.
//!
//! **The bar for putting something here is that two suites were
//! building the same thing, not that two suites need a thing of the
//! same shape.** Several probe suites in this crate exist to check the
//! kernel against a truth derived without it, and two such derivations
//! sharing one routine would be one derivation wearing two names. So
//! the line is drawn inside those oracles rather than around them:
//! `patch` holds the quadrature loop and the derivative recurrence,
//! each taking the caller's OWN basis or evaluator, and the seeding
//! that makes a derivation independent stays with the suite.
//!
//! Two rules follow, and both are checkable by reading:
//!
//! - each module below says which of its neighbours it deliberately
//!   did not absorb;
//! - every suite that keeps its own copy of something this tree holds
//!   says why AT the copy — the two `basis` seedings
//!   (`cert5_r2_probes.rs`, `review_r1_rational_probes.rs`), the two
//!   reviewer rebuilds of the quarter cylinder (`cert10_r1_probes.rs`,
//!   `cert10r2_probes.rs`), and the three door-driving helpers that do
//!   not go through `patch::face_posture` (`cert5_r2_probes.rs`,
//!   `r2_cert6_probes.rs`, and `cert5_r1_patch_probes.rs`'s `width`).
//!
//! Families this tree does not hold at ALL — the `band()`
//! constructors, the `edge`/`great` topology builders, the
//! `p3`/`v3`/`p` point constructors — are not reasoned about at each
//! site; they are a unit of their own (proposed as TCOST-8) and are
//! listed in this PR's body, not restated here.

pub(crate) mod fixture;
pub(crate) mod patch;
pub(crate) mod sample;
