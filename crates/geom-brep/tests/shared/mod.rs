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
//!   says why AT the copy. **The list below is the whole of it**, so a
//!   reader can check the rule by grepping `Deliberately not` and
//!   comparing counts, rather than trusting a sample:
//!
//!   - the two `basis` seedings (`cert5_r2_probes.rs`,
//!     `review_r1_rational_probes.rs`) and `review_r1_rational_probes`'s
//!     own `eval`;
//!   - the two reviewer rebuilds of the quarter cylinder
//!     (`cert10_r1_probes.rs`, `cert10r2_probes.rs`);
//!   - the three door-driving helpers that do not go through
//!     `patch::face_posture` (`cert5_r2_probes.rs`, `r2_cert6_probes.rs`,
//!     and `cert5_r1_patch_probes.rs`'s `width`);
//!   - the five bands that are not the run's
//!     (`decoration_plane_mint.rs`, `pcurve_p1a_meter.rs` twice,
//!     `r2_probes.rs`, and one row of `pcurve_p1b_r2_probes.rs`);
//!   - the loop-edge and rim builders that are not this tree's job
//!     (`mesh10r2_probes.rs`'s `edge`, `iso_rectangle_door.rs`'s `rim`,
//!     `imported_chart_arc_rim.rs`'s and `revolved_point_anchor.rs`'s);
//!   - the surfaces that are not in the canonical frame
//!     (`review_m6_3_chart_probes.rs`, `pcurve_conic.rs`,
//!     `review_m2_pr7_props.rs`, `offset_mint.rs`,
//!     `review_m5_pr9_jet_probe.rs`'s `zcyl`);
//!   - the reviewer-pair rebuilds, which are the class this tree must
//!     never absorb even when the text is identical: `control` and
//!     `tmer` across `iso_rectangle_door.rs` / `mesh10r1_probes.rs` /
//!     `mesh10r2_probes.rs`, `wall_domain` across `m5_pr7_ssi.rs` /
//!     `review_m5_pr7b_ssi.rs`, `width`/`point_width` across
//!     `arc_eval_anchor.rs` / `review_arceval_r1_probes.rs`, and
//!     `cone_dist` across `offa_r1_probes.rs` / `offset_mint.rs`.
//!
//! The bar has a second half that only shows up on the small helpers.
//! A one-line constructor carries no derivation, so nothing about
//! sharing it can make two suites agree by construction; what it can do
//! is hide that two spellings were never the same value. So each module
//! below states, for the spellings it merged, WHY they are one value —
//! `Band::linear`'s own body for `tol`, `Real::from_f64`'s two impls
//! for `point`, `RingInterval`'s two doors for `ring` — and the
//! spellings that are a different value stay put and say so.

pub(crate) mod fixture;
#[cfg(feature = "interval")]
pub(crate) mod interval;
pub(crate) mod patch;
pub(crate) mod point;
pub(crate) mod ring;
pub(crate) mod sample;
pub(crate) mod surf;
pub(crate) mod tol;
pub(crate) mod topo;
