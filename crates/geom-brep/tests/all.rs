//! Aggregated integration-test binary for `geom-brep`.
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

#[path = "approx_surface.rs"]
mod approx_surface;
#[path = "arc_eval_anchor.rs"]
mod arc_eval_anchor;
#[path = "decoration_plane_mint.rs"]
mod decoration_plane_mint;
#[path = "imported_chart_arc_rim.rs"]
mod imported_chart_arc_rim;
#[path = "intersect_table.rs"]
mod intersect_table;
#[path = "m4_remint_headroom.rs"]
mod m4_remint_headroom;
#[path = "m4_remint_sliver.rs"]
mod m4_remint_sliver;
#[path = "m5_pr12_circle_certificate.rs"]
mod m5_pr12_circle_certificate;
#[path = "m5_pr7_ssi.rs"]
mod m5_pr7_ssi;
#[path = "m5_pr9_tangent.rs"]
mod m5_pr9_tangent;
#[path = "m7_8_plane_nurbs_edge.rs"]
mod m7_8_plane_nurbs_edge;
#[path = "m8_f67_r1_probes.rs"]
mod m8_f67_r1_probes;
#[path = "offa_r1_probes.rs"]
mod offa_r1_probes;
#[path = "offb_r1_probes.rs"]
mod offb_r1_probes;

#[path = "offb_r2_probes.rs"]
mod offb_r2_probes;

#[path = "offset_fit.rs"]
mod offset_fit;

#[path = "offset_mint.rs"]
mod offset_mint;
#[path = "pcurve_conic.rs"]
mod pcurve_conic;
#[path = "pcurve_general.rs"]
mod pcurve_general;
#[path = "pcurve_p1a_meter.rs"]
mod pcurve_p1a_meter;
#[path = "pcurve_p1a_meter_interval.rs"]
mod pcurve_p1a_meter_interval;
#[path = "pcurve_parameter_finding.rs"]
mod pcurve_parameter_finding;
#[path = "r1_pxn_probes.rs"]
mod r1_pxn_probes;
#[path = "review_arceval_r1_probes.rs"]
mod review_arceval_r1_probes;
#[path = "review_flux_probes_r1.rs"]
mod review_flux_probes_r1;
#[path = "review_m2_pr3_certify.rs"]
mod review_m2_pr3_certify;
#[path = "review_m2_pr7_props.rs"]
mod review_m2_pr7_props;
#[path = "review_m5_pr3_e2e.rs"]
mod review_m5_pr3_e2e;
#[path = "review_m5_pr7_adversarial.rs"]
mod review_m5_pr7_adversarial;
#[path = "review_m5_pr7_enclosure.rs"]
mod review_m5_pr7_enclosure;
#[path = "review_m5_pr7b_ssi.rs"]
mod review_m5_pr7b_ssi;
#[path = "review_m5_pr9_jet_probe.rs"]
mod review_m5_pr9_jet_probe;
#[path = "review_m6_3_chart_probes.rs"]
mod review_m6_3_chart_probes;
#[path = "review_m6_surgery_rider.rs"]
mod review_m6_surgery_rider;
#[path = "review_pr12_meridian_probe.rs"]
mod review_pr12_meridian_probe;
#[path = "review_r1_rational_probes.rs"]
mod review_r1_rational_probes;
#[path = "rim_dim_review_probes.rs"]
mod rim_dim_review_probes;
#[path = "rim_dim_scale_twins.rs"]
mod rim_dim_scale_twins;
#[path = "s58_iso_rectangle.rs"]
mod s58_iso_rectangle;
#[path = "s81_one_rim_level_rule.rs"]
mod s81_one_rim_level_rule;
#[path = "span_meter_dim_twins.rs"]
mod span_meter_dim_twins;

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

#[path = "r2_probes.rs"]
mod r2_probes;
#[path = "review_probes_m8_4.rs"]
mod review_probes_m8_4;
