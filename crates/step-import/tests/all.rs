//! Aggregated integration-test binary for `step-import`.
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
//! WHY: each extra test binary is a whole codegen+link of the ~40-rlib
//! graph, re-monomorphizing the kernel's generic `T: Real` code into its
//! own compilation unit. #179 collapsed the rest of the workspace this way
//! (249 targets -> 12); `step-import` was missed and kept its 26, which was
//! two thirds of every remaining test target in the workspace.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run — `every_suite_file_is_
//! aggregated` below fails loudly if you forget.
//!
//! Test IDs gain a module prefix (`parser::round_trip` rather than
//! `round_trip`, under binary `all` rather than binary `parser`); the set
//! of tests is otherwise identical.

// Each suite keeps its own verbatim `mod common;`, so the shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

#[path = "cert1_r1_import_probes.rs"]
mod cert1_r1_import_probes;
#[path = "cert5_r1_import_probes.rs"]
mod cert5_r1_import_probes;
#[path = "corpus_fold.rs"]
mod corpus_fold;
#[path = "curve_promotion_report.rs"]
mod curve_promotion_report;
#[path = "freecad.rs"]
mod freecad;
#[path = "halfcap_pole.rs"]
mod halfcap_pole;

#[path = "inst_review_probes.rs"]
mod inst_review_probes;
#[path = "mesh8r2_probes.rs"]
mod mesh8r2_probes;
#[path = "nurbs_import.rs"]
mod nurbs_import;
#[path = "p1b_r1_import_scan.rs"]
mod p1b_r1_import_scan;
#[path = "parser.rs"]
mod parser;

#[path = "poleguard.rs"]
mod poleguard;
#[path = "probe_dup.rs"]
mod probe_dup;
#[path = "probe_eps.rs"]
mod probe_eps;
#[path = "probe_fuzz.rs"]
mod probe_fuzz;
#[path = "probe_knot.rs"]
mod probe_knot;
#[path = "probe_mirror.rs"]
mod probe_mirror;
#[path = "probe_outer.rs"]
mod probe_outer;
#[path = "probe_refusals.rs"]
mod probe_refusals;
#[path = "probe_review.rs"]
mod probe_review;
#[path = "probe_sense.rs"]
mod probe_sense;
#[path = "probe_vol.rs"]
mod probe_vol;
#[path = "r1_dm1_probe.rs"]
mod r1_dm1_probe;
#[path = "r2_import_door.rs"]
mod r2_import_door;
#[path = "recognize_pins.rs"]
mod recognize_pins;
#[path = "rev_import_probe.rs"]
mod rev_import_probe;
#[path = "review_band_probes.rs"]
mod review_band_probes;
#[path = "review_k3_probe.rs"]
mod review_k3_probe;
#[path = "review_probes.rs"]
mod review_probes;
#[path = "review_probes_m7_3.rs"]
mod review_probes_m7_3;
#[path = "review_r1_tier_gate_probes.rs"]
mod review_r1_tier_gate_probes;
#[path = "roundtrip.rs"]
mod roundtrip;
#[path = "rw2_probes.rs"]
mod rw2_probes;
#[path = "s58_iso_rectangle.rs"]
mod s58_iso_rectangle;
#[path = "split_iso_side.rs"]
mod split_iso_side;
#[path = "tier_gate.rs"]
mod tier_gate;
#[path = "verbs_chamfer_roundtrip.rs"]
mod verbs_chamfer_roundtrip;
#[path = "wild.rs"]
mod wild;

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
