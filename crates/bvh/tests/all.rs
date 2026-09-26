//! Aggregated integration-test binary for `bvh`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! The files themselves are untouched: each keeps its own `//!` docs and its
//! inner attributes (`#![cfg(feature = "probe")]` and friends work as
//! module-level attributes). The shared helper tree `tests/common/` is
//! declared ONCE, below, as a module of THIS root, and a suite that wants
//! it says `use crate::common::…;` — one declaration is one compile of it
//! per binary rather than one per including suite.
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

// The shared helper tree. No `#[path]` on it, deliberately: a path
// attribute in this file is the aggregation guard's census of SUITE
// files, and a helper directory carrying a `mod.rs` is not a suite.
mod common;

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

test_utils::every_suite_file_is_aggregated!();
