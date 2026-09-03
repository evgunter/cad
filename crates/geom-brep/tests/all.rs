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

#[path = "approx_surface.rs"]
mod approx_surface;
#[path = "arc_eval_anchor.rs"]
mod arc_eval_anchor;
#[path = "cert1_r1_probes.rs"]
mod cert1_r1_probes;
#[path = "cert1_sphere_polar.rs"]
mod cert1_sphere_polar;
#[path = "cert3r1_e2e.rs"]
mod cert3r1_e2e;
#[path = "cert5_arm_and_cells.rs"]
mod cert5_arm_and_cells;
#[path = "cert5_r1_patch_probes.rs"]
mod cert5_r1_patch_probes;

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
#[path = "r2_probe_sphere_polar.rs"]
mod r2_probe_sphere_polar;

#[path = "offb_r2_probes.rs"]
mod offb_r2_probes;

#[path = "cert7_r1_probes.rs"]
mod cert7_r1_probes;

#[path = "cert7_r2_probes.rs"]
mod cert7_r2_probes;

#[path = "cert10_r1_probes.rs"]
mod cert10_r1_probes;

#[path = "cert10r2_probes.rs"]
mod cert10r2_probes;

#[path = "r2_quad_digit_probe.rs"]
mod r2_quad_digit_probe;

#[path = "offset_fit.rs"]
mod offset_fit;

#[path = "cert5_r2_probes.rs"]
mod cert5_r2_probes;
#[path = "offset_mint.rs"]
mod offset_mint;
#[path = "pcurve_conic.rs"]
mod pcurve_conic;
#[path = "pcurve_general.rs"]
mod pcurve_general;
#[path = "pcurve_p1a_meter.rs"]
mod pcurve_p1a_meter;
#[path = "pcurve_p1b_r2_probes.rs"]
mod pcurve_p1b_r2_probes;
#[path = "pcurve_parameter_finding.rs"]
mod pcurve_parameter_finding;
#[path = "r1_pxn_probes.rs"]
mod r1_pxn_probes;
#[path = "r2_cert3_e2e.rs"]
mod r2_cert3_e2e;
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
#[path = "revolved_point_anchor.rs"]
mod revolved_point_anchor;
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

/// Guards the `autotests = false` hazard: a suite file added under
/// `tests/` but not declared above would silently stop being compiled
/// and run. Both directions are asserted — every file on disk is
/// declared, and every declaration answers to a file, so no number
/// about this file is stated in prose without being computed.
///
/// The walk is `test_utils::source::suite_files`, which recurses into
/// group directories and tells a suite from a shared helper by Rust's
/// own module rule; read it before adding either.
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
}

#[path = "r2_probes.rs"]
mod r2_probes;
#[path = "review_probes_m8_4.rs"]
mod review_probes_m8_4;

#[path = "r1_perimeter_probes.rs"]
mod r1_perimeter_probes;

#[path = "r2_cert6_probes.rs"]
mod r2_cert6_probes;

#[path = "cert6_gauge_rows.rs"]
mod cert6_gauge_rows;

#[path = "n2r1_probes.rs"]
mod n2r1_probes;

#[path = "cert_n2r2_class3_probes.rs"]
mod cert_n2r2_class3_probes;

#[path = "cert_n2r2_class56_probes.rs"]
mod cert_n2r2_class56_probes;

#[path = "iso_rectangle_door.rs"]
mod iso_rectangle_door;
#[path = "mesh10r1_probes.rs"]
mod mesh10r1_probes;
#[path = "mesh10r2_probes.rs"]
mod mesh10r2_probes;
#[path = "mesh11_arc_branch.rs"]
mod mesh11_arc_branch;
#[path = "mesh11r1_probes.rs"]
mod mesh11r1_probes;
#[path = "mesh11r2_base_probes.rs"]
mod mesh11r2_base_probes;
#[path = "mesh11r2_probes.rs"]
mod mesh11r2_probes;
#[path = "r2_mesh7_door_probes.rs"]
mod r2_mesh7_door_probes;
