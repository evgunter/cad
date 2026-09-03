//! Aggregated integration-test binary for `topo`.
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
// `tests/common/mod.rs`, `tests/fixture/mod.rs` — and every consumer
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
mod fixture;

#[path = "box_with_hole.rs"]
mod box_with_hole;
#[path = "census_g2_carrier.rs"]
mod census_g2_carrier;
#[path = "corner_table.rs"]
mod corner_table;
#[path = "crosslap_rest.rs"]
mod crosslap_rest;
#[path = "cube_by_hand.rs"]
mod cube_by_hand;
#[path = "display_contract.rs"]
mod display_contract;
#[path = "geometric_cube.rs"]
mod geometric_cube;
#[path = "graft_disjoint.rs"]
mod graft_disjoint;
#[path = "h14_census_deferrals.rs"]
mod h14_census_deferrals;
#[path = "interval_body.rs"]
mod interval_body;
#[path = "issue86_double_subtract.rs"]
mod issue86_double_subtract;
#[path = "issue93_nested_islands.rs"]
mod issue93_nested_islands;
#[path = "m3_pr1_surgery.rs"]
mod m3_pr1_surgery;
#[path = "m3_pr2_reduce.rs"]
mod m3_pr2_reduce;
#[path = "m3_pr3_split.rs"]
mod m3_pr3_split;
#[path = "m3_pr4_boolean.rs"]
mod m3_pr4_boolean;
#[path = "m3_pr5_boolean_ops.rs"]
mod m3_pr5_boolean_ops;
#[path = "m3_pr6_saddle.rs"]
mod m3_pr6_saddle;
#[path = "m3_pr6_tier3prime.rs"]
mod m3_pr6_tier3prime;
#[path = "m4_pr2_transform.rs"]
mod m4_pr2_transform;
#[path = "m4_remint_transform.rs"]
mod m4_remint_transform;
#[path = "m5_pr7_split_meter.rs"]
mod m5_pr7_split_meter;
#[path = "m5_pr8_bvh_diff.rs"]
mod m5_pr8_bvh_diff;
#[path = "m5_s1_rest_zip.rs"]
mod m5_s1_rest_zip;
#[path = "m6_2_fitted_at_rest.rs"]
mod m6_2_fitted_at_rest;
#[path = "m6_3_chart_completion.rs"]
mod m6_3_chart_completion;
#[path = "m9_1_contact_vocabulary.rs"]
mod m9_1_contact_vocabulary;
#[path = "m9_2_census_door.rs"]
mod m9_2_census_door;
#[path = "m9_2b_r2_probes.rs"]
mod m9_2b_r2_probes;
#[path = "m9_c1_r1_probes.rs"]
mod m9_c1_r1_probes;
#[path = "m9_c1_rest_face_rung.rs"]
mod m9_c1_rest_face_rung;
#[path = "mate4a_ef_bound_rung.rs"]
mod mate4a_ef_bound_rung;
#[path = "mate5_cyl_eps_rung.rs"]
mod mate5_cyl_eps_rung;
#[path = "mate8_witness_schedule.rs"]
mod mate8_witness_schedule;
#[path = "mate9_crossing_rung.rs"]
mod mate9_crossing_rung;
#[path = "merge_skip.rs"]
mod merge_skip;
#[path = "mesh8_coherence.rs"]
mod mesh8_coherence;
#[path = "quad_lane_is_the_certified_lane.rs"]
mod quad_lane_is_the_certified_lane;
#[path = "r1_mate4a_probes.rs"]
mod r1_mate4a_probes;
#[path = "r1_mate5_interval_probe.rs"]
mod r1_mate5_interval_probe;
#[path = "r1_mate5_probe.rs"]
mod r1_mate5_probe;
#[path = "r1_mate8_decomp_probe.rs"]
mod r1_mate8_decomp_probe;
#[path = "r1_mate8_probes.rs"]
mod r1_mate8_probes;
#[path = "r2_probes.rs"]
mod r2_probes;
#[path = "review_m1_pr5.rs"]
mod review_m1_pr5;
#[path = "review_m2_pr3.rs"]
mod review_m2_pr3;
#[path = "review_m2_pr7.rs"]
mod review_m2_pr7;
#[path = "review_m3_pr1.rs"]
mod review_m3_pr1;
#[path = "review_m3_pr2.rs"]
mod review_m3_pr2;
#[path = "review_m3_pr3_bob.rs"]
mod review_m3_pr3_bob;
#[path = "review_m3_pr3_consumer.rs"]
mod review_m3_pr3_consumer;
#[path = "review_m3_pr3_order.rs"]
mod review_m3_pr3_order;
#[path = "review_m3_pr3_pil.rs"]
mod review_m3_pr3_pil;
#[path = "review_m3_pr3_rings.rs"]
mod review_m3_pr3_rings;
#[path = "review_m3_pr4.rs"]
mod review_m3_pr4;
#[path = "review_m3_pr5.rs"]
mod review_m3_pr5;
#[path = "review_m3_pr55.rs"]
mod review_m3_pr55;
#[path = "review_m3_pr6.rs"]
mod review_m3_pr6;
#[path = "review_m4_pr2_transform.rs"]
mod review_m4_pr2_transform;
#[path = "review_m6_2_probes.rs"]
mod review_m6_2_probes;
#[path = "review_m9_1_probes.rs"]
mod review_m9_1_probes;
#[path = "review_m9_1_r2_probes.rs"]
mod review_m9_1_r2_probes;
#[path = "review_mate4a_r2_probes.rs"]
mod review_mate4a_r2_probes;
#[path = "review_mate9_r1_probes.rs"]
mod review_mate9_r1_probes;
#[path = "review_mate9_r2_probes.rs"]
mod review_mate9_r2_probes;
#[path = "review_s1_controls.rs"]
mod review_s1_controls;
#[path = "review_s1_probes.rs"]
mod review_s1_probes;
#[path = "review_s6_probe.rs"]
mod review_s6_probe;
#[path = "review_ssiflat_r1_probes.rs"]
mod review_ssiflat_r1_probes;
#[path = "review_ssiflat_r2_probes.rs"]
mod review_ssiflat_r2_probes;
#[path = "rim_dim_boolean_twins.rs"]
mod rim_dim_boolean_twins;
#[path = "rim_dim_review_probes.rs"]
mod rim_dim_review_probes;
#[path = "seat3_flush_detector.rs"]
mod seat3_flush_detector;
#[path = "shell_roles.rs"]
mod shell_roles;
#[path = "solid_separation.rs"]
mod solid_separation;
#[path = "void_door.rs"]
mod void_door;

/// Guards the `autotests = false` hazard and the ONE HOME rule in one
/// call: every suite file under `tests/` is mounted above, every mount
/// answers to a file, and no suite declares a module of its own.
///
/// The three checks, their messages and the walk that feeds them live
/// in `test_utils::source::aggregation_violations` — once, for every
/// crate that carries this row. Read it before adding a suite or a
/// shared helper.
#[test]
fn every_suite_file_is_aggregated() {
    let tests = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let violations = test_utils::source::aggregation_violations(&tests, include_str!("all.rs"));
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
#[path = "f7d_delta_probes.rs"]
mod f7d_delta_probes;
#[path = "probe_census.rs"]
mod probe_census;
#[path = "probe_f34_review.rs"]
mod probe_f34_review;
#[path = "probe_s5_sectors.rs"]
mod probe_s5_sectors;
#[path = "review_f7_pole_r1_probes.rs"]
mod review_f7_pole_r1_probes;
#[path = "verbs_cylsph_tangent_residuals.rs"]
mod verbs_cylsph_tangent_residuals;
#[path = "verbs_f7_collinear_seam.rs"]
mod verbs_f7_collinear_seam;
#[path = "verbs_f7_r2_probes.rs"]
mod verbs_f7_r2_probes;
