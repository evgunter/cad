//! Aggregated integration-test binary for `geom`.
//!
//! Every suite under `tests/` is included here VERBATIM via `#[path]`,
//! so this one binary stands in for one test target per suite. The
//! files themselves are untouched: each keeps its own `//!` docs and its
//! inner attributes (`#![cfg(feature = "interval")]` and friends work as
//! module-level attributes).
//!
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! The suites are grouped in `tests/curves/` and `tests/surfaces/`,
//! mirroring the crate's two modules — the two halves were separate
//! crates and their suite names collide (`boxes.rs`,
//! `span_window_pairing.rs`, `review_m5_pr3_attack*.rs`,
//! `decoration_ring_coords.rs`). A suite does NOT carry a
//! `mod <helper>;` line of its own: the shared helper trees are declared
//! once, below, as modules of THIS root, and a suite that wants one says
//! `use crate::<group>::<helper>;`. One declaration means one parse, one
//! resolve, one type-check and one codegen of that helper per binary
//! instead of one per including suite.
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
//! ADDING A SUITE: drop the file in `tests/curves/` or
//! `tests/surfaces/` AND add a `#[path]` line below. `autotests = false`
//! in Cargo.toml means a file that is not listed here does not compile
//! and does not run — `every_suite_file_is_aggregated` below fails
//! loudly if you forget.
//!
//! Test IDs gain a module prefix (`curves_projection::round_trip` rather
//! than `round_trip`, under binary `all` rather than binary
//! `projection`); the set of tests is otherwise identical.

// The shared helper trees, declared ONCE for the whole binary. This one
// lives INSIDE a group directory (`tests/curves/n1r2_fixtures/mod.rs`),
// so it is declared inside an inline `mod curves` block: an inline
// module extends the directory its children resolve against, which is
// what puts `tests/curves/` on the path. Every consumer reaches that one
// instance through `use crate::curves::n1r2_fixtures;`.
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
mod curves {
    pub mod n1r2_fixtures;
}

// ---- curves ----
#[path = "curves/boxes.rs"]
mod curves_boxes;
#[path = "curves/compose.rs"]
mod curves_compose;
#[path = "curves/curvo_oracle.rs"]
mod curves_curvo_oracle;
#[path = "curves/decoration_ring_coords.rs"]
mod curves_decoration_ring_coords;
#[path = "curves/fit_certify.rs"]
mod curves_fit_certify;
#[path = "curves/fitting.rs"]
mod curves_fitting;
#[path = "curves/hull_circle_rehearsal.rs"]
mod curves_hull_circle_rehearsal;
#[path = "curves/lt_r1_probes.rs"]
mod curves_lt_r1_probes;
#[path = "curves/m5_pr7_speed_meter.rs"]
mod curves_m5_pr7_speed_meter;
#[path = "curves/m8_14_long_turn_meter.rs"]
mod curves_m8_14_long_turn_meter;
#[path = "curves/n1r1_c24_dump.rs"]
mod curves_n1r1_c24_dump;
// Lane registration (aggregation guard): the R1 meter probe was pushed
// without a `#[path]` line.
#[path = "cert_n2r2_probes.rs"]
mod cert_n2r2_probes;
#[path = "curves/n1r1_c24_meter.rs"]
mod curves_n1r1_c24_meter;
#[path = "curves/n1r1_lift_probes.rs"]
mod curves_n1r1_lift_probes;
#[path = "curves/n1r2_bench.rs"]
mod curves_n1r2_bench;
#[path = "curves/n1r2_dump.rs"]
mod curves_n1r2_dump;
#[path = "curves/n1r2_lift_probes.rs"]
mod curves_n1r2_lift_probes;
#[path = "curves/n1r2_lift_probes_interval.rs"]
mod curves_n1r2_lift_probes_interval;
#[path = "curves/nurbs_differential.rs"]
mod curves_nurbs_differential;
#[path = "curves/nurbs_interval.rs"]
mod curves_nurbs_interval;
#[path = "curves/param_near.rs"]
mod curves_param_near;
#[path = "curves/param_near_interval.rs"]
mod curves_param_near_interval;
#[path = "curves/projection.rs"]
mod curves_projection;
#[path = "curves/r2_lt_probes.rs"]
mod curves_r2_lt_probes;
#[path = "curves/review_m5_pr2_e2e.rs"]
mod curves_review_m5_pr2_e2e;
#[path = "curves/review_m5_pr3_attack.rs"]
mod curves_review_m5_pr3_attack;
#[path = "curves/review_m5_pr3_attack_interval.rs"]
mod curves_review_m5_pr3_attack_interval;
#[path = "curves/review_m5_pr4_adversarial.rs"]
mod curves_review_m5_pr4_adversarial;
#[path = "curves/span_window_pairing.rs"]
mod curves_span_window_pairing;
#[path = "curves/split_at.rs"]
mod curves_split_at;
#[path = "dual_foot_tangent.rs"]
mod dual_foot_tangent;
#[path = "n2r1_probes.rs"]
mod n2r1_probes;
#[path = "net_placeholder_width.rs"]
mod net_placeholder_width;
#[path = "net_placeholder_width_interval.rs"]
mod net_placeholder_width_interval;

// ---- surfaces ----
#[path = "surfaces/boxes.rs"]
mod surfaces_boxes;
#[path = "surfaces/decoration_ring_coords.rs"]
mod surfaces_decoration_ring_coords;
#[path = "surfaces/m5_pr7_ders3.rs"]
mod surfaces_m5_pr7_ders3;
#[path = "surfaces/m5_pr7_surface_projection.rs"]
mod surfaces_m5_pr7_surface_projection;
#[path = "surfaces/nurbs_surface.rs"]
mod surfaces_nurbs_surface;
#[path = "surfaces/nurbs_surface_interval.rs"]
mod surfaces_nurbs_surface_interval;
#[path = "surfaces/review_m2_pr1.rs"]
mod surfaces_review_m2_pr1;
#[path = "surfaces/review_m2_pr1_interval.rs"]
mod surfaces_review_m2_pr1_interval;
#[path = "surfaces/review_m5_pr3_attack.rs"]
mod surfaces_review_m5_pr3_attack;
#[path = "surfaces/review_m5_pr3_attack_interval.rs"]
mod surfaces_review_m5_pr3_attack_interval;
#[path = "surfaces/s32_jet_projection.rs"]
mod surfaces_s32_jet_projection;
#[path = "surfaces/span_window_pairing.rs"]
mod surfaces_span_window_pairing;

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
