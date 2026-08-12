//! Aggregated integration-test binary for `geom-core`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary replaces what were 20 separate test targets. The files
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
//! EXEMPT: `tolerance_init.rs` is deliberately NOT aggregated — it needs
//! its own process for the tolerance `OnceLock` and has its own
//! `[[test]]` target. The guard below knows about it by name.
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

#[path = "ambiguity_k_env.rs"]
mod ambiguity_k_env;
#[path = "band_tolerance.rs"]
mod band_tolerance;
#[path = "flagged_census.rs"]
mod flagged_census;
#[path = "interval_band.rs"]
mod interval_band;
#[path = "m5_pr1_poison_conservation.rs"]
mod m5_pr1_poison_conservation;
#[path = "m5_pr7b_tensor_compose.rs"]
mod m5_pr7b_tensor_compose;
#[path = "review_m0_pr2.rs"]
mod review_m0_pr2;
#[path = "review_m0_pr3.rs"]
mod review_m0_pr3;
#[path = "review_m0_pr4.rs"]
mod review_m0_pr4;
#[path = "review_m0_pr5.rs"]
mod review_m0_pr5;
#[path = "review_m0_pr6.rs"]
mod review_m0_pr6;
#[path = "review_m2_pr7_k.rs"]
mod review_m2_pr7_k;
#[path = "review_m5_pr1_launder.rs"]
mod review_m5_pr1_launder;
#[path = "review_m5_pr2_scratch.rs"]
mod review_m5_pr2_scratch;
#[path = "review_m5_pr2_scratch_hull.rs"]
mod review_m5_pr2_scratch_hull;
#[path = "review_m5_pr7_svd.rs"]
mod review_m5_pr7_svd;
#[path = "review_m5_pr7b_tensor.rs"]
mod review_m5_pr7b_tensor;
#[path = "review_margin_probe.rs"]
mod review_margin_probe;
#[path = "ring_interval_differential.rs"]
mod ring_interval_differential;
#[path = "ring_interval_fuzz.rs"]
mod ring_interval_fuzz;
#[path = "spline_hull.rs"]
mod spline_hull;

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
        // DELIBERATE EXEMPTION, not an oversight: `tolerance_init.rs`
        // needs its own PROCESS (the global tolerance commits once per
        // process), so it has its own `[[test]]` target in Cargo.toml.
        // Aggregating it here made `cargo test -p geom-core` fail
        // whichever suite happened to touch the global first; CI only
        // stayed green because nextest forks per test.
        //
        // Enforced in BOTH directions: skipping it silently would let a
        // future edit re-aggregate it and reintroduce the race, so its
        // absence is asserted rather than assumed.
        if name == "tolerance_init.rs" {
            assert!(
                !src.contains(&format!("#[path = \"{name}\"]")),
                "tolerance_init.rs must NOT be aggregated: it needs its own \
                 process for the tolerance OnceLock and has its own [[test]] \
                 target. Re-adding it here makes `cargo test -p geom-core` \
                 fail on a clean tree, invisibly under nextest."
            );
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
