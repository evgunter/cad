//! Aggregated integration-test binary for `viewer`.
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
//! WHY: on the CI runner (2 vCPU) each extra test binary cost ~1.9 s of
//! codegen+link — measured at 494 of the 514 s of the workspace build job
//! (see the LINK/DEBUGINFO note in .github/workflows/ci.yml). The suites
//! are small; the per-binary constant was the bill.
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

#[path = "assembly_display.rs"]
mod assembly_display;
#[path = "assembly_walk.rs"]
mod assembly_walk;
#[path = "camera_ops.rs"]
mod camera_ops;
#[path = "doc_io.rs"]
mod doc_io;
#[path = "eval_seam.rs"]
mod eval_seam;
#[path = "frame_policy.rs"]
mod frame_policy;
#[path = "input_mapping.rs"]
mod input_mapping;
#[path = "mate_tool_flow.rs"]
mod mate_tool_flow;
#[path = "panel_edits.rs"]
mod panel_edits;
#[path = "review_gui0_r1.rs"]
mod review_gui0_r1;
#[path = "review_gui0_r2.rs"]
mod review_gui0_r2;
#[path = "review_gui2_r1.rs"]
mod review_gui2_r1;
#[path = "review_gui2_r2.rs"]
mod review_gui2_r2;
#[path = "review_gui3_r1.rs"]
mod review_gui3_r1;
#[path = "review_gui3_r2.rs"]
mod review_gui3_r2;
#[path = "review_gui4_r2.rs"]
mod review_gui4_r2;
#[path = "scene_build.rs"]
mod scene_build;
#[path = "select_pick.rs"]
mod select_pick;
#[path = "tree_badges.rs"]
mod tree_badges;
#[path = "undo_tree.rs"]
mod undo_tree;

/// Guards the `autotests = false` hazard: a suite file added to `tests/`
/// but not declared above would silently stop being compiled and run.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let src = include_str!("all.rs");
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
