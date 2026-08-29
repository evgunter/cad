//! Aggregated integration-test binary for `sweep`.
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

#[path = "offb_r1_loft_probes.rs"]
mod offb_r1_loft_probes;
#[path = "offc_r1_probes.rs"]
mod offc_r1_probes;
#[path = "offd2_r1_probes.rs"]
mod offd2_r1_probes;
#[path = "offd_r1_probes.rs"]
mod offd_r1_probes;
#[path = "shellfix1_bitdump.rs"]
mod shellfix1_bitdump;
#[path = "shellfix1_r1_probes.rs"]
mod shellfix1_r1_probes;
#[path = "verbs_offc_consumer.rs"]
mod verbs_offc_consumer;
#[path = "verbs_offd.rs"]
mod verbs_offd;
#[path = "verbs_shell.rs"]
mod verbs_shell;

#[path = "bitdump.rs"]
mod bitdump;
#[path = "extrude_acceptance.rs"]
mod extrude_acceptance;
#[path = "extrude_interval.rs"]
mod extrude_interval;
#[path = "issue93_az_intersect.rs"]
mod issue93_az_intersect;
#[path = "k_report.rs"]
mod k_report;
#[path = "lib_u3_sections.rs"]
mod lib_u3_sections;
#[path = "m3_pr5_extrude_booleans.rs"]
mod m3_pr5_extrude_booleans;
#[path = "m5_pr10_frontier.rs"]
mod m5_pr10_frontier;
#[path = "m5_pr10_skin.rs"]
mod m5_pr10_skin;
#[path = "m5_pr10_skin_interval.rs"]
mod m5_pr10_skin_interval;
#[path = "m5_pr11_quad_interval.rs"]
mod m5_pr11_quad_interval;
#[path = "m5_pr11_quad_props.rs"]
mod m5_pr11_quad_props;
#[path = "m5_pr12_battery.rs"]
mod m5_pr12_battery;
#[path = "m5_pr12_blends.rs"]
mod m5_pr12_blends;
#[path = "m5_pr12_die.rs"]
mod m5_pr12_die;
#[path = "m5_pr12_die_body.rs"]
mod m5_pr12_die_body;
#[path = "m5_pr12_fix_pass.rs"]
mod m5_pr12_fix_pass;
#[path = "m5_pr12_refusals.rs"]
mod m5_pr12_refusals;
#[path = "m5_pr5_tilted_cut.rs"]
mod m5_pr5_tilted_cut;
#[path = "m5_pr6_pcurves.rs"]
mod m5_pr6_pcurves;
#[path = "m5_pr9_boss_union.rs"]
mod m5_pr9_boss_union;
#[path = "m5_pr9_cosurface_merge.rs"]
mod m5_pr9_cosurface_merge;
#[path = "m5_pr9_sector2.rs"]
mod m5_pr9_sector2;
#[path = "m5_pr9c_sphere_doors.rs"]
mod m5_pr9c_sphere_doors;
#[path = "m5_pr9c_sphere_doors_interval.rs"]
mod m5_pr9c_sphere_doors_interval;
#[path = "m5_s10_face_sense.rs"]
mod m5_s10_face_sense;
#[path = "m5_s11_concave_sense.rs"]
mod m5_s11_concave_sense;
#[path = "m5_s11_concave_sense_interval.rs"]
mod m5_s11_concave_sense_interval;
#[path = "m5_s12_curved_ops.rs"]
mod m5_s12_curved_ops;
#[path = "m5_s12_curved_ops_interval.rs"]
mod m5_s12_curved_ops_interval;
#[path = "m5_s13_pips.rs"]
mod m5_s13_pips;
#[path = "m5_s13_pips_interval.rs"]
mod m5_s13_pips_interval;
#[path = "m5_s13_review_probes.rs"]
mod m5_s13_review_probes;
#[path = "m6_5_fillet_naming.rs"]
mod m6_5_fillet_naming;
#[path = "m6_chart_mints.rs"]
mod m6_chart_mints;
#[path = "m6_loft_body.rs"]
mod m6_loft_body;
#[path = "m6_rider.rs"]
mod m6_rider;
#[path = "m6_surgery.rs"]
mod m6_surgery;
#[path = "m6_surgery_interval.rs"]
mod m6_surgery_interval;
#[path = "m6_tube.rs"]
mod m6_tube;
#[path = "m7_skin_integral.rs"]
mod m7_skin_integral;
#[path = "m8_14_long_turn_sweep.rs"]
mod m8_14_long_turn_sweep;
#[path = "m9_2b_r2_probes.rs"]
mod m9_2b_r2_probes;
#[path = "m9_d1_r1_probes.rs"]
mod m9_d1_r1_probes;
#[path = "m9_d1_r2_probes.rs"]
mod m9_d1_r2_probes;
#[path = "mass_props.rs"]
mod mass_props;
#[path = "mass_props_interval.rs"]
mod mass_props_interval;
#[path = "readback_doors.rs"]
mod readback_doors;
#[path = "review_arceval_r1_probes.rs"]
mod review_arceval_r1_probes;
#[path = "review_arms2_r1_probes.rs"]
mod review_arms2_r1_probes;
#[path = "review_arms3_r1_probes.rs"]
mod review_arms3_r1_probes;
#[path = "review_chamfer_r1_probes.rs"]
mod review_chamfer_r1_probes;
#[path = "review_d2_adv_probes.rs"]
mod review_d2_adv_probes;
#[path = "review_d2_recourse_at_the_site.rs"]
mod review_d2_recourse_at_the_site;
#[path = "review_d8_consumer_differential.rs"]
mod review_d8_consumer_differential;
#[path = "review_m2_pr4.rs"]
mod review_m2_pr4;
#[path = "review_m2_pr4_interval.rs"]
mod review_m2_pr4_interval;
#[path = "review_m2_pr5.rs"]
mod review_m2_pr5;
#[path = "review_m2_pr5_interval.rs"]
mod review_m2_pr5_interval;
#[path = "review_m2_pr7.rs"]
mod review_m2_pr7;
#[path = "review_m2_pr7_interval.rs"]
mod review_m2_pr7_interval;
#[path = "review_m3_pr1_sweep.rs"]
mod review_m3_pr1_sweep;
#[path = "review_m5_pr10.rs"]
mod review_m5_pr10;
#[path = "review_m5_pr10_interval.rs"]
mod review_m5_pr10_interval;
#[path = "review_m5_pr9_base_props_probe.rs"]
mod review_m5_pr9_base_props_probe;
#[path = "review_m5_pr9_boss_probe.rs"]
mod review_m5_pr9_boss_probe;
#[path = "review_m5_pr9_inband_at_rest.rs"]
mod review_m5_pr9_inband_at_rest;
#[path = "review_m6_3_loft_probes.rs"]
mod review_m6_3_loft_probes;
#[path = "review_m6_5_pr2_sweep_probes.rs"]
mod review_m6_5_pr2_sweep_probes;
#[path = "review_m6_surgery_probes.rs"]
mod review_m6_surgery_probes;
#[path = "review_pr12_probes.rs"]
mod review_pr12_probes;
#[path = "review_s11_adv.rs"]
mod review_s11_adv;
#[path = "review_s12_adv.rs"]
mod review_s12_adv;
#[path = "review_s6_probe.rs"]
mod review_s6_probe;
#[path = "review_verbs_rim_lever_probes.rs"]
mod review_verbs_rim_lever_probes;
#[path = "revolve_ball.rs"]
mod revolve_ball;
#[path = "revolve_cone.rs"]
mod revolve_cone;
#[path = "revolve_determinism.rs"]
mod revolve_determinism;
#[path = "revolve_errors.rs"]
mod revolve_errors;
#[path = "revolve_interval.rs"]
mod revolve_interval;
#[path = "revolve_partial.rs"]
mod revolve_partial;
#[path = "revolve_ring.rs"]
mod revolve_ring;
#[path = "revolve_washer.rs"]
mod revolve_washer;
#[path = "ring_r1_probes.rs"]
mod ring_r1_probes;
#[path = "s16_box_soundness.rs"]
mod s16_box_soundness;
#[path = "s49_census_jurisdiction.rs"]
mod s49_census_jurisdiction;
#[path = "verbs_arms1_annulus.rs"]
mod verbs_arms1_annulus;
#[path = "verbs_arms1_r1_probes.rs"]
mod verbs_arms1_r1_probes;
#[path = "verbs_arms2_arms.rs"]
mod verbs_arms2_arms;
#[path = "verbs_arms2_bud.rs"]
mod verbs_arms2_bud;
#[path = "verbs_arms3.rs"]
mod verbs_arms3;
#[path = "verbs_chamfer.rs"]
mod verbs_chamfer;
#[path = "verbs_cylcyl_probe.rs"]
mod verbs_cylcyl_probe;
#[path = "verbs_cylcyl_r1_review_probes.rs"]
mod verbs_cylcyl_r1_review_probes;
#[path = "verbs_cylcylb_r1_blinded_probes.rs"]
mod verbs_cylcylb_r1_blinded_probes;
#[path = "verbs_pierce.rs"]
mod verbs_pierce;
#[path = "verbs_pierce_r1_probes.rs"]
mod verbs_pierce_r1_probes;
#[path = "verbs_pierce_r2_probes.rs"]
mod verbs_pierce_r2_probes;
#[path = "verbs_rim_closed_lever.rs"]
mod verbs_rim_closed_lever;
#[path = "verbs_rim_r1_probes.rs"]
mod verbs_rim_r1_probes;
#[path = "verbs_tubewall.rs"]
mod verbs_tubewall;
#[path = "verbs_tubewall_r1_fingerprint.rs"]
mod verbs_tubewall_r1_fingerprint;
#[path = "verbs_tubewall_r1_probes.rs"]
mod verbs_tubewall_r1_probes;
#[path = "verbs_tubewall_r2_probes.rs"]
mod verbs_tubewall_r2_probes;
#[path = "verbs_tubewall_r2_solidbits.rs"]
mod verbs_tubewall_r2_solidbits;

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

#[path = "m8_3_rational_volume.rs"]
mod m8_3_rational_volume;

#[path = "m8_4_intersection_iso.rs"]
mod m8_4_intersection_iso;

#[path = "m9_2_chart_region_loft.rs"]
mod m9_2_chart_region_loft;

#[path = "m9_3_wall_door.rs"]
mod m9_3_wall_door;

#[path = "m9_3_zip.rs"]
mod m9_3_zip;

#[path = "review_probes_m8_4.rs"]
mod review_probes_m8_4;

#[path = "r1_probes_m9_3.rs"]
mod r1_probes_m9_3;

#[path = "verbs_gate_r1_probes.rs"]
mod verbs_gate_r1_probes;

#[path = "verbs_f7_r2_probes.rs"]
mod verbs_f7_r2_probes;
#[path = "verbs_shell_r2_probes.rs"]
mod verbs_shell_r2_probes;
#[path = "verbs_shell_r2b.rs"]
mod verbs_shell_r2b;
