//! Aggregated integration-test binary for `verbs`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite — the
//! workspace rule `scripts/gates/test-aggregation.sh` enforces, for the
//! reason `crates/topo/tests/all.rs`'s header records.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run —
//! `every_suite_file_is_aggregated` below fails loudly if you forget.

// The shared fixture tree, declared ONCE for the whole binary. NO
// `#[path]` on it, deliberately: a path attribute in this file is the
// aggregation guard's census of SUITE files, and a helper module
// directory is not a suite.
mod fixture;

#[path = "layer_guard.rs"]
mod layer_guard;
#[path = "param_flow.rs"]
mod param_flow;
#[path = "run_door.rs"]
mod run_door;

/// The aggregation and ONE HOME checks, whose one home — the walk, the
/// three checks and the argument for each — is `test_utils::source::aggregation_violations`.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
