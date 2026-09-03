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
///
/// It also pins ONE HOME for every shared helper: no suite file may
/// carry a `mod <name>;` of its own. That form loads a FILE as a module
/// of the declaring suite, and in an aggregated binary the same helper
/// is then parsed, resolved, type-checked and codegen'd once per suite
/// that declares it — the cost this crate's `all.rs` header describes.
/// One declaration at the root of this file, and `use crate::<name>;`
/// in each suite, makes it one compilation for the whole binary.
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
    // ONE HOME for every shared helper, enforced over the same walk.
    // A `mod <name>;` in a SUITE loads that helper file as a module of
    // THAT suite, so inside this one binary the helper is parsed,
    // resolved, type-checked and codegen'd once per suite that declares
    // it. Declared once at the top of this file and reached with
    // `use crate::<name>;`, it is compiled once for the binary.
    // Inline `mod <name> { … }` blocks are not this and stay legal;
    // helper TREES are directories carrying a `mod.rs`, which the walk
    // above already excludes.
    let redeclared: Vec<String> = found
        .iter()
        .flat_map(|rel| {
            let text =
                std::fs::read_to_string(root.join(rel)).expect("a walked suite file reads back");
            test_utils::source::file_module_decls(&text)
                .into_iter()
                .map(move |name| format!("{rel}: mod {name};"))
        })
        .collect();
    assert!(
        redeclared.is_empty(),
        "a suite declares a module of its own, which compiles that file once per \
         declaring suite inside this one binary: {redeclared:?}. Declare it once in \
         tests/all.rs (`mod <name>;`, no `#[path]`) and say `use crate::<name>;` here."
    );
}
