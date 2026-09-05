//! Aggregated integration-test binary for `step-import`.
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

// The shared helper trees, declared ONCE for the whole binary. This file
// is the crate root, so a plain `mod` resolves against `tests/` —
// `tests/common/mod.rs` — and every consumer
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
mod common;

#[path = "cert1_r1_import_probes.rs"]
mod cert1_r1_import_probes;
#[path = "cert5_r1_import_probes.rs"]
mod cert5_r1_import_probes;
#[path = "cert_n2r2_consumer_probes.rs"]
mod cert_n2r2_consumer_probes;
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
#[path = "onb_wild_normal_census.rs"]
mod onb_wild_normal_census;
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
#[path = "tcost_k3_import_certificate.rs"]
mod tcost_k3_import_certificate;
#[path = "tier_gate.rs"]
mod tier_gate;
#[path = "verbs_chamfer_roundtrip.rs"]
mod verbs_chamfer_roundtrip;
#[path = "wild.rs"]
mod wild;

/// The aggregation and ONE HOME checks, whose one home — the walk, the
/// three checks and the argument for each — is `test_utils::source::aggregation_violations`.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
