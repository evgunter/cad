//! Aggregated integration-test binary for `geom-curves`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary replaces what were 14 separate test targets. The files
//! themselves are untouched: each keeps its own `//!` docs, its inner
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

#[path = "boxes.rs"]
mod boxes;
#[path = "compose.rs"]
mod compose;
#[path = "curvo_oracle.rs"]
mod curvo_oracle;
#[path = "decoration_ring_coords.rs"]
mod decoration_ring_coords;
#[path = "fit_certify.rs"]
mod fit_certify;
#[path = "fitting.rs"]
mod fitting;
#[path = "hull_circle_rehearsal.rs"]
mod hull_circle_rehearsal;
#[path = "lt_r1_probes.rs"]
mod lt_r1_probes;
#[path = "m5_pr7_speed_meter.rs"]
mod m5_pr7_speed_meter;
#[path = "m8_14_long_turn_meter.rs"]
mod m8_14_long_turn_meter;
#[path = "nurbs_differential.rs"]
mod nurbs_differential;
#[path = "nurbs_interval.rs"]
mod nurbs_interval;
#[path = "projection.rs"]
mod projection;
#[path = "r2_lt_probes.rs"]
mod r2_lt_probes;
#[path = "review_m5_pr2_e2e.rs"]
mod review_m5_pr2_e2e;
#[path = "review_m5_pr3_attack.rs"]
mod review_m5_pr3_attack;
#[path = "review_m5_pr3_attack_interval.rs"]
mod review_m5_pr3_attack_interval;
#[path = "review_m5_pr4_adversarial.rs"]
mod review_m5_pr4_adversarial;
#[path = "span_window_pairing.rs"]
mod span_window_pairing;
#[path = "split_at.rs"]
mod split_at;

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
