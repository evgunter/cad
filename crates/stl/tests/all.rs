//! Aggregated integration-test binary for `stl`.
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

#[path = "export.rs"]
mod export;
#[path = "issue111_az_export.rs"]
mod issue111_az_export;
#[path = "m3_pr6_exports.rs"]
mod m3_pr6_exports;
#[path = "review_m2_pr7.rs"]
mod review_m2_pr7;
#[path = "review_m3_pr55_e2e.rs"]
mod review_m3_pr55_e2e;
#[path = "review_m3_pr6_e2e.rs"]
mod review_m3_pr6_e2e;

/// Guards the `autotests = false` hazard: a suite file added to `tests/`
/// but not declared above would silently stop being compiled and run.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    // Comments blanked, string literals KEPT: the needle IS a string
    // literal, and a mount that has been commented out must not answer
    // for the file it names — that is the silent direction, and it
    // drops a whole suite from the build with the guard still green.
    let src = test_utils::source::code_and_literals(include_str!("all.rs"));
    let mut missing: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("tests/ is readable") {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let name = path
            .file_name()
            .expect("file has a name")
            .to_string_lossy()
            .to_string();
        if name == "all.rs" {
            continue;
        }
        if !src.contains(&format!("#[path = \"{name}\"]")) {
            missing.push(name);
        }
    }
    missing.sort();
    assert!(
        missing.is_empty(),
        "tests/*.rs suites are not declared in tests/all.rs, so `autotests = false` \
         is silently dropping them: {missing:?}. Add a `#[path]` line for each."
    );
}
