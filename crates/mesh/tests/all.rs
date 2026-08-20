//! Aggregated integration-test binary for `mesh`.
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

#[path = "budget_meter.rs"]
mod budget_meter;
#[path = "errors.rs"]
mod errors;
#[path = "exact_vs_mesh.rs"]
mod exact_vs_mesh;
#[path = "fitted_refusals.rs"]
mod fitted_refusals;
#[path = "genus.rs"]
mod genus;
#[path = "issue111_az_needle.rs"]
mod issue111_az_needle;
#[path = "m5_pr11_trimmed.rs"]
mod m5_pr11_trimmed;
#[path = "m5_s10_face_sense.rs"]
mod m5_s10_face_sense;
#[path = "m5_s11_concave_sense.rs"]
mod m5_s11_concave_sense;
#[path = "m7_nurbs_trimmed.rs"]
mod m7_nurbs_trimmed;
#[path = "newell_probes.rs"]
mod newell_probes;
#[path = "prisms.rs"]
mod prisms;
#[path = "probe_review.rs"]
mod probe_review;
#[path = "profile_overrides.rs"]
mod profile_overrides;
#[path = "review_m2_pr6_cert_oracle.rs"]
mod review_m2_pr6_cert_oracle;
#[path = "review_m2_pr6_checkmesh_audit.rs"]
mod review_m2_pr6_checkmesh_audit;
#[path = "review_m2_pr6_determinism.rs"]
mod review_m2_pr6_determinism;
#[path = "review_m2_pr6_errors.rs"]
mod review_m2_pr6_errors;
#[path = "review_m2_pr6_walk_shapes.rs"]
mod review_m2_pr6_walk_shapes;
#[path = "review_m3_pr1_mesh.rs"]
mod review_m3_pr1_mesh;
#[path = "revolves.rs"]
mod revolves;
#[path = "wedge.rs"]
mod wedge;

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
