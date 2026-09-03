//! Aggregated integration-test binary for `bvh`.
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

#[path = "aggregator_headers.rs"]
mod aggregator_headers;
#[path = "determinism.rs"]
mod determinism;
#[path = "proximity.rs"]
mod proximity;
#[path = "proximity_r2.rs"]
mod proximity_r2;
#[path = "ray.rs"]
mod ray;
#[path = "ray_r2.rs"]
mod ray_r2;
#[path = "review_gui1_r1.rs"]
mod review_gui1_r1;

/// Guards the `autotests = false` hazard and the ONE HOME rule in one
/// call: every suite file under `tests/` is mounted above, every mount
/// answers to a file, and no suite declares a module of its own.
///
/// The three checks, their messages and the walk that feeds them live
/// in `test_utils::source::aggregation_violations` — once, for every
/// crate that carries this row. Read it before adding a suite or a
/// shared helper.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
