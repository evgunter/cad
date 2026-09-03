//! Aggregated integration-test binary for `viewer`.
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

#[path = "assembly_display.rs"]
mod assembly_display;
#[path = "assembly_walk.rs"]
mod assembly_walk;
#[path = "blend_authoring.rs"]
mod blend_authoring;
#[path = "camera_ops.rs"]
mod camera_ops;
#[path = "cascade_delete.rs"]
mod cascade_delete;
#[path = "chrome_labels.rs"]
mod chrome_labels;
#[path = "combine_ops.rs"]
mod combine_ops;
#[path = "creation_ops.rs"]
mod creation_ops;
#[path = "datum_draw.rs"]
mod datum_draw;
#[path = "display_budget.rs"]
mod display_budget;
#[path = "doc_io.rs"]
mod doc_io;
#[path = "edge_pick.rs"]
mod edge_pick;
#[path = "error_display.rs"]
mod error_display;
#[path = "eval_seam.rs"]
mod eval_seam;
#[path = "focus_highlight.rs"]
mod focus_highlight;
#[path = "frame_policy.rs"]
mod frame_policy;
#[path = "input_mapping.rs"]
mod input_mapping;
#[path = "instance_authoring.rs"]
mod instance_authoring;
#[path = "mate_tool_flow.rs"]
mod mate_tool_flow;
#[path = "panel_display.rs"]
mod panel_display;
#[path = "panel_edits.rs"]
mod panel_edits;
#[path = "path_authoring.rs"]
mod path_authoring;
#[path = "prefs.rs"]
mod prefs;
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
#[path = "review_gui4_r1.rs"]
mod review_gui4_r1;
#[path = "review_gui4_r2.rs"]
mod review_gui4_r2;
#[path = "review_m10_1_r1.rs"]
mod review_m10_1_r1;
#[path = "scene_build.rs"]
mod scene_build;
#[path = "select_pick.rs"]
mod select_pick;
#[path = "story_assembly.rs"]
mod story_assembly;
#[path = "story_authoring.rs"]
mod story_authoring;
#[path = "story_parametric.rs"]
mod story_parametric;
#[path = "theme.rs"]
mod theme;
#[path = "tree_badges.rs"]
mod tree_badges;
#[path = "tree_shape.rs"]
mod tree_shape;
#[path = "undo_tree.rs"]
mod undo_tree;
#[path = "valid_range.rs"]
mod valid_range;

/// Guards the `autotests = false` hazard: a suite file added under
/// `tests/` but not declared above would silently stop being compiled
/// and run. Both directions are asserted — every file on disk is
/// declared, and every declaration answers to a file, so no number
/// about this file is stated in prose without being computed.
///
/// The walk is `test_utils::source::suite_files`, which recurses into
/// group directories and tells a suite from a shared helper by Rust's
/// own module rule; read it before adding either.
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
