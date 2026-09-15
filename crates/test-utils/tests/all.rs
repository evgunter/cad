//! Aggregated integration-test binary for `test-utils`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! WHY ONE BINARY: each extra test target is a full codegen+link of the
//! whole rlib graph, which on the CI runner dominates the build job.
//! `scripts/gates/test-aggregation.sh` is what holds the crate to one.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in `Cargo.toml` means a file that is not
//! listed here does not compile and does not run.
//!
//! Test IDs gain a module prefix (`reader_census::…` rather than the
//! bare name, under binary `all`); the set of tests is otherwise
//! identical.

#[path = "deny_unknown_fields_census.rs"]
mod deny_unknown_fields_census;
#[path = "reader_census.rs"]
mod reader_census;

/// The aggregation and ONE HOME checks, whose one home — the walk, the
/// three checks and the argument for each — is
/// `test_utils::source::aggregation_violations`.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
