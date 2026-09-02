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

// Each suite keeps its own verbatim `mod <helper>;`, so a shared helper is
// loaded once per suite that uses it. That is deliberate — the alternative
// is editing the suites — and it is what `duplicate_mod` is warning about.
// Allowed HERE ONLY, by name: no blanket `#![allow]`, which would weaken
// the lint gate for every suite module included below.
#![allow(clippy::duplicate_mod)]

#[path = "bool1_fix_pass.rs"]
mod bool1_fix_pass;
#[path = "bool1_r1_probes.rs"]
mod bool1_r1_probes;
#[path = "bool2_cone_doors.rs"]
mod bool2_cone_doors;
#[path = "bool2_cone_doors_interval.rs"]
mod bool2_cone_doors_interval;
#[path = "bool2_r1_probes.rs"]
mod bool2_r1_probes;
#[path = "bool2_r2_probes.rs"]
mod bool2_r2_probes;
#[path = "bool3_r1_probes.rs"]
mod bool3_r1_probes;
#[path = "bool3_torus_doors.rs"]
mod bool3_torus_doors;
#[path = "bool3_torus_doors_interval.rs"]
mod bool3_torus_doors_interval;
#[path = "offb_r1_loft_probes.rs"]
mod offb_r1_loft_probes;
#[path = "offc_r1_probes.rs"]
mod offc_r1_probes;
#[path = "offd2_r1_probes.rs"]
mod offd2_r1_probes;
#[path = "offd_r1_probes.rs"]
mod offd_r1_probes;
#[path = "p1b_r1_probes.rs"]
mod p1b_r1_probes;
#[path = "pcurve_p1b_r2_probes.rs"]
mod pcurve_p1b_r2_probes;
#[path = "r1_mate3_probes.rs"]
mod r1_mate3_probes;
#[path = "r2_mate3_probes.rs"]
mod r2_mate3_probes;
#[path = "r2_mesh1_donut_probes.rs"]
mod r2_mesh1_donut_probes;
#[path = "sf2a_r1.rs"]
mod sf2a_r1;
#[path = "sf2a_r1_head.rs"]
mod sf2a_r1_head;
#[path = "sf2a_r2_interval_probe.rs"]
mod sf2a_r2_interval_probe;
#[path = "sf2a_r2_probes.rs"]
mod sf2a_r2_probes;
#[path = "sf2b_axial.rs"]
mod sf2b_axial;
#[path = "sf2b_head.rs"]
mod sf2b_head;
#[path = "sf2b_interval_probe.rs"]
mod sf2b_interval_probe;
#[path = "sf2b_r1_probes.rs"]
mod sf2b_r1_probes;
#[path = "sf2b_r2_probes.rs"]
mod sf2b_r2_probes;
#[path = "shellfix1_bitdump.rs"]
mod shellfix1_bitdump;
#[path = "shellfix1_r1_probes.rs"]
mod shellfix1_r1_probes;
#[path = "torax_axial.rs"]
mod torax_axial;
#[path = "torax_interval.rs"]
mod torax_interval;
#[path = "verbs_offc_consumer.rs"]
mod verbs_offc_consumer;
#[path = "verbs_offd.rs"]
mod verbs_offd;
#[path = "verbs_shell.rs"]
mod verbs_shell;

#[path = "bitdump.rs"]
mod bitdump;
#[path = "blend1_r1_probes.rs"]
mod blend1_r1_probes;
#[path = "blend2_r2_probes.rs"]
mod blend2_r2_probes;
#[path = "blend3_concave_chamfer.rs"]
mod blend3_concave_chamfer;
#[path = "blend3_r2_probes.rs"]
mod blend3_r2_probes;
#[path = "blend4_concave_fillet.rs"]
mod blend4_concave_fillet;
#[path = "blend4_r1_probes.rs"]
mod blend4_r1_probes;
#[path = "blend6_verb_vocab.rs"]
mod blend6_verb_vocab;
#[path = "blend_seam_split_rim.rs"]
mod blend_seam_split_rim;
#[path = "blend_tworims.rs"]
mod blend_tworims;
#[path = "cert5_offgrid_knot_rational.rs"]
mod cert5_offgrid_knot_rational;
#[path = "cert5_r1_e2e_probes.rs"]
mod cert5_r1_e2e_probes;
#[path = "cert8_r1_probes.rs"]
mod cert8_r1_probes;
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
#[path = "r1_probes_issue1362_donut.rs"]
mod r1_probes_issue1362_donut;
#[path = "readback_doors.rs"]
mod readback_doors;
#[path = "review_arceval_r1_probes.rs"]
mod review_arceval_r1_probes;
#[path = "review_arms2_r1_probes.rs"]
mod review_arms2_r1_probes;
#[path = "review_arms3_r1_probes.rs"]
mod review_arms3_r1_probes;
#[path = "review_blend1_r2_probes.rs"]
mod review_blend1_r2_probes;
#[path = "review_blend3_r1_probes.rs"]
mod review_blend3_r1_probes;
#[path = "review_blend4_r2_probes.rs"]
mod review_blend4_r2_probes;
#[path = "review_blend6_r1_probes.rs"]
mod review_blend6_r1_probes;
#[path = "review_blend6_r2_probes.rs"]
mod review_blend6_r2_probes;
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
#[path = "turning_orientation.rs"]
mod turning_orientation;
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
#[path = "verbs_ga_r2_probes.rs"]
mod verbs_ga_r2_probes;
#[path = "verbs_germarms.rs"]
mod verbs_germarms;
#[path = "verbs_germarms2.rs"]
mod verbs_germarms2;
#[path = "verbs_germarms2_interval.rs"]
mod verbs_germarms2_interval;
#[path = "verbs_germarms_interval.rs"]
mod verbs_germarms_interval;
#[path = "verbs_germarms_r1_probes.rs"]
mod verbs_germarms_r1_probes;
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
#[path = "verbs_sphsph_chart.rs"]
mod verbs_sphsph_chart;
#[path = "verbs_sphsph_opening.rs"]
mod verbs_sphsph_opening;
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

#[path = "m8_3_rational_volume.rs"]
mod m8_3_rational_volume;

#[path = "m8_4_intersection_iso.rs"]
mod m8_4_intersection_iso;

#[path = "m9_2_chart_region_loft.rs"]
mod m9_2_chart_region_loft;

#[path = "r2_probe_cert8.rs"]
mod r2_probe_cert8;

#[path = "m9_3_wall_door.rs"]
mod m9_3_wall_door;

#[path = "m9_3_zip.rs"]
mod m9_3_zip;
#[path = "mate2_cyl_rest.rs"]
mod mate2_cyl_rest;
#[path = "mate2_r1_probes.rs"]
mod mate2_r1_probes;
#[path = "mate2_r2_probes.rs"]
mod mate2_r2_probes;
#[path = "mate7a_r1_probes.rs"]
mod mate7a_r1_probes;
#[path = "mate7a_r2_probes.rs"]
mod mate7a_r2_probes;
#[path = "mate7a_torus_rest.rs"]
mod mate7a_torus_rest;

#[path = "review_probes_m8_4.rs"]
mod review_probes_m8_4;

#[path = "r1_probes_m9_3.rs"]
mod r1_probes_m9_3;

#[path = "verbs_gate_r1_probes.rs"]
mod verbs_gate_r1_probes;

#[path = "f7d_delta_probes.rs"]
mod f7d_delta_probes;
#[path = "verbs_f7_r2_probes.rs"]
mod verbs_f7_r2_probes;
#[path = "verbs_shell_r2_probes.rs"]
mod verbs_shell_r2_probes;
#[path = "verbs_shell_r2b.rs"]
mod verbs_shell_r2b;

#[path = "r1_p2_probes.rs"]
mod r1_p2_probes;

#[path = "bool1_r2_probes.rs"]
mod bool1_r2_probes;
#[path = "r1_area_gauge_probes.rs"]
mod r1_area_gauge_probes;
