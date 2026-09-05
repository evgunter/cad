//! Aggregated integration-test binary for `geom-core`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! Each suite keeps its own `//!` docs and its inner attributes
//! (`#![cfg(feature = "interval")]` and friends work as module-level
//! attributes). What it does NOT keep is a `mod <helper>;` line of its
//! own: the shared helper trees are declared once, below, as modules of
//! THIS root, and a suite that wants one says `use crate::<helper>;`.
//! One declaration means one parse, one resolve, one type-check and one
//! codegen of that helper per binary instead of one per including suite.
//!
//! What that gives up: a suite file is no longer compilable as its own
//! crate root, because `crate::` now names this binary. Nothing in the
//! tree compiles them that way — `autotests = false` plus the guard below
//! make this file the only root — but it was true before and is not now.
//!
//! WHY ONE BINARY: on the CI runner (2 vCPU) the per-binary codegen+link
//! constant dominated the workspace build job — the suites are small, so
//! that constant was the bill. The figures are deliberately NOT restated
//! here: they were measured once, nothing in the repo re-takes them, and
//! the LINK/DEBUGINFO note in .github/workflows/ci.yml is the one place
//! that carries them with their date, their provenance run and the record
//! of what has since changed.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run — `every_suite_file_is_
//! aggregated` below fails loudly if you forget.
//!
//! Test IDs gain a module prefix (`export::round_trip` rather than
//! `round_trip`, under binary `all` rather than binary `export`); the set
//! of tests is otherwise identical.

// The shared helper trees, declared ONCE for the whole binary. This file
// is the crate root, so a plain `mod` resolves against `tests/` —
// `tests/span_fixtures/mod.rs` — and every consumer
// reaches that one instance through `use crate::<helper>;`.
//
// NO `#[path]` ON THESE, deliberately: a path attribute in this file is
// the aggregation guard's census of SUITE files
// (`every_suite_file_is_aggregated` counts them against the directory
// walk), and a helper module directory is not a suite. `mod` without the
// attribute is also what `test_utils::source::suite_files` assumes when
// it skips a directory carrying a `mod.rs`.
//
// There is no `#![allow(clippy::duplicate_mod)]` here because no file is
// loaded twice any more; if one ever is, the lint is meant to fire.
mod span_fixtures;

#[path = "ambiguity_k_env.rs"]
mod ambiguity_k_env;
#[path = "band_tolerance.rs"]
mod band_tolerance;
#[path = "bounds_census.rs"]
mod bounds_census;
#[path = "cert3r1_poison_detail.rs"]
mod cert3r1_poison_detail;
#[path = "cert3r1_probes.rs"]
mod cert3r1_probes;
#[path = "certified_door.rs"]
mod certified_door;
#[path = "coeffs_bit_identity.rs"]
mod coeffs_bit_identity;
#[path = "d8_knot_queries_adversarial.rs"]
mod d8_knot_queries_adversarial;
#[path = "decoration_seam.rs"]
mod decoration_seam;
#[path = "eps_provenance.rs"]
mod eps_provenance;
#[path = "flagged_census.rs"]
mod flagged_census;
#[path = "interval_band.rs"]
mod interval_band;
#[path = "k_stats_doors.rs"]
mod k_stats_doors;
#[path = "knot_queries_differential.rs"]
mod knot_queries_differential;
#[path = "m10_7_r1_retag_probe.rs"]
mod m10_7_r1_retag_probe;
#[path = "m10_7_r1_sym_probes.rs"]
mod m10_7_r1_sym_probes;
#[path = "m5_pr1_poison_conservation.rs"]
mod m5_pr1_poison_conservation;
#[path = "m5_pr7b_tensor_compose.rs"]
mod m5_pr7b_tensor_compose;
#[path = "review_m0_pr2.rs"]
mod review_m0_pr2;
#[path = "review_m0_pr3.rs"]
mod review_m0_pr3;
#[path = "review_m0_pr4.rs"]
mod review_m0_pr4;
#[path = "review_m0_pr5.rs"]
mod review_m0_pr5;
#[path = "review_m0_pr6.rs"]
mod review_m0_pr6;
#[path = "review_m2_pr7_k.rs"]
mod review_m2_pr7_k;
#[path = "review_m5_pr1_launder.rs"]
mod review_m5_pr1_launder;
#[path = "review_m5_pr2_scratch.rs"]
mod review_m5_pr2_scratch;
#[path = "review_m5_pr2_scratch_hull.rs"]
mod review_m5_pr2_scratch_hull;
#[path = "review_m5_pr7_svd.rs"]
mod review_m5_pr7_svd;
#[path = "review_m5_pr7b_tensor.rs"]
mod review_m5_pr7b_tensor;
#[path = "review_margin_probe.rs"]
mod review_margin_probe;
#[path = "ring_interval_differential.rs"]
mod ring_interval_differential;
#[path = "ring_interval_fuzz.rs"]
mod ring_interval_fuzz;
#[path = "span_basis_identity.rs"]
mod span_basis_identity;
#[path = "span_hull_window.rs"]
mod span_hull_window;
#[path = "span_newtype.rs"]
mod span_newtype;
#[path = "spline_hull.rs"]
mod spline_hull;
#[path = "tolerance_init.rs"]
mod tolerance_init;

/// The aggregation and ONE HOME checks, whose one home — the walk, the
/// three checks and the argument for each — is `test_utils::source::aggregation_violations`.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[path = "cert4r2_probes.rs"]
mod cert4r2_probes;

#[path = "period_fold_centred.rs"]
mod period_fold_centred;

#[path = "cert4r1_probe_period.rs"]
mod cert4r1_probe_period;

#[path = "r1_p2_onb_probes.rs"]
mod r1_p2_onb_probes;
#[path = "r2_cert3_probes.rs"]
mod r2_cert3_probes;

#[path = "cert3_evidence.rs"]
mod cert3_evidence;

#[path = "onb_signed_zero_evidence.rs"]
mod onb_signed_zero_evidence;

#[path = "m10_7_r2_sym_probes.rs"]
mod m10_7_r2_sym_probes;

#[path = "unit_segment.rs"]
mod unit_segment;

#[path = "props1_evidence.rs"]
mod props1_evidence;

#[path = "props1_review_rows.rs"]
mod props1_review_rows;
