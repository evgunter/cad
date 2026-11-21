//! Aggregated integration-test binary for `editor-core`.
//!
//! Every `tests/*.rs` suite is included here VERBATIM via `#[path]`, so
//! this one binary stands in for one test target per suite.
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.
//!
//! Each suite keeps its own `//!` docs and its inner attributes
//! (`#![cfg(feature = "probe")]` and friends work as module-level
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
// `tests/corpus/mod.rs` and `tests/fixture/mod.rs` — and every consumer
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
mod blocks;
mod corpus;
mod fixture;
// The wire-surgery helper, editor-core's alone — `tests/fixture/` and
// `tests/corpus/` are symlinked into `crates/viewer/tests/`, and this
// one is not, because its subject is `serde_json`.
mod wire;

#[path = "a_blend_refusal_names_its_edge.rs"]
mod a_blend_refusal_names_its_edge;
#[path = "a_bore_taken_alone_is_smooth_at_its_seams_interval.rs"]
mod a_bore_taken_alone_is_smooth_at_its_seams_interval;
#[path = "a_dual_is_bound_to_its_build_of_record.rs"]
mod a_dual_is_bound_to_its_build_of_record;
#[path = "a_keel_on_the_floor_names_its_two_edges.rs"]
mod a_keel_on_the_floor_names_its_two_edges;
#[path = "a_turned_discs_union_names_its_unfinished_member.rs"]
mod a_turned_discs_union_names_its_unfinished_member;
#[path = "aligned_overlap_quiets.rs"]
mod aligned_overlap_quiets;
#[path = "asm1_identity_pins.rs"]
mod asm1_identity_pins;
#[path = "asm2a_instantiate.rs"]
mod asm2a_instantiate;
#[path = "asm2b_multisolid.rs"]
mod asm2b_multisolid;
#[path = "asm4_split_inline.rs"]
mod asm4_split_inline;
#[path = "asm_parent_held_names.rs"]
mod asm_parent_held_names;
#[path = "asm_r2a_mate_solve.rs"]
mod asm_r2a_mate_solve;
#[path = "asm_r2a_mate_wire.rs"]
mod asm_r2a_mate_wire;
#[path = "asm_r2b_assembly.rs"]
mod asm_r2b_assembly;
#[path = "asm_r2b_interface_wire.rs"]
mod asm_r2b_interface_wire;
#[path = "asm_upd_pin_update.rs"]
mod asm_upd_pin_update;
#[path = "assemble_one_local_battery.rs"]
mod assemble_one_local_battery;
#[path = "atrest_carried_coincidence.rs"]
mod atrest_carried_coincidence;
#[path = "atrest_operation_rows_quiet.rs"]
mod atrest_operation_rows_quiet;
#[path = "band_closed_mixed_chain_names.rs"]
mod band_closed_mixed_chain_names;
#[path = "band_joined_rim_names.rs"]
mod band_joined_rim_names;
#[path = "band_mixed_corner_names.rs"]
mod band_mixed_corner_names;
#[path = "band_planar_cut_off_names.rs"]
mod band_planar_cut_off_names;
#[path = "band_planar_mitre_names.rs"]
mod band_planar_mitre_names;
#[path = "band_run_wall_names.rs"]
mod band_run_wall_names;
#[path = "blend5_r1_probes.rs"]
mod blend5_r1_probes;
#[path = "blend5_r2_probes.rs"]
mod blend5_r2_probes;
#[path = "blend5_rim_support.rs"]
mod blend5_rim_support;
#[path = "blend5_rim_support_wire.rs"]
mod blend5_rim_support_wire;
#[path = "blend_carries_contacts.rs"]
mod blend_carries_contacts;
#[path = "blend_dual_sensitivity.rs"]
mod blend_dual_sensitivity;
#[path = "bool12r2_ec_probe.rs"]
mod bool12r2_ec_probe;
#[path = "bool13_r1_probes.rs"]
mod bool13_r1_probes;
#[path = "bool13r2_probes.rs"]
mod bool13r2_probes;
#[path = "boolean_op_wire.rs"]
mod boolean_op_wire;
#[path = "boss_flush_offer.rs"]
mod boss_flush_offer;
#[path = "cascade_delete.rs"]
mod cascade_delete;
#[path = "cert3r1_dump.rs"]
mod cert3r1_dump;
#[path = "clearance_refusals_say_what_they_found.rs"]
mod clearance_refusals_say_what_they_found;
#[path = "coincidence_door.rs"]
mod coincidence_door;
#[path = "corpus_finish_count.rs"]
mod corpus_finish_count;
#[path = "display_contract.rs"]
mod display_contract;
#[path = "dm3_indexed_read.rs"]
mod dm3_indexed_read;
#[path = "dm4_migration_one_shot.rs"]
mod dm4_migration_one_shot;
#[path = "dm4_names_by_variable.rs"]
mod dm4_names_by_variable;
#[path = "dm4_reads_key_booleans.rs"]
mod dm4_reads_key_booleans;
#[path = "dm7_delete_strands.rs"]
mod dm7_delete_strands;
#[path = "docm1_face_frame.rs"]
mod docm1_face_frame;
#[path = "docm1_face_frame_interval.rs"]
mod docm1_face_frame_interval;
#[path = "docm2_part.rs"]
mod docm2_part;
#[path = "docm2_part_interval.rs"]
mod docm2_part_interval;
#[path = "docm3_union.rs"]
mod docm3_union;
#[path = "docm4_evaluation_identity.rs"]
mod docm4_evaluation_identity;
#[path = "docm5_subject.rs"]
mod docm5_subject;
#[path = "docm6_seam_declarations.rs"]
mod docm6_seam_declarations;
#[path = "docm8_flat_merged.rs"]
mod docm8_flat_merged;
#[path = "docm9_range.rs"]
mod docm9_range;
#[path = "door_step_order.rs"]
mod door_step_order;
#[path = "dsc_checks.rs"]
mod dsc_checks;
#[path = "e4_dual_door.rs"]
mod e4_dual_door;
#[path = "e6_drive_over_a_loft_at_interval.rs"]
mod e6_drive_over_a_loft_at_interval;
#[path = "edit_blend_canonical.rs"]
mod edit_blend_canonical;
#[path = "edit_blend_one_box_of_two.rs"]
mod edit_blend_one_box_of_two;
#[path = "edit_bore_chamfer.rs"]
mod edit_bore_chamfer;
#[path = "edit_bore_two_rims.rs"]
mod edit_bore_two_rims;
#[path = "edit_concave_bar.rs"]
mod edit_concave_bar;
#[path = "edit_doc_param_distribution.rs"]
mod edit_doc_param_distribution;
#[path = "edit_doc_param_unit.rs"]
mod edit_doc_param_unit;
#[path = "edit_has_minted.rs"]
mod edit_has_minted;
#[path = "edit_instance_crossing_names.rs"]
mod edit_instance_crossing_names;
#[path = "edit_ladder_rim.rs"]
mod edit_ladder_rim;
#[path = "edit_one_cycle_members.rs"]
mod edit_one_cycle_members;
#[path = "edit_one_predicate.rs"]
mod edit_one_predicate;
#[path = "edit_pair_apply_names.rs"]
mod edit_pair_apply_names;
#[path = "edit_placement_corpus_bits.rs"]
mod edit_placement_corpus_bits;
#[path = "edit_planar_carve.rs"]
mod edit_planar_carve;
#[path = "edit_recorded_notation.rs"]
mod edit_recorded_notation;
#[path = "edit_ruled_carve.rs"]
mod edit_ruled_carve;
#[path = "edit_set_program.rs"]
mod edit_set_program;
#[path = "edit_step_segments.rs"]
mod edit_step_segments;
#[path = "edit_strip_mouth_cups.rs"]
mod edit_strip_mouth_cups;
#[path = "emit_ball_cut_by_brick.rs"]
mod emit_ball_cut_by_brick;
#[path = "emit_boolean_vertex_keys.rs"]
mod emit_boolean_vertex_keys;
#[path = "emit_union_member_order.rs"]
mod emit_union_member_order;
#[path = "eval10_section_reads_the_nominal.rs"]
mod eval10_section_reads_the_nominal;
#[path = "eval4_accept_funnel.rs"]
mod eval4_accept_funnel;
#[path = "eval6_placers_over_instances.rs"]
mod eval6_placers_over_instances;
#[path = "eval6_placers_over_instances_interval.rs"]
mod eval6_placers_over_instances_interval;
#[path = "eval9_nominal_in_the_key.rs"]
mod eval9_nominal_in_the_key;
#[path = "every_predicate_name_is_minted_once.rs"]
mod every_predicate_name_is_minted_once;
#[path = "f64_lane_validates_once.rs"]
mod f64_lane_validates_once;
#[path = "face_chart_and_signed_depth.rs"]
mod face_chart_and_signed_depth;
#[path = "face_face_touch_carry.rs"]
mod face_face_touch_carry;
#[path = "fillet_step_walk.rs"]
mod fillet_step_walk;
#[path = "fix_loop_polygon_expr.rs"]
mod fix_loop_polygon_expr;
#[path = "fix_pattern_mate_crossing.rs"]
mod fix_pattern_mate_crossing;
#[path = "fork7_shared_is_named.rs"]
mod fork7_shared_is_named;
#[path = "g_family_of_booleans.rs"]
mod g_family_of_booleans;
#[path = "g_family_review_rows.rs"]
mod g_family_review_rows;
#[path = "g_index_families.rs"]
mod g_index_families;
#[path = "g_ring_closure_measure.rs"]
mod g_ring_closure_measure;
#[path = "gap_reads_a_cell_against_a_round_face.rs"]
mod gap_reads_a_cell_against_a_round_face;
#[path = "gap_reads_vertex_and_edge_cells.rs"]
mod gap_reads_vertex_and_edge_cells;
#[path = "named_source.rs"]
mod named_source;
#[path = "placement_values_direction.rs"]
mod placement_values_direction;
#[path = "reach_slab_cut_sector_side.rs"]
mod reach_slab_cut_sector_side;
#[path = "recourse_roster.rs"]
mod recourse_roster;
#[path = "refactor_pose_definitions.rs"]
mod refactor_pose_definitions;
#[path = "refusal_concision.rs"]
mod refusal_concision;
#[path = "refusal_concision_at_rest.rs"]
mod refusal_concision_at_rest;
#[path = "refusal_concision_chains.rs"]
mod refusal_concision_chains;
#[path = "refusal_concision_refactor.rs"]
mod refusal_concision_refactor;
#[path = "remap_reorders_ids.rs"]
mod remap_reorders_ids;
#[path = "resolve_cited_line.rs"]
mod resolve_cited_line;
#[path = "resolve_group_membership.rs"]
mod resolve_group_membership;
#[path = "resolve_piece_ladder.rs"]
mod resolve_piece_ladder;
#[path = "resolve_upstream_scope.rs"]
mod resolve_upstream_scope;
#[path = "revalue_corpus.rs"]
mod revalue_corpus;
#[path = "rv_dm7_probes.rs"]
mod rv_dm7_probes;
#[path = "takeover_door.rs"]
mod takeover_door;

#[path = "rv_matehead_probes.rs"]
mod rv_matehead_probes;

#[path = "a_boxed_rectangle_certifies_over_its_box.rs"]
mod a_boxed_rectangle_certifies_over_its_box;
#[path = "a_taken_slab_over_a_box_is_never_refused_as_a_flip.rs"]
mod a_taken_slab_over_a_box_is_never_refused_as_a_flip;
#[path = "gui1_pick.rs"]
mod gui1_pick;
#[path = "gui1_pick_r2.rs"]
mod gui1_pick_r2;
#[path = "lib_dietool_crossing.rs"]
mod lib_dietool_crossing;
#[path = "lib_doors_node_result.rs"]
mod lib_doors_node_result;
#[path = "lib_g14_split_walls.rs"]
mod lib_g14_split_walls;
#[path = "lib_g16_blend_messages.rs"]
mod lib_g16_blend_messages;
#[path = "lib_g16_chamfer_node.rs"]
mod lib_g16_chamfer_node;
#[path = "lib_g16_corpus_name_digests.rs"]
mod lib_g16_corpus_name_digests;
#[path = "lib_g17_r1_probes.rs"]
mod lib_g17_r1_probes;
#[path = "lib_g17_r2_probes.rs"]
mod lib_g17_r2_probes;
#[path = "lib_g17_shell_node.rs"]
mod lib_g17_shell_node;
#[path = "lib_placedunion.rs"]
mod lib_placedunion;
#[path = "lib_sel1_geoselect.rs"]
mod lib_sel1_geoselect;
#[path = "lib_u5_interrogate.rs"]
mod lib_u5_interrogate;
#[path = "lib_u7_select.rs"]
mod lib_u7_select;
#[path = "load_door_payload_param_ref.rs"]
mod load_door_payload_param_ref;
#[path = "load_door_program_validate.rs"]
mod load_door_program_validate;
#[path = "load_door_slot_dimension.rs"]
mod load_door_slot_dimension;
#[path = "loft_section_ids.rs"]
mod loft_section_ids;
#[path = "m10_1_analysis.rs"]
mod m10_1_analysis;
#[path = "m10_1_distribution_wire.rs"]
mod m10_1_distribution_wire;
#[path = "m10_1_r2_probes.rs"]
mod m10_1_r2_probes;
#[path = "m10_2_measure.rs"]
mod m10_2_measure;
#[path = "m10_2_measure_wire.rs"]
mod m10_2_measure_wire;
#[path = "m10_2_r1_probes.rs"]
mod m10_2_r1_probes;
#[path = "m10_3_driver_interval.rs"]
mod m10_3_driver_interval;
#[path = "m10_3_driver_k_probe_interval.rs"]
mod m10_3_driver_k_probe_interval;
#[path = "m10_3_r1_probes_interval.rs"]
mod m10_3_r1_probes_interval;
#[path = "m10_3_r2_probes_interval.rs"]
mod m10_3_r2_probes_interval;
#[path = "m10_4_r1_probes_interval.rs"]
mod m10_4_r1_probes_interval;
#[path = "m10_4_seed.rs"]
mod m10_4_seed;
#[path = "maintenance_net.rs"]
mod maintenance_net;
#[path = "meet_by_construction.rs"]
mod meet_by_construction;
#[path = "member_budget.rs"]
mod member_budget;
#[path = "msolve10_door_admission.rs"]
mod msolve10_door_admission;
#[path = "msolve11_mate_log.rs"]
mod msolve11_mate_log;
#[path = "msolve13_read_at_operand.rs"]
mod msolve13_read_at_operand;
#[path = "msolve14_run_scalar.rs"]
mod msolve14_run_scalar;
#[path = "msolve1_transform_aware.rs"]
mod msolve1_transform_aware;
#[path = "msolve2_member_chain.rs"]
mod msolve2_member_chain;
#[path = "msolve3_placer_refused.rs"]
mod msolve3_placer_refused;
#[path = "msolve4_mate_memo.rs"]
mod msolve4_mate_memo;
#[path = "msolve5_operand_refusals.rs"]
mod msolve5_operand_refusals;
#[path = "msolve6_part_extent.rs"]
mod msolve6_part_extent;
#[path = "msolve7_member_residue.rs"]
mod msolve7_member_residue;
#[path = "msolve8_levered_clash.rs"]
mod msolve8_levered_clash;
#[path = "msolve9_from_face.rs"]
mod msolve9_from_face;
#[path = "on_verdict_rows.rs"]
mod on_verdict_rows;
#[path = "onb_seam_class_interval.rs"]
mod onb_seam_class_interval;
#[path = "onb_wall_normal_census.rs"]
mod onb_wall_normal_census;
#[path = "onehome_switch_parity_interval.rs"]
mod onehome_switch_parity_interval;
#[path = "rv_payloadrefs_probes.rs"]
mod rv_payloadrefs_probes;

#[path = "expr_nesting_bound.rs"]
mod expr_nesting_bound;
#[path = "m10_4_r2_probes_interval.rs"]
mod m10_4_r2_probes_interval;
#[path = "m10_4_stackup_interval.rs"]
mod m10_4_stackup_interval;
#[path = "m10_5_clearance_interval.rs"]
mod m10_5_clearance_interval;
#[path = "m10_5_r1_probes_interval.rs"]
mod m10_5_r1_probes_interval;
#[path = "m10_5_r2_probes_interval.rs"]
mod m10_5_r2_probes_interval;
#[path = "m10_6_ci_rows_interval.rs"]
mod m10_6_ci_rows_interval;
#[path = "m10_6_clearance_assertion_interval.rs"]
mod m10_6_clearance_assertion_interval;
#[path = "m10_6_mc_draws.rs"]
mod m10_6_mc_draws;
#[path = "m10_6_r1_probes_interval.rs"]
mod m10_6_r1_probes_interval;
#[path = "m10_6_reports_interval.rs"]
mod m10_6_reports_interval;
#[path = "m10_7_census_probe.rs"]
mod m10_7_census_probe;
#[path = "m10_7_lever.rs"]
mod m10_7_lever;
#[path = "m10_7_plate.rs"]
mod m10_7_plate;
#[path = "m10_7_probe_interval.rs"]
mod m10_7_probe_interval;
#[path = "m10_7_r1_census_probe.rs"]
mod m10_7_r1_census_probe;
#[path = "m10_7_r1_probes_interval.rs"]
mod m10_7_r1_probes_interval;
#[path = "m10_di_dual_corpus.rs"]
mod m10_di_dual_corpus;
#[path = "m10_p_fence.rs"]
mod m10_p_fence;
#[path = "m10_p_lift.rs"]
mod m10_p_lift;
#[path = "m4_pr1_dims.rs"]
mod m4_pr1_dims;
#[path = "m4_pr1_doc.rs"]
mod m4_pr1_doc;
#[path = "m4_pr1_eval.rs"]
mod m4_pr1_eval;
#[path = "m4_pr1_paths.rs"]
mod m4_pr1_paths;
#[path = "m4_pr2_eval.rs"]
mod m4_pr2_eval;
#[path = "m4_pr2_eval_interval.rs"]
mod m4_pr2_eval_interval;
#[path = "m4_pr2_frame.rs"]
mod m4_pr2_frame;
#[path = "m4_pr2_wire.rs"]
mod m4_pr2_wire;
#[path = "m4_pr3_names.rs"]
mod m4_pr3_names;
#[path = "m4_pr3_names_bool.rs"]
mod m4_pr3_names_bool;
#[path = "m4_pr3_names_ci.rs"]
mod m4_pr3_names_ci;
#[path = "m4_pr3_names_interval.rs"]
mod m4_pr3_names_interval;
#[path = "m4_pr3_names_rework.rs"]
mod m4_pr3_names_rework;
#[path = "m4_pr4_appearance_hook.rs"]
mod m4_pr4_appearance_hook;
#[path = "m4_pr4_banked.rs"]
mod m4_pr4_banked;
#[path = "m4_pr4_ci.rs"]
mod m4_pr4_ci;
#[path = "m4_pr4_diff.rs"]
mod m4_pr4_diff;
#[path = "m4_pr4_edits.rs"]
mod m4_pr4_edits;
#[path = "m4_pr4_hit.rs"]
mod m4_pr4_hit;
#[path = "m4_pr4_interval.rs"]
mod m4_pr4_interval;
#[path = "m4_pr4_resolve.rs"]
mod m4_pr4_resolve;
#[path = "m4_pr5_glue.rs"]
mod m4_pr5_glue;
#[path = "m4_pr6_eps_diff.rs"]
mod m4_pr6_eps_diff;
#[path = "m4_pr6_floats.rs"]
mod m4_pr6_floats;
#[path = "m4_pr6_golden.rs"]
mod m4_pr6_golden;
#[path = "m4_pr6_refusal.rs"]
mod m4_pr6_refusal;
#[path = "m4_pr6_review_probes.rs"]
mod m4_pr6_review_probes;
#[path = "m4_pr6_roundtrip.rs"]
mod m4_pr6_roundtrip;
#[path = "m4_pr7_appearance.rs"]
mod m4_pr7_appearance;
#[path = "m4_pr7_appearance_interval.rs"]
mod m4_pr7_appearance_interval;
#[path = "m4_pr8_corpus.rs"]
mod m4_pr8_corpus;
#[path = "m4_pr8_corpus_interval.rs"]
mod m4_pr8_corpus_interval;
#[path = "m4_pr8_k_probe.rs"]
mod m4_pr8_k_probe;
#[path = "m4_pr8_latency.rs"]
mod m4_pr8_latency;
#[path = "m5_pr10_nodes.rs"]
mod m5_pr10_nodes;
#[path = "m5_pr11_corpus_curved.rs"]
mod m5_pr11_corpus_curved;
#[path = "m5_pr12_fillet_node.rs"]
mod m5_pr12_fillet_node;
#[path = "m5_pr5_corpus.rs"]
mod m5_pr5_corpus;
#[path = "m5_pr5_corpus_probe.rs"]
mod m5_pr5_corpus_probe;
#[path = "m5_pr6_pcurve_persistence.rs"]
mod m5_pr6_pcurve_persistence;
#[path = "m5_pr8_bvh_diff.rs"]
mod m5_pr8_bvh_diff;
#[path = "m5_s1_rest_union.rs"]
mod m5_s1_rest_union;
#[path = "m6_5_downstream.rs"]
mod m6_5_downstream;
#[path = "m6_5_selection_refusals.rs"]
mod m6_5_selection_refusals;
#[path = "m6_5_selection_wire.rs"]
mod m6_5_selection_wire;
#[path = "m6_composed_node.rs"]
mod m6_composed_node;
#[path = "m9_d1_r1_probes.rs"]
mod m9_d1_r1_probes;
#[path = "m9_d1_r2_probes.rs"]
mod m9_d1_r2_probes;
#[path = "mate1_member_vocab.rs"]
mod mate1_member_vocab;
#[path = "mate1_r1_probes.rs"]
mod mate1_r1_probes;
#[path = "mate1r2_probes.rs"]
mod mate1r2_probes;
#[path = "mate6_gather_mints.rs"]
mod mate6_gather_mints;
#[path = "mate6r2_probes.rs"]
mod mate6r2_probes;
#[path = "meta_minted_ids.rs"]
mod meta_minted_ids;
#[path = "meta_nesting_bound.rs"]
mod meta_nesting_bound;
#[path = "name_depth.rs"]
mod name_depth;
#[path = "name_invariance.rs"]
mod name_invariance;
#[path = "name_size_against_cut_depth.rs"]
mod name_size_against_cut_depth;
#[path = "name_tables_by_position.rs"]
mod name_tables_by_position;
#[path = "name_words_corpus.rs"]
mod name_words_corpus;
#[path = "name_words_rows.rs"]
mod name_words_rows;
#[path = "names_verbatim_edge_evaluator.rs"]
mod names_verbatim_edge_evaluator;
#[path = "node_labels.rs"]
mod node_labels;
#[path = "node_standing.rs"]
mod node_standing;
#[path = "p2_face.rs"]
mod p2_face;
#[path = "p2_gauge_offsets_and_spaces.rs"]
mod p2_gauge_offsets_and_spaces;
#[path = "p2_gauge_poses_and_doors.rs"]
mod p2_gauge_poses_and_doors;
#[path = "p2_gauges.rs"]
mod p2_gauges;
#[path = "p2_split.rs"]
mod p2_split;
#[path = "pair_seam_name_order.rs"]
mod pair_seam_name_order;
#[path = "parallel_node_map_interval.rs"]
mod parallel_node_map_interval;
#[path = "parallel_node_map_probe.rs"]
mod parallel_node_map_probe;
#[path = "part_depth_bound.rs"]
mod part_depth_bound;
#[path = "parts_entered_before_the_map.rs"]
mod parts_entered_before_the_map;
#[path = "perf12_census_bvh_diff.rs"]
mod perf12_census_bvh_diff;
#[path = "perf12_census_goldens.rs"]
mod perf12_census_goldens;
#[path = "perf2_name_keying_differential.rs"]
mod perf2_name_keying_differential;
#[path = "pick3_early_out.rs"]
mod pick3_early_out;
#[path = "pierce_ring_engraving.rs"]
mod pierce_ring_engraving;
#[path = "pirad_wire.rs"]
mod pirad_wire;
#[path = "place_mate_frame_offset.rs"]
mod place_mate_frame_offset;
#[path = "product_finishes_once.rs"]
mod product_finishes_once;
#[path = "props_verdict_shapes.rs"]
mod props_verdict_shapes;

#[path = "peg_in_bore_union.rs"]
mod peg_in_bore_union;
#[path = "r1_bool11_ec_probe.rs"]
mod r1_bool11_ec_probe;
#[path = "r1_dual_probes.rs"]
mod r1_dual_probes;
#[path = "r1_m10_1_corruptions.rs"]
mod r1_m10_1_corruptions;
#[path = "r1_m10_1_probes.rs"]
mod r1_m10_1_probes;
#[path = "r2_cert3_coord_dump.rs"]
mod r2_cert3_coord_dump;
#[path = "r2_keydiff.rs"]
mod r2_keydiff;
#[path = "r2_m10_2_probes.rs"]
mod r2_m10_2_probes;
#[path = "r2_m10_6_probes_interval.rs"]
mod r2_m10_6_probes_interval;
#[path = "r2_m10_di_probes.rs"]
mod r2_m10_di_probes;
#[path = "rev_fix_xsplit_unreachable.rs"]
mod rev_fix_xsplit_unreachable;
#[path = "review_gui1_r1.rs"]
mod review_gui1_r1;
#[path = "review_m4_pr1.rs"]
mod review_m4_pr1;
#[path = "review_m4_pr1_die.rs"]
mod review_m4_pr1_die;
#[path = "review_m4_pr2.rs"]
mod review_m4_pr2;
#[path = "review_m5_pr10_sweep_node.rs"]
mod review_m5_pr10_sweep_node;
#[path = "review_m5_pr1_e2e_interval.rs"]
mod review_m5_pr1_e2e_interval;
#[path = "review_m5_pr9_doc_probe.rs"]
mod review_m5_pr9_doc_probe;
#[path = "review_m6_5_pr2_probes.rs"]
mod review_m6_5_pr2_probes;
#[path = "review_pick3_r1_probes.rs"]
mod review_pick3_r1_probes;
#[path = "review_pick3_r2_probes.rs"]
mod review_pick3_r2_probes;
#[path = "review_pick_r2_probes.rs"]
mod review_pick_r2_probes;
#[path = "ring_r1_names_probe.rs"]
mod ring_r1_names_probe;
#[path = "rv_onepred3_probes.rs"]
mod rv_onepred3_probes;
#[path = "scalar_frame_r1_probes.rs"]
mod scalar_frame_r1_probes;
#[path = "seat4_verb_lowering.rs"]
mod seat4_verb_lowering;
#[path = "seat7_sweep_lowering.rs"]
mod seat7_sweep_lowering;
#[path = "seat8_split_lowering.rs"]
mod seat8_split_lowering;
#[path = "select_member_read.rs"]
mod select_member_read;
#[path = "split_and_shell_carry_contacts.rs"]
mod split_and_shell_carry_contacts;
#[path = "split_carries_spaces.rs"]
mod split_carries_spaces;
#[path = "split_pinch_keeps.rs"]
mod split_pinch_keeps;
#[path = "step_handle_binding.rs"]
mod step_handle_binding;
#[path = "switch_display_units.rs"]
mod switch_display_units;
#[path = "switch_dump.rs"]
mod switch_dump;
#[path = "switch_naming.rs"]
mod switch_naming;
#[path = "switch_plate_param.rs"]
mod switch_plate_param;
#[path = "switch_program_key.rs"]
mod switch_program_key;
#[path = "switch_program_vocabulary.rs"]
mod switch_program_vocabulary;
#[path = "switch_slots.rs"]
mod switch_slots;
#[path = "trim_3_windows_interval.rs"]
mod trim_3_windows_interval;
#[path = "u8a_parse.rs"]
mod u8a_parse;
#[path = "union_defects_in_evaluation_order.rs"]
mod union_defects_in_evaluation_order;
#[path = "union_flush_onto_edge_contact.rs"]
mod union_flush_onto_edge_contact;
#[path = "union_fold_face_face_touch.rs"]
mod union_fold_face_face_touch;
#[path = "union_pinch_member_order.rs"]
mod union_pinch_member_order;
#[path = "unreadable_by_this_build.rs"]
mod unreadable_by_this_build;

test_utils::every_suite_file_is_aggregated!();

#[path = "lib_tube_node.rs"]
mod lib_tube_node;

#[path = "lib_tube_r1_probes.rs"]
mod lib_tube_r1_probes;

#[path = "lib_tube_r1_dump.rs"]
mod lib_tube_r1_dump;

#[path = "lib_tube_r1_probes2.rs"]
mod lib_tube_r1_probes2;

#[path = "lib_tube_r2_probes.rs"]
mod lib_tube_r2_probes;

#[path = "m10_7_r2_probes_interval.rs"]
mod m10_7_r2_probes_interval;

#[path = "m10_8_arc_family_interval.rs"]
mod m10_8_arc_family_interval;
#[path = "m10_8_harness.rs"]
mod m10_8_harness;
#[path = "m10_8_pins_interval.rs"]
mod m10_8_pins_interval;
#[path = "m10_8_r1_probes_interval.rs"]
mod m10_8_r1_probes_interval;

#[path = "m10_8_r2_probes_interval.rs"]
mod m10_8_r2_probes_interval;

#[path = "m10_9_evidence_interval.rs"]
mod m10_9_evidence_interval;
#[path = "m10_9_pins_interval.rs"]
mod m10_9_pins_interval;

#[path = "m10_9_r2_probes_interval.rs"]
mod m10_9_r2_probes_interval;

#[path = "kernel_runs_take_the_callers_stop.rs"]
mod kernel_runs_take_the_callers_stop;
#[path = "kstats_bracket_rows.rs"]
mod kstats_bracket_rows;

#[path = "m10_9_r1_probes_interval.rs"]
mod m10_9_r1_probes_interval;

#[path = "sym_9_retry_interval.rs"]
mod sym_9_retry_interval;

#[path = "m10_10_evidence_interval.rs"]
mod m10_10_evidence_interval;
#[path = "m10_10_pins_interval.rs"]
mod m10_10_pins_interval;

#[path = "m10_10_r1_probes_interval.rs"]
mod m10_10_r1_probes_interval;
#[path = "m10_10_r2_probes_interval.rs"]
mod m10_10_r2_probes_interval;
#[path = "m10_bulge_interval.rs"]
mod m10_bulge_interval;
#[path = "m10_derived_frame_interval.rs"]
mod m10_derived_frame_interval;
#[path = "m10_derived_frame_tilted_interval.rs"]
mod m10_derived_frame_tilted_interval;
#[path = "m10_sym_drive_memo_interval.rs"]
mod m10_sym_drive_memo_interval;
#[path = "m10_sym_profile_interval.rs"]
mod m10_sym_profile_interval;
#[path = "sym11_exact_channel_rows.rs"]
mod sym11_exact_channel_rows;

#[path = "wire_band_cause.rs"]
mod wire_band_cause;
#[path = "wire_legal_union_refusals.rs"]
mod wire_legal_union_refusals;
#[path = "wire_operand_door.rs"]
mod wire_operand_door;
#[path = "wire_product_gather_tie.rs"]
mod wire_product_gather_tie;
#[path = "wire_rv_bytes.rs"]
mod wire_rv_bytes;
#[path = "wire_rv_unknown.rs"]
mod wire_rv_unknown;
#[path = "wire_selection_kind_doors.rs"]
mod wire_selection_kind_doors;

#[path = "a_keel_ring_names_its_two_edges.rs"]
mod a_keel_ring_names_its_two_edges;
#[path = "a_mate_on_a_full_turn_bore_names.rs"]
mod a_mate_on_a_full_turn_bore_names;
#[path = "a_mirror_is_a_construction.rs"]
mod a_mirror_is_a_construction;
#[path = "decide_1_self_dot_interval.rs"]
mod decide_1_self_dot_interval;
#[path = "decide_3_split_rows_interval.rs"]
mod decide_3_split_rows_interval;
#[path = "decide_6_read_cost_interval.rs"]
mod decide_6_read_cost_interval;
#[path = "decide_7_rule_g_cost_interval.rs"]
mod decide_7_rule_g_cost_interval;
#[path = "edit_refusal_recourse.rs"]
mod edit_refusal_recourse;
#[path = "emit_crossing_sense.rs"]
mod emit_crossing_sense;
#[path = "emit_curved_rim_chord.rs"]
mod emit_curved_rim_chord;
#[path = "emit_edge_piece_locality.rs"]
mod emit_edge_piece_locality;
#[path = "emit_nested_union_flat.rs"]
mod emit_nested_union_flat;
#[path = "emit_pair_cut_and_merged.rs"]
mod emit_pair_cut_and_merged;
#[path = "emit_per_face_parent.rs"]
mod emit_per_face_parent;
#[path = "emit_seam_edge_merged.rs"]
mod emit_seam_edge_merged;
#[path = "emit_seam_junction.rs"]
mod emit_seam_junction;
#[path = "emit_shared_rim_several.rs"]
mod emit_shared_rim_several;
#[path = "emit_sphere_cut_in_cap.rs"]
mod emit_sphere_cut_in_cap;
#[path = "emit_split_duplicate.rs"]
mod emit_split_duplicate;
#[path = "emit_split_edge_lineage.rs"]
mod emit_split_edge_lineage;
#[path = "emit_union_borders.rs"]
mod emit_union_borders;
#[path = "emit_union_curved_flush.rs"]
mod emit_union_curved_flush;
#[path = "emit_union_flush_names.rs"]
mod emit_union_flush_names;
#[path = "emit_union_rim_piece_ranks.rs"]
mod emit_union_rim_piece_ranks;
#[path = "emit_union_seam_rank.rs"]
mod emit_union_seam_rank;
#[path = "glue_on_zero_rows.rs"]
mod glue_on_zero_rows;
#[path = "intent_dprime1_target_space.rs"]
mod intent_dprime1_target_space;
#[path = "intent_dprime2_mates_read_geometry.rs"]
mod intent_dprime2_mates_read_geometry;
#[path = "intent_dprime3_assemble_product_slot.rs"]
mod intent_dprime3_assemble_product_slot;
#[path = "intent_e_operation_frame_of_reads.rs"]
mod intent_e_operation_frame_of_reads;
#[path = "intent_literals_a_definitions.rs"]
mod intent_literals_a_definitions;
#[path = "intent_literals_b_door.rs"]
mod intent_literals_b_door;
#[path = "intent_literals_c_slots.rs"]
mod intent_literals_c_slots;
#[path = "intent_literals_d_constants.rs"]
mod intent_literals_d_constants;
#[path = "intent_loft_set_sections.rs"]
mod intent_loft_set_sections;
#[path = "intent_part_census_seam.rs"]
mod intent_part_census_seam;
#[path = "intent_part_split_half.rs"]
mod intent_part_split_half;
#[path = "intent_placement_carries_at_rest.rs"]
mod intent_placement_carries_at_rest;
#[path = "intent_s2_a_outputs.rs"]
mod intent_s2_a_outputs;
#[path = "intent_s2_b_reads.rs"]
mod intent_s2_b_reads;
#[path = "intent_s2_c_world.rs"]
mod intent_s2_c_world;
#[path = "intent_s2_d_measure.rs"]
mod intent_s2_d_measure;
#[path = "intent_s2_e_select.rs"]
mod intent_s2_e_select;
#[path = "intent_s2_f_mates.rs"]
mod intent_s2_f_mates;
#[path = "intent_s3_a_poses.rs"]
mod intent_s3_a_poses;
#[path = "intent_s3_a_prime_moves.rs"]
mod intent_s3_a_prime_moves;
#[path = "intent_s3_b_placements.rs"]
mod intent_s3_b_placements;
#[path = "intent_s3_c_mates.rs"]
mod intent_s3_c_mates;
#[path = "intent_s3_d_placements.rs"]
mod intent_s3_d_placements;
#[path = "intent_s4_h_placed.rs"]
mod intent_s4_h_placed;
#[path = "intent_s5_a_relation.rs"]
mod intent_s5_a_relation;
#[path = "intent_s5_b_interference.rs"]
mod intent_s5_b_interference;
#[path = "intent_s5_c_at_rest_check.rs"]
mod intent_s5_c_at_rest_check;
#[path = "intent_s5_c_quietable.rs"]
mod intent_s5_c_quietable;
#[path = "intent_s6_c_trace.rs"]
mod intent_s6_c_trace;
#[path = "intent_s6_d_tangency.rs"]
mod intent_s6_d_tangency;
#[path = "intent_s6_e_traced_on_a_face.rs"]
mod intent_s6_e_traced_on_a_face;
#[path = "intent_unused_check.rs"]
mod intent_unused_check;
#[path = "intent_value_chart_orientation.rs"]
mod intent_value_chart_orientation;
#[path = "intent_vars_2_table.rs"]
mod intent_vars_2_table;
#[path = "intent_vars_3_readers.rs"]
mod intent_vars_3_readers;
#[path = "review_s5b_r2_probes.rs"]
mod review_s5b_r2_probes;
#[path = "run_wall_offers.rs"]
mod run_wall_offers;
#[path = "source_readers.rs"]
mod source_readers;
#[path = "split_joined_keys_tie.rs"]
mod split_joined_keys_tie;
#[path = "union_flush_fuses.rs"]
mod union_flush_fuses;
#[path = "union_member_orders_decide_alike.rs"]
mod union_member_orders_decide_alike;
#[path = "value_chart_sweeps.rs"]
mod value_chart_sweeps;
#[path = "vessel_cavity_interval.rs"]
mod vessel_cavity_interval;
// Review probe for PR 7264, not for merge: spells the slow set so CI runs it.
// a5_certified_clearance_runs_over_a_seated_mate a7_the_width_ladder_of_the_split_of_a_widened_box a_band_contributor_refuses_the_rss_whole_naming_every_band a_band_costs_its_costlier_bound a_band_parameter_certifies_normally_and_prices_nothing a_bar_through_a_sunk_boss_names_its_merged_wall_as_the_slot_does a_blend_bounded_by_a_recorded_vertex_refuses_the_touch a_blend_clear_of_a_pinch_carries_the_unions_records a_blends_turn_is_quieted_by_no_gap_and_never_refuses a_blind_bore_pin_in_a_turned_frame_is_quietable a_blind_bore_pin_is_not_quieted_by_the_bores_gap a_blind_bore_pin_is_quietable_and_refuses_until_quieted a_block_over_the_wedges_junction_builds_in_every_member_order a_boolean_document_round_trips_byte_identical a_booleans_glue_is_quiet_under_its_gap_and_nothing_else a_boss_on_a_box_mated_flat_onto_the_cap_is_tangent_for_every_tilt a_boss_on_a_revolved_band_is_unproven a_boss_on_one_piece_is_not_cited_and_one_straddling_the_divider_is a_boss_on_one_piece_of_a_covered_face_is_cited_in_no_order a_boss_on_the_bottom_cap_stands_outward_and_its_pocket_runs_up_the_wall a_boss_on_the_top_cap_stands_on_the_top a_boss_traced_off_the_walls_axis_is_tangent_by_construction a_boss_traced_on_a_hole_drilled_through_the_same_chart_proves a_boss_traced_on_one_cylinder_is_unproven_against_its_image a_boss_traced_on_the_walls_other_half_proves a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency a_branch_change_is_a_decision_flip_and_names_the_predicate a_bridge_folded_after_both_wedges_seams_their_walls a_bridge_met_with_the_wedges_holds_each_corner_on_the_line_as_two_copies a_budget_too_small_is_indeterminate_and_never_a_bound a_bumped_parameter_at_dual64_recomputes_exactly_its_cone a_bushing_stacked_on_its_own_axis_glues_its_walls a_cap_and_a_bar_over_a_sunk_boss_name_alike_in_every_order a_carried_census_row_is_the_parts_verdict_and_none_is_the_hosts a_carried_contact_is_quiet_by_the_parts_assertion_alone a_carried_door_job_is_pending_cancelable_and_settles a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line a_chain_of_unions_names_past_every_stack_through_every_door_on_the_smallest_stack a_chain_one_past_the_bound_refuses_depth_exceeded_on_the_smallest_stack a_chain_with_a_disjoint_member_fuses_in_every_order a_chord_across_the_glued_pieces_line_names_wherever_the_cut_crosses a_chord_between_two_merged_faces_builds_in_a_fold_and_after_a_split a_chord_between_two_merged_faces_is_the_seam_of_their_parents a_coaxial_press_fit_is_quietable a_coincidence_of_poses_is_quiet_under_its_distance a_collar_placed_by_values_is_two_axes_and_the_finding_names_the_mate a_consumer_drives_a_two_parameter_document_at_four_widths a_content_key_moves_exactly_when_the_report_does a_corner_the_tool_passes_through_is_not_on_it_by_construction a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider a_covered_contact_decides_alike_in_every_order
// a_crossing_of_a_merged_rim_is_named_the_same_in_every_order_that_fuses a_cup_a_size_edit_puts_inside_a_bands_region_vanishes_on_the_blends_verdict a_cusp_loft_document_gathers_with_its_nurbs_seam_unjudged_by_kind a_decision_flips_within_contains_a_value_that_does_not_build a_degraded_tangent_forfeits_the_pruning_and_nothing_else a_distribution_changes_no_content_key_at_interval a_distribution_changes_no_content_key_naming_key_or_verdict a_divider_beside_a_joined_seam_does_not_reach_across_it a_driver_certified_leaf_carries_the_certificate a_face_cut_and_merged_in_one_step_names_each_piece_for_what_it_holds a_face_through_a_seam_is_the_meeting_the_step_that_meets_it_decides a_failed_row_said_within_the_largest_table_is_cheap_again a_family_of_boolean_results_reads_whole_and_by_index a_familys_carry_is_every_members_whichever_order_they_stand_in a_far_feature_moves_no_other_name a_far_side_written_through_the_half_angle_proves_every_seat a_fillet_inserted_after_a_kept_leg_joins_its_wall_unreported a_fillet_moved_off_a_corner_reports_exactly_what_it_takes a_filleted_box_records_one_unproven_turn_per_corner a_filleted_disc_records_its_coaxial_supports_and_tangent_joints a_flat_union_of_plate_boss_and_bar_names_the_glue a_floor_swept_through_a_quarter_turn_moves_its_copy_continuously a_flush_glue_is_quiet_under_its_gap a_flush_rest_glues_and_records_its_unproven_rows a_flush_tile_beside_t1_renames_only_the_faces_gaining_its_area a_flush_union_publishes_one_table_in_every_member_order a_foreign_verdict_or_box_refuses_typed a_four_member_chain_fuses_in_every_order a_frame_read_off_a_rim_proves_by_its_heading a_frame_slid_spun_or_flipped_along_the_axis_still_proves a_frame_spun_by_a_variable_keeps_its_heading_only_about_its_z a_frame_turned_in_radians_asks_for_the_axis_read_off_the_face a_freezing_drive_is_identical_across_schedules_and_the_dial a_gap_between_two_lines_is_a_decision_flip_below_the_crossing a_glued_rest_and_its_corners_are_unproven_rows_named_by_their_operands a_headed_press_fit_is_not_quietable a_heading_typed_along_the_tangent_is_unproven a_held_flush_stretch_is_named_for_every_member_that_holds_it a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis a_holed_cylinder_wall_drops_the_cells_in_its_hole a_huge_ring_nobody_cancels_runs_out_before_it_builds a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time a_leaf_under_two_middles_is_reported_once a_loft_placed_on_a_rotated_frame_certifies_under_a_box_elsewhere a_loft_section_seed_rides_the_section_to_the_measure a_lying_oracle_is_indistinguishable_at_the_seam a_matched_pair_gathers_bit_identically a_mate_on_a_full_turn_bore_names a_mate_states_its_own_seat_and_no_other_row a_member_engulfing_the_others_reads_only_what_follows_it a_member_order_moves_no_name a_merged_face_cites_the_verdict_of_the_member_face_it_holds a_name_two_member_orders_both_publish_denotes_the_same_geometry a_named_scope_asks_what_a_name_denotes_before_how_many_answer
// a_negative_band_is_not_intersected_with_the_canonical_turn a_part_instanced_twice_is_replayed_once a_part_two_documents_down_is_reported_once_by_its_route a_partial_revolve_band_reports_its_phantom_turn a_partly_proven_part_carries_its_own_verdict a_parts_unproven_rest_is_reported_through_its_instance a_pick_of_a_pick_at_a_seat_evaluates_and_splits a_pick_of_a_pick_crosses_as_its_composed_path a_piece_of_the_plates_face_still_holds_the_line a_pieces_name_grows_at_most_linearly_with_the_cuts_it_sits_under a_pinch_rows_tool_cell_is_the_tool_read_for_every_pose_tool a_pinch_union_builds_one_body_in_every_member_order a_pinch_union_publishes_one_table_in_every_member_order a_placed_halfs_crossings_are_not_on_the_unplaced_face_by_construction a_planted_flip_refuses_flip_crossing_and_names_what_flipped a_planted_violated_reads_violated_over_a_certified_leaf a_plate_swept_through_all_but_the_pole_moves_its_hung_copy_continuously a_point_polygon_loft_certifies_under_a_box_elsewhere a_posed_placement_moves_its_copy_and_its_records_rigidly a_prism_bridging_two_wedges_is_named_alike_in_every_member_order_that_builds a_prism_crossing_one_wedge_about_the_line_publishes_one_table_in_every_member_order a_prong_on_the_mirror_plane_by_value_alone_is_left_unproven a_quarter_ball_beside_its_missing_quarters_holds_against_a_dense_distance a_recorded_edge_the_blend_ends_on_keeps_its_record a_refused_bundle_states_nothing a_refusing_finding_has_a_gap_zero_that_holds a_rename_recomputes_nothing_and_mints_nothing a_report_settled_at_rest_is_the_synchronous_report a_reshaping_reports_exactly_the_held_names_whose_referent_it_takes a_revolved_quarter_band_is_windowed_and_holds a_revolves_axis_lies_in_its_profiles_plane_over_a_box a_rim_on_the_other_cavitys_floor_meets_its_rim_where_it_meets_its_wall a_ring_of_free_member_angles_proves_no_seat a_ring_of_unit_blocks_meeting_on_its_axis_judges_every_pair a_ring_on_a_tilted_axis_proves_every_side_seat a_ring_serving_two_pierces_records_their_fusion_unproven a_ring_stepped_by_a_free_step_leaves_the_closing_seat_alone_unproven a_ring_stepped_by_an_exact_turn_proves_every_side_seat a_ring_with_a_member_at_a_rational_cosine_proves_every_side_seat a_rod_on_a_plate_is_quiet_under_its_gap a_rounded_plate_chamfered_by_name_names_each_section_once_and_keeps_its_names a_rounded_pockets_mouth_filleted_by_name_removes_pappuss_volume a_seat_between_two_constructions_of_one_carrier_proves a_second_cut_of_a_union_seams_pieces_follows_it_too a_second_subtract_cites_only_member_faces_lying_under_each_face a_section_edge_lies_on_the_wall_it_runs_across a_session_decides_and_counts_as_the_serial_walk_at_one_and_four_threads a_slab_across_a_round_boss_builds_in_every_order a_slab_across_the_line_leaves_the_touch_on_its_pieces a_slab_across_the_whole_face_pairs_each_piece_with_its_own_side a_slab_placed_then_unioned_certifies_in_one_leaf a_slab_under_a_taker_over_wider_boxes_is_never_refused_as_a_flip
// a_sliver_wrapped_in_the_ops_own_error_is_priced_sliver_terminal_not_budget a_split_and_an_inline_carry_a_loft_walls_name_onto_the_minted_sections a_split_document_read_by_port_round_trips_byte_identical a_split_keeps_its_names_when_the_splitting_feature_moves a_split_planes_frame_moves_no_name a_splits_on_row_is_quiet_under_its_tool_gap a_sqrt_zero_tangent_forfeits_every_parameter_and_a_max_kink_forfeits_none a_staircase_cites_every_wall_along_its_side a_stale_or_foreign_verdict_is_refused_by_content a_stated_axis_proves_a_wall_pair_the_fold_does_not a_strip_cup_a_size_edit_puts_inside_a_bands_region_vanishes_where_the_blends_reach_parts a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order a_subtracts_chord_between_two_merged_faces_is_named_alike_in_every_symmetry a_supplementary_chamfer_turn_is_recorded_unproven a_symbolic_run_builds_a_part_and_the_door_leaves_a_row_on_one_unreplayed a_symmetric_parts_seam_is_its_face_read_twice a_tangent_arc_leaving_the_trace_proves a_terminal_sliver_refuses_naming_its_predicate_and_is_not_refined a_three_member_intersect_of_a_sunk_boss_names_alike_in_every_order a_tie_carried_through_a_later_fold_step_counts_one_parent a_tied_faces_joined_seam_beside_the_merge_is_the_bare_name a_tied_parents_seam_group_counts_its_own_pieces a_tilt_within_one_topology_moves_no_name a_tilted_cylinders_boss_is_tangent_for_every_tilt a_tilted_press_fit_is_quiet_and_its_axes_are_decided_in_band a_tube_islands_the_top_and_a_slab_divides_tube_and_top a_union_across_a_nurbs_edge_refuses_where_two_arcs_pads_meet a_union_cites_only_what_the_finished_body_holds a_union_over_a_union_publishes_flat_sets_in_every_order a_union_records_a_vertex_touch_once a_unions_corner_held_twice_then_once_reads_two_to_one_in_every_member_order a_unions_group_resized_at_any_fold_step_reads_two_to_one a_vertex_cites_a_member_edge_whole_and_lies_on_it a_vertex_read_twice_names_alike_with_its_vertex_pairs_reversed a_vertex_read_twice_names_its_result_in_every_op a_vertex_read_twice_names_its_result_in_every_op_at_every_pose a_vertex_row_is_the_unions_once_in_every_member_order a_wall_across_two_flush_arcs_publishes_one_table_in_every_order a_wall_against_itself_reads_zero_and_violates_its_at_least_bound a_wall_whose_two_ends_are_two_variables_is_not_one_radius a_wedge_resting_on_its_sloped_face_stays_on_a_tilting_floor a_wide_box_over_a_copy_certifies_whole a_wrapped_escalation_never_certifies_inside_the_band adversarial_rows_bit_equal_superset_and_ordered an_aligned_boss_on_the_other_piece_is_not_cited an_arc_crossed_twice_reunites_by_construction an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere an_authoring_history_moves_no_name an_earlier_seam_split_and_joined_away_in_one_step_names_alike an_edge_whose_member_face_was_glued_away_is_a_seam_in_every_member_order an_f64_profile_log_and_an_interval_one_diff_to_no_flip an_instance_under_a_boolean_still_reports_its_parts_rows
// an_l_shaped_face_holds_where_it_has_no_material an_offset_cylinder_keeps_its_boss_through_a_centre_edit an_operand_order_moves_no_name an_unproven_fold_row_is_reported_under_the_judged_row_decided_at_its_point an_unseeded_evaluation_is_the_build an_untoleranced_parameter_is_no_axis_and_blocks_no_rss an_upper_bound_over_a_box_is_proved_at_every_point_or_not_at_all an_upper_bound_over_a_notch_is_proved_only_by_face_points appearance_only_edit_recomputes_zero_nodes at_rest_census_verdicts_match_f64_at_interval at_sym_a_fixed_box_is_the_build attribute_survives_no_flip_parameter_motion_on_the_die boss_union_gains_tessellation_and_props_columns both_documents_evaluate_green_at_the_interval_scalar c5_one_documents_structure_is_the_same_in_every_lane_and_the_dual_value_is_f64s cancelation_returns_a_typed_partial_result canceled_partials_are_typed_and_never_pollute_the_memo chained_pair_unions_fold_a_flush_partner_onto_the_edge_contact containment_does_not_fire_while_a_boundary_leaf_certifies containment_fires_when_the_chamber_is_strictly_inside_the_box corpus_boolean_operands_superset_pin corpus_evaluations_bit_equal_at_interval corpus_evaluations_bit_equal_realized_vs_idealized corpus_rows_bit_equal_superset_and_ordered crossing_axes_are_quiet_under_their_distance_and_a_radius_is_not crossings_of_an_arc_cut_twice_name_apart_by_sense_in_every_member_order crossings_of_one_open_arc_name_apart_by_sense_in_every_member_order crossings_of_seams_with_no_start_name_apart_by_sense_in_every_member_order cut_cylinder_gains_tessellation_and_props_columns decide_3_no_predicate_loses_a_decision decide_6_the_profile_decides_nothing decide_7_the_profile_decides_nothing deep_value_channel_identity_f64_vs_dual64_including_carrier_arenas deep_value_channel_identity_interval_vs_dual_interval degenerate_queries_land_in_an_arm_with_a_holding_receipt die_name_tables_are_golden direct_validation_door_behavior_at_dual64 doc_param_edit_recomputes_the_param_cone document_names_are_unique_and_stable documents_outside_the_corpus_read_apart_too dual_interval_evaluates_with_the_interval_value_channel e2e_a_real_tolerance_gets_a_holds_over_zero_leaves e2e_channel_slider_over_an_epsilon_box early_exit_accounts_for_what_it_did_not_examine edit_back_restores_bit_identical_bodies equal_over_a_clearance_is_violated_off_either_bound evaluation_and_drop_keep_their_own_stack every_assertion_carrying_corpus_document_has_a_budget every_attached_radius_was_keyed_first every_band_a_revolve_mints_is_about_its_axis_output_at_sym every_configuration_lands_in_one_arm_with_a_holding_receipt every_corpus_document_reads_alike_in_a_second_computing_frame every_corpus_documents_name_tables_and_persisted_text_are_pinned every_corpus_documents_name_tables_are_golden every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget every_corpus_placement_places_its_body_by_the_pinned_bits every_corpus_space_is_a_space_variable every_corpus_step_is_answered_against_its_own_authored_geometry
// every_corpus_step_is_answered_and_the_answers_partition_the_loop every_cut_mints_exactly_its_straddling_member_edges every_decided_name_is_read_and_every_site_is_listed every_document_evaluates_at_dual64_with_the_f64_value_channel every_document_evaluates_green every_document_evaluates_green_at_interval every_edge_of_a_rounded_pocket_fillets_at_its_closed_form every_evaluation_literal_stamps_the_document every_fold_row_names_every_member_cell_its_cells_are_made_of every_formula_the_door_walks_refuses_typed_as_an_unheld_name every_leaf_reports_one_column_under_every_schedule_and_both_dials every_member_order_of_a_cut_seam_union_publishes_one_table every_moat_round_a_diamond_pocket_touching_the_block_walls_splits_along_its_floor every_node_publishes_the_heads_its_verbatim_edge_requires every_node_result_reader_goes_through_usable_or_is_listed every_registered_assertion_holds_over_the_certified_leaves_within_budget every_run_lands_in_exactly_one_arm_with_a_holding_receipt every_union_decides_alike_in_every_member_order every_union_of_the_emit_corpus_decides_alike_in_every_member_order every_walk_fixed_image_is_geometrically_fixed exact_mass_pins_hold face_frame_sense_matches_the_stored_flag_on_every_corpus_face four_flush_boxes_in_a_line_publish_one_table_in_every_order holes_touching_at_a_corner_build_one_body_in_every_member_order holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order incremental_recompute_reuses_the_cone_complement inversion_is_total_on_the_die islands_under_a_ring_rib_and_a_cup_border_their_inner_walls list_input_and_set_list_input_agree_on_every_node_kind localizing_reads_each_copy_into_the_products_frame_once m10_10_all_four_discharge_at_the_nominal_and_the_chart_phase_is_the_doors m10_10_the_plate_needs_no_door m10_10_the_plates_ceiling_is_the_flip m10_10_the_walked_rows_envelope_is_a_theorem_over_the_box m10_8_the_a0_set_is_a0_alone m10_9_no_registrant_lies_on_any_measured_document m10_9_the_pad_at_both_rule_f_dials m10_9_the_value_channel_is_untouched_on_a_certifying_box m10_9_what_the_registry_buys m10_bulge_the_bosss_split_at_the_nominal m10_bulge_the_d_tabs_literal_split_at_the_nominal m10_bulge_the_d_tabs_parameter_split_at_the_nominal m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does memo_reuse_transfers_tables_bit_identically moving_the_bar_across_the_tiles_joint_renames_only_the_faces_beside_it no_corpus_document_publishes_a_nested_merged_row no_corpus_face_sits_on_the_frame_seam no_order_of_the_area_overlap_union_with_a_fourth_member_refuses_a_fold_contact no_order_of_the_probe_corpus_refuses_several_shared_rims no_order_of_the_rows_documents_refuses_with_an_emission no_order_of_the_split_fixture_under_a_covering_block_refuses_a_fold_contact no_tangents_forfeits_the_pruning_and_nothing_else_on_the_comb one_assertion_over_a_family_quiets_each_members_own_finding
// outer_values_over_empty_inner_indices_run_out partial_overlaps_name_each_joined_edge_for_the_rims_it_runs_along pip_depth_motion_without_flips_leaves_every_table_identical poisoning_hits_descendants_only_and_is_walkable published_row_cells_are_member_cells_that_meet r1_a_real_tolerance_study_on_the_stepped_shaft r1_another_documents_verdict_certifies_this_one r1_clearance_answers_alike_in_either_order r1_dual_product_matches_f64_including_carriers r1_dual_value_channel_matches_f64_including_carriers r1_e2e_consumer_drive_at_dual64 r1_is_dl3s_measured_problem_reproducible r1_probe_a_split_by_a_face_plane_is_quiet_under_its_tool_gap r1_rss_totality_and_the_fixed_parameter_door r2_a_zero_term_budget_reproduces_the_tier_off_verdict r2_probe_a_split_by_a_faces_plane_is_quiet_under_its_tool_gap r2_the_door_moves_nothing_but_the_receipt_on_every_m10_fixture r2_the_shape_report_attributes_each_decision_to_its_own_door r2_two_convex_shafts_side_by_side_quiet_under_a_holding_gap real_ridges_whose_lines_meet_read_about_any_axis rebuild_latency_manifest_pins_the_corpus_structure regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle removing_any_pip_leaves_both_die_fillets_resolving report_key_tells_two_budgets_apart revalue_boss_union revalue_boss_union_apart_to_its_parting revalue_cut_cylinder revalue_declared_tangency revalue_die_chamfer revalue_heat_sink_fins revalue_loft_prism revalue_measured_web revalue_plate_param revalue_plate_param_apart_to_its_parting same_seed_replay_at_dual64_reuses_everything save_load_replay_identity scopes_that_meet_are_within_any_upper_bound_over_a_box separation_has_no_successor_of_its_own set_loft_sections_adds_drops_and_reorders shafts_side_by_side_quiet_under_a_holding_gap_at_every_bearing solids_touching_along_one_line_build_one_body_in_every_member_order sym11_the_exact_channel_never_contradicts_past_the_ceiling sym12_no_copysign_atom_reaches_a_decision_on_the_cheap_documents sym_9_the_drive_writes_the_ladders_receipt sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured the_accelerator_changes_no_verdict_on_the_comb the_analysis_lanes_read_a_round_gap_over_its_radius the_answer_is_deterministic_across_repeats the_at_least_arm_reads_violated_off_the_faces_upper_end the_at_rest_census_verdicts_are_goldened_bit_exact the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order the_balls_interval_union_encloses_its_closed_form_under_a_bounded_pad the_blend_documents_evaluate_to_their_committed_digests the_boolean_documents_evaluate_to_their_committed_digests the_bore_pin_gap_stackup_pins_the_lift the_bracket_walk_through_the_public_doors the_brackets_mixed_corners_name_each_entity_once_and_keep_their_names the_brackets_odd_plus_one_chamfer_names_each_turn_once_and_keeps_its_names the_bvh_prune_decides_inside_the_band_where_the_funnel_would_not the_cap_states_its_meridian_crossings_as_meetings the_census_door_is_absent_at_dual64_and_the_gather_matches_f64 the_certificate_is_inside_the_locally_valid_range
// the_certification_width_is_no_longer_bounded_by_epsilon the_certification_width_threshold_is_no_longer_a_fraction_of_epsilon the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean the_corpus_band_trim_census_is_the_recorded_one the_corpus_evaluation_is_bit_identical_at_f64 the_corpus_evaluation_is_bit_identical_at_interval the_corpus_geometry_is_bit_identical_with_ids_masked the_corpus_has_no_split_with_an_empty_side the_corpus_runs_its_pinned_count_of_finishes the_corpus_samples_every_node_kind_with_rows the_cost_curve_is_flat_where_the_bound_is_broken the_cost_curve_is_measured_at_both_ends the_die_evaluates_at_interval_and_brackets_the_oracle the_die_tool_documents_saved_text_is_pinned the_die_tool_is_one_union_and_still_cuts the_dies_union_is_the_chain_it_replaced the_dl3_witnesses_are_refused_by_the_closed_form_alone_at_dual64 the_door_proves_two_thirds_of_the_composed_dies_joints the_drive_answers_each_certified_leaf_through_the_assertion the_driver_and_the_report_are_schedule_independent the_f64_lane_is_the_second_pass_bit_for_bit the_f64_value_is_the_revalidated_form the_fin_group_is_one_placement_and_one_union the_first_boundary_stops_the_walk_and_a_seed_on_the_island_certifies_it the_fold_is_the_same_over_a_parallel_and_a_sequential_drive the_fold_is_the_same_under_both_driver_schedules the_fold_replays_a_leaf_on_the_drives_own_lane the_forms_the_plates_walks_build_are_pinned the_gather_opens_at_dual64_and_the_witness_set_is_pinned the_goldening_forms_are_schedule_free_and_the_human_form_is_not_one the_group_recomputes_exactly_when_its_inputs_move the_guided_dual_pass_evaluates_the_build_paths_nodes_over_the_corpus the_instrument_and_the_door_replay_in_one_session the_junction_where_the_wedges_meet_has_one_name_in_every_member_order the_ladder_rim_phase_already_reached_the_emitter_from_the_corpus the_load_doors_limit_is_the_deepest_body_a_save_writes the_log_is_a_function_of_the_member_order_alone the_margin_thin_accounting_is_goldened_bit_exact the_mc_tail_fraction_converges_on_the_accountings_tail the_migrated_world_is_the_pre_c_product the_one_axis_leaves_tile_the_seed the_pads_bore_finishes_alone_over_the_box_at_sym_interval the_pair_boolean_names_its_pieces_by_the_same_rule the_part_select_document_evaluates_at_sym_interval the_plain_memo_moves_no_decision the_plains_ledger_lines_are_the_same_under_every_dial_set the_polygon_close_is_written_once_in_shipped_src the_reachable_ladder_is_driven_through_its_doors the_receipt_identity_holds_on_every_edge_drive the_recorded_band_trim_counts_are_executable the_refused_chain_flattened_by_one_level_evaluates_at_the_bound the_registry_split_is_measured_at_a_pinned_point the_registry_wrapper_and_its_door_agree_on_every_corpus_document the_split_documents_evaluate_to_their_committed_digests the_strict_question_is_total_over_budgets_too the_sweep_documents_evaluate_to_their_committed_digests
// the_symmetric_plate_corpus_documents_seam_is_its_face_read_twice the_t_interval_encloses_the_exact_crossing_and_the_clamped_point the_ten_fin_heat_sink_is_bit_equal_superset_and_ordered the_tours_stop_two_assertion_reads_holds_where_the_caption_says_fails the_traced_wall_reads_the_radius_atom_at_sym the_two_hole_plate_stackup the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it the_union_node_folds_a_flush_partner_onto_the_edge_contact the_union_node_publishes_one_table_in_every_member_order the_union_records_the_constructed_tangency_proven_by_its_heading the_verdict_is_bit_identical_across_repeats_and_schedules the_vessels_cavity_certifies_embedded_at_both_scalars the_worked_example_names_one_table_in_every_member_order threaded_prior_never_contaminates_values_at_dual64 three_overlapping_strips_are_one_divider two_bars_that_sever_a_sunk_boss_wall_only_together_name_the_glue two_blocks_resting_on_one_floor_are_one_plane_at_their_bases two_blocks_that_open_a_sunk_boss_wall_only_together_name_the_glue two_copies_on_one_axis_glue_their_walls_on_one_axis two_half_bosses_and_a_bar_name_the_glue two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle two_pinches_build_one_body_in_every_member_order two_pins_are_two_rows_each_said_from_its_own_version two_press_fits_between_one_pair_are_not_quietable two_revolves_reading_one_axis_glue_unproven vocabulary_coverage_is_total wedges_meeting_at_a_vertex_build_one_body_in_every_member_order what_spans_above_the_top_is_not_cited
