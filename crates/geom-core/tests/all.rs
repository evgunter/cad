//! Aggregated integration-test binary for `geom-core`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! The files themselves are untouched: each keeps its own `//!` docs, its inner
//! attributes (`#![cfg(feature = "interval")]` and friends work as
//! module-level attributes), and its own `mod <helper>;` lines — a
//! `#[path]` module's child modules resolve against the DIRECTORY
//! CONTAINING the path file, i.e. `tests/`, exactly as when each file was
//! its own crate root.
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

// Each suite keeps its own verbatim `mod <helper>;`, so a shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

#[path = "ambiguity_k_env.rs"]
mod ambiguity_k_env;
#[path = "band_tolerance.rs"]
mod band_tolerance;
#[path = "cert3r1_poison_detail.rs"]
mod cert3r1_poison_detail;
#[path = "cert3r1_probes.rs"]
mod cert3r1_probes;
#[path = "certified_door.rs"]
mod certified_door;
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

/// Guards the `autotests = false` hazard: a suite file added under
/// `tests/` but not declared above would silently stop being compiled
/// and run. Both directions are asserted — every file on disk is
/// declared, and every declaration answers to a file, so no number
/// about this file is stated in prose without being computed.
///
/// The walk is `test_utils::source::suite_files`, which recurses into
/// group directories and tells a suite from a shared helper by Rust's
/// own module rule; read it before adding either.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    // Comments blanked, string literals KEPT — see
    // `test_utils::source::code_and_literals`, which states why.
    let src = test_utils::source::code_and_literals(include_str!("all.rs"));
    let found = test_utils::source::suite_files(&root);
    let missing: Vec<&String> = found
        .iter()
        .filter(|rel| !src.contains(&format!("#[path = \"{rel}\"]")))
        .collect();
    assert!(
        missing.is_empty(),
        "suites under tests/ are not declared in tests/all.rs, so `autotests = false` \
         is silently dropping them: {missing:?}. Add a `#[path]` line for each."
    );
    // The converse, computed rather than restated: one `#[path]` line
    // per suite file, no orphan declaration. The `format!` above spells
    // its quote ESCAPED, so it is not one of these matches.
    let declared = src.matches("#[path = \"").count();
    assert_eq!(
        declared,
        found.len(),
        "tests/all.rs declares {declared} suites but {} suite files exist under tests/",
        found.len()
    );
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
