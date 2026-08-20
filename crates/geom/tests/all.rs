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
//! `decoration_ring_coords.rs`). A `#[path]` module's child modules
//! resolve against the DIRECTORY CONTAINING the path file, so a suite
//! that grows a `mod <helper>;` resolves it inside its own group
//! directory.
//!
//! WHY ONE BINARY: on the CI runner (2 vCPU) each extra test binary cost
//! ~1.9 s of codegen+link — measured at 494 of the 514 s of the
//! workspace build job (see the LINK/DEBUGINFO note in
//! .github/workflows/ci.yml). The suites are small; the per-binary
//! constant was the bill.
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

// Each suite keeps its own verbatim `mod <helper>;`, so a shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

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
#[path = "curves/nurbs_differential.rs"]
mod curves_nurbs_differential;
#[path = "curves/nurbs_interval.rs"]
mod curves_nurbs_interval;
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
#[path = "surfaces/span_window_pairing.rs"]
mod surfaces_span_window_pairing;

/// Guards the `autotests = false` hazard: a suite file added under
/// `tests/` but not declared above would silently stop being compiled
/// and run. Walks the group directories, not just `tests/` itself, and
/// checks the declaration COUNT against what is on disk so no number
/// about this file can be asserted in prose without being computed.
///
/// One shape to know before you trip it: this guard treats every `.rs`
/// file under `tests/` as a suite, so a shared HELPER placed in a group
/// directory is reported as an undeclared suite. None exist today; the
/// header above anticipates a suite growing a `mod <helper>;`, and when
/// one does the helper belongs beside it with a `#[path]` line of its
/// own, or the guard needs a stated exclusion — not a silent one.
#[test]
// Scoped to this fn on purpose: a crate-root `#![allow]` in this file would
// weaken the lint gate for every suite module included above.
#[allow(clippy::expect_used)]
fn every_suite_file_is_aggregated() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let src = include_str!("all.rs");
    let mut missing: Vec<String> = Vec::new();
    let mut found = 0usize;
    let mut pending = vec![root.clone()];
    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir).expect("tests/ subtree is readable") {
            let path = entry.expect("readable dir entry").path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let rel = path
                .strip_prefix(&root)
                .expect("entry is under tests/")
                .to_string_lossy()
                .replace('\\', "/");
            if rel == "all.rs" {
                continue;
            }
            found += 1;
            if !src.contains(&format!("#[path = \"{rel}\"]")) {
                missing.push(rel);
            }
        }
    }
    missing.sort();
    assert!(
        missing.is_empty(),
        "suites under tests/ are not declared in tests/all.rs, so `autotests = false` \
         is silently dropping them: {missing:?}. Add a `#[path]` line for each."
    );

    // The count, computed rather than restated. `missing` proves every
    // file on disk is declared; this proves the converse is not padded
    // — one `#[path]` line per suite file, no orphan declarations. The
    // `format!` call above spells its quote escaped, so it is not one
    // of these matches.
    let declared = src.matches("#[path = \"").count();
    assert_eq!(
        declared, found,
        "tests/all.rs declares {declared} suites but {found} suite files exist under tests/"
    );
}
