//! Aggregated integration-test binary for `verbs`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite — the
//! workspace rule `scripts/gates/test-aggregation.sh` enforces, for the
//! reason `crates/topo/tests/all.rs`'s header records.
//!
//! ADDING A SUITE: drop the file in `tests/` AND add a `#[path]` line
//! below. `autotests = false` in Cargo.toml means a file that is not
//! listed here does not compile and does not run —
//! `every_suite_file_is_aggregated` below fails loudly if you forget.

#[path = "layer_guard.rs"]
mod layer_guard;
#[path = "param_flow.rs"]
mod param_flow;
#[path = "run_door.rs"]
mod run_door;

/// The aggregation's own guard: every suite file under `tests/` is
/// declared above. Without it `autotests = false` silently drops a new
/// file, and a suite that never runs is indistinguishable from one that
/// passes.
#[test]
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
