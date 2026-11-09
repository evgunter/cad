//! Aggregated integration-test binary for `source-scope`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite
//! (`scripts/gates/test-aggregation.sh` holds the crate to one).
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below; `autotests = false` means an unlisted file does not run.

#[path = "scopes.rs"]
mod scopes;

test_utils::every_suite_file_is_aggregated!();
