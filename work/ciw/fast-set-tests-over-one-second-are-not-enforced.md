---
id: fast-set-tests-over-one-second-are-not-enforced
kind: issue
title: A fast-set test that takes 1 s or more fails no check
status: closed
opened: 2026-10-16
closed: 2026-10-16
priority: P3
cost: M
branch: ciw/fast-set-one-second-limit
pr: 5229
---



## Found

The discipline puts a test that costs too much in the slow set
(`.config/nextest.toml`'s `ci` profile `default-filter`), but nothing checked
it. One local pass of the workspace fast set (`cargo nextest run --profile ci
--workspace`, the `test` job's env, 4 vCPU, 2026-10-16 at 95c77e15) timed
14 198 passing tests at 1463 cpu-s:

| local time | tests |
|---|---|
| 0.3–0.5 s | 183 |
| 0.5–0.7 s | 84 |
| 0.7–1.0 s | 76 |
| ≥ 1.0 s | 222 |

**Scale.** The only hosted times within reach are the 5000-line tail of one
TIER=all `test` job (job 116211764790, 10-15): 2388 tests of its 1e-12 pass.
On the 68 tests both runs timed at ≥ 0.3 s locally, hosted/local has a median
of 0.96. The spread is wide (0.66 median over every test ≥ 0.05 s, p90 1.23,
one at 7.1), and the hosted pass ran at a different ε. So the bar for moving
was local ≥ 0.7 s, plus every test the hosted tail timed at ≥ 0.7 s.

## Fixed

- `scripts/check-fast-set-times.py` reads the default-ε step's junit report
  before `nextest-failures.py` deletes it. A passing fast-set test fails the
  step at ≥ 0.5 s when the diff adds it, and at ≥ 1.0 s otherwise. The diff's
  added names come from the `change filter` job's base, fetched at depth 1 in
  the `test` job. Each offender gets an `::error` (three, then a count), a
  step-summary row and a log-tail line naming the slow-set entry to add. It
  has a `--selftest` in the lint job.
- Nightly runs the `ci` profile with `--ignore-default-filter`, so it has no
  fast set to check.
- The 302 tests below moved to the slow set (222 of them at ≥ 1 s locally).
  The local fast set went from 1463 to 480 cpu-s.

## The margin after the move

- The slowest fast-set test now takes 0.695 s locally.
- 81 fast-set tests sit at 0.5–0.7 s locally. None sit at 0.7–1.0 s.
- In the hosted tail (1e-12, before the move), 3 of 2388 tests sat at
  0.7–1.0 s and 14 at 0.5–1.0 s.

## Moved

| local s | hosted 1e-12 s | binary | test |
|---|---|---|---|
| 71.82 |  | `editor-core::all` | `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` |
| 61.91 |  | `sweep::all` | `review_d2_adv_probes::d2_reached_variants` |
| 49.96 | 94.11 | `sweep::all` | `review_d2_adv_probes::d2_no_input_reaches_a_panic` |
| 38.18 |  | `geom-brep::all` | `review_4553_pxn_probes::the_decades_sweep_certifies_every_ruling_at_both_scalars` |
| 34.40 |  | `sweep::all` | `pis_cut_cavity::every_cut_through_a_bore_reads_its_truth_at_every_pose` |
| 29.05 |  | `sweep::all` | `a_pierce_held_in_a_curved_face::a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order` |
| 16.72 | 11.05 | `topo::all` | `contact7_touch_sweeps::a_rotated_comb_and_channel_sweep_clears_no_overlap` |
| 15.39 |  | `editor-core::all` | `emit_union_rim_piece_ranks::a_flush_union_publishes_one_table_in_every_member_order` |
| 13.24 |  | `sweep::all` | `a_pierce_held_in_a_curved_face::a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order` |
| 13.22 |  | `step-import::all` | `corpus_fold::the_folds_divergence_is_exactly_the_reported_promotion` |
| 12.75 |  | `geom-brep::all` | `ssi_limb3_one_arc::the_fold_answers_its_two_arcs_paired_as_the_locus_pairs_them` |
| 9.99 |  | `sweep::all` | `full_turn_bore_mate::intersect_and_differences_answer_the_closed_form` |
| 9.98 |  | `editor-core::all` | `e6_drive_over_a_loft_at_interval::a_point_polygon_loft_certifies_under_a_box_elsewhere` |
| 9.71 |  | `sweep::all` | `verbs_germarms2::every_pose_of_the_family_builds_at_its_closed_form` |
| 9.40 |  | `sweep::all` | `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order` |
| 9.20 |  | `geom-brep::all` | `m5_pr7_ssi::a_sliver_the_march_could_step_through_refuses_as_near_tangent` |
| 8.85 |  | `geom-brep::all` | `cert10r2_probes::probe1_quarter_cylinder_fold_norms_dominate` |
| 8.85 | 5.23 | `topo::all` | `m4_pr2_transform::the_m7_8_cubes_carriers_certify_at_rest_before_and_after_a_move` |
| 8.09 |  | `geom-brep::all` | `m5_pr7_ssi::the_uncertified_door_refines_until_limbs_1_and_2_pass` |
| 7.96 |  | `geom-brep::all` | `cert10r2_probes::probe1_signed_reading_encloses_random_rational_sweep` |
| 7.23 | 7.87 | `topo` | `review_m1_pr4::kill_ops_on_torn_bodies_panic_only_naming_a_premise` |
| 7.12 |  | `geom-brep::all` | `review_4553_pxn_probes::every_section_ruling_certifies_at_both_scalars` |
| 6.96 |  | `step-import::all` | `coherence_channel::the_import_corpora_are_quiet_and_the_only_lane_boundary_is_the_trimmed_face` |
| 6.92 | 4.85 | `topo::all` | `cleave_mint_doors::split_edge_splits_an_m7_8_edge_into_two_of_the_class_at_f64` |
| 6.78 | 3.95 | `topo::all` | `cleave_mint_doors::set_edge_curve_re_describes_every_m7_8_edge_at_f64` |
| 6.71 |  | `sweep::all` | `bool3_r1_probes::r1_oracle_sweep_prints_its_own_numbers` |
| 6.66 |  | `sweep::all` | `a_swept_cusp_is_legal_at_rest::a_loft_whose_sections_disagree_passes_with_the_seam_unjudged_by_kind` |
| 6.64 |  | `topo` | `cert_m3r1_probes::m3_a_corrupt_m7_8_wall_is_caught_at_every_door_whose_bound_names_the_right` |
| 6.30 |  | `editor-core::all` | `decide_3_split_rows_interval::decide_3_no_predicate_loses_a_decision` |
| 6.24 |  | `step-import::all` | `wall_column_structure::a_validated_wall_never_refuses_its_own_boundary_column` |
| 6.21 |  | `step-import::all` | `p1b_r1_import_scan::no_committed_fixture_imports_with_a_scaffold_at_rest` |
| 6.18 |  | `sweep::all` | `germ_torus_rods::random_rods_through_the_tube_never_answer_a_wrong_body` |
| 5.95 |  | `step-import::all` | `recognize_pins::own_corpus_promotions_are_exactly_the_enumerated_planar_walls` |
| 5.95 |  | `step-import::all` | `perf12_census_goldens::the_import_census_verdicts_are_goldened_bit_exact` |
| 5.73 |  | `geom-brep::all` | `cert10r2_probes::probe1_signed_reading_encloses_adversarial_extremes` |
| 5.44 |  | `editor-core::all` | `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` |
| 5.43 | 15.22 | `topo::all` | `m6_2_fitted_at_rest::a_rung3_edge_at_rest_carries_its_projected_rows_and_its_edge_the_tube` |
| 5.13 |  | `sweep::all` | `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself` |
| 5.07 |  | `editor-core::all` | `m4_pr6_roundtrip::save_load_replay_identity` |
| 5.06 |  | `geom::all` | `surfaces_stretch_sup::the_bound_covers_random_rational_nets` |
| 4.52 |  | `step-import::all` | `review_probes_m7_3::probe_all_unit_weight_rational_instance_imports_identically` |
| 4.35 |  | `sweep::all` | `bisector_nodes::a_rod_against_a_bore_builds_every_operation` |
| 4.31 |  | `editor-core::all` | `m5_pr8_bvh_diff::corpus_evaluations_bit_equal_realized_vs_idealized` |
| 4.22 | 19.82 | `topo::all` | `m6_2_fitted_at_rest::a_split_restricts_the_projected_row_to_each_child` |
| 4.12 | 2.93 | `topo::all` | `cleave_mint_doors::a_rechart_beside_the_wall_re_certifies_its_m7_8_edge_at_f64` |
| 4.03 |  | `geom-brep::all` | `m5_pr7_ssi::a_pair_bending_late_refuses_on_limb_3_without_refining_to_the_wall` |
| 4.00 |  | `editor-core::all` | `m4_pr8_corpus_interval::every_document_evaluates_green_at_interval` |
| 4.00 |  | `step-import::all` | `imported_loft_rim_offset::an_imported_lofts_cap_rims_are_derived_when_the_cap_moves` |
| 3.96 |  | `editor-core::all` | `m10_2_measure::a_cusp_loft_document_gathers_with_its_nurbs_seam_unjudged_by_kind` |
| 3.94 |  | `step-import::all` | `review_probes_m7_3::probe_reexport_promotion_divergence` |
| 3.83 |  | `geom-brep::all` | `offset_fit::an_offset_past_the_curvature_reach_refuses_at_the_collapse_meter` |
| 3.82 | 15.05 | `topo::all` | `m6_2_fitted_at_rest::certified::the_projected_rows_and_the_tube_certify_at_the_interval_scalar` |
| 3.80 |  | `geom-brep::all` | `offb_r1_probes::r1_extreme_d_near_the_collapse_bound_certifies_and_contains` |
| 3.61 |  | `editor-core::all` | `perf12_census_bvh_diff::corpus_rows_bit_equal_superset_and_ordered` |
| 3.59 |  | `editor-core::all` | `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval` |
| 3.55 | 3.13 | `topo::all` | `contact5_gate_and_beam::a_grid_of_brick_pairs_clears_exactly_the_rests` |
| 3.52 |  | `step-import::all` | `review_probes_m7_3::probe_iso_adoption_deterministic` |
| 3.44 |  | `step-import::all` | `recognize_pins::quasi_uniform_vocabulary_reads_the_same_surface` |
| 3.44 |  | `sweep::all` | `a_swept_cusp_is_legal_at_rest::a_cusp_loft_passes_with_its_nurbs_seam_unjudged_by_kind` |
| 3.41 |  | `topo` | `boolean::combine::recertify_lane_tests::graft_solid_certifies_the_lane_classes_through_the_scalars_lane` |
| 3.38 |  | `step-import::all` | `nurbs_import::an_adopted_iso_column_is_a_knot_domain_end` |
| 3.36 |  | `editor-core::all` | `pinned_lift_validates_once::the_lifted_form_is_the_revalidated_form_at_dual` |
| 3.29 |  | `topo` | `boolean::circle_torus::tests::no_wrong_certified_answer_across_circle_radii` |
| 3.18 |  | `editor-core::all` | `r2_m10_di_probes::assemble_census_door_matches_f64_at_dual64` |
| 3.14 |  | `editor-core::all` | `m10_p_lift::the_lift_at_f64_reproduces_the_pinned_evaluation` |
| 3.12 |  | `editor-core::all` | `docm3_union::removing_any_pip_leaves_both_die_fillets_resolving` |
| 3.07 |  | `sweep::all` | `reach_cone_split::an_upright_cone_splits_at_every_pose_through_its_apex_faces` |
| 3.01 |  | `topo` | `cert_m3r1_probes::m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else` |
| 3.00 |  | `editor-core::all` | `m10_di_dual_corpus::the_gather_opens_at_dual64_and_the_witness_set_is_pinned` |
| 2.98 |  | `geom::all` | `surfaces_stretch_floor::the_per_axis_floors_reach_past_the_area_ceiling` |
| 2.98 |  | `editor-core::all` | `r1_dual_probes::r1_is_dl3s_measured_problem_reproducible` |
| 2.97 | 8.00 | `topo::all` | `m5_pr7_split_meter::the_end_to_end_nurbs_split_row` |
| 2.94 |  | `geom-brep::all` | `offb_r2_probes::p3_offset_just_inside_the_certified_reach_still_bounds_the_sample` |
| 2.81 |  | `editor-core::all` | `pinned_lift_validates_once::the_lifted_form_is_the_revalidated_form_at_f64` |
| 2.79 |  | `sweep::all` | `bisector_nodes::equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms` |
| 2.77 |  | `editor-core::all` | `intent_s2_c_world::the_migrated_world_is_the_pre_c_product` |
| 2.76 |  | `editor-core::all` | `r1_dual_probes::r1_dual_product_matches_f64_including_carriers` |
| 2.68 |  | `editor-core::all` | `m10_di_dual_corpus::every_document_evaluates_at_dual64_with_the_f64_value_channel` |
| 2.67 |  | `editor-core::all` | `m4_pr8_corpus::incremental_recompute_reuses_the_cone_complement` |
| 2.66 |  | `step-export::all` | `m5_pr13_curved::curved_exports_are_byte_deterministic` |
| 2.65 |  | `editor-core::all` | `r2_m10_di_probes::threaded_prior_never_contaminates_values_at_dual64` |
| 2.60 |  | `geom-brep::all` | `m5_pr7_ssi::a_tiny_net_the_plane_meets_traces_or_refuses_by_the_kind_its_size_earns` |
| 2.53 |  | `editor-core::all` | `r1_dual_probes::r1_dual_value_channel_matches_f64_including_carriers` |
| 2.46 | 16.96 | `topo::all` | `m6_2_fitted_at_rest::a_producers_closing_mint_re_derives_the_projected_row` |
| 2.45 |  | `editor-core::all` | `m10_4_seed::an_unseeded_evaluation_is_the_default_options_evaluation` |
| 2.44 |  | `editor-core::all` | `r2_m10_di_probes::deep_value_channel_identity_f64_vs_dual64_including_carrier_arenas` |
| 2.40 |  | `editor-core::all` | `union_pinch_member_order::a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order` |
| 2.39 |  | `editor-core::all` | `docm5_subject::the_assembly_wrapper_and_its_door_agree_on_every_corpus_document` |
| 2.37 |  | `editor-core::all` | `m10_4_r2_probes_interval::the_guided_dual_pass_evaluates_the_build_paths_nodes_over_the_corpus` |
| 2.35 |  | `geom-brep::all` | `cert10_r1_probes::cert10r1_the_signed_reading_encloses_the_true_partials` |
| 2.32 | 2.03 | `topo::all` | `contact7_touch_sweeps::a_rotated_bracket_and_brick_sweep_clears_no_overlap` |
| 2.29 |  | `topo` | `euler::tests::a_fan_mev_rebases_the_plane_x_nurbs_class_through_the_policy_lane_and_mev_null_splits_it` |
| 2.24 |  | `editor-core::all` | `emit_union_rim_piece_ranks::a_covered_contact_decides_alike_in_every_order` |
| 2.23 |  | `step-export::all` | `kernel_sidecars::kernel_sidecar_fields_match_live_kernel` |
| 2.17 |  | `mesh::all` | `review_m2_pr6_errors::survives_delta_fine_but_sane_still_tessellates` |
| 2.02 | 2.54 | `viewer::all` | `display_budget::a_budget_delta_does_not_depend_on_the_delta_asked_for` |
| 1.98 |  | `sweep::all` | `full_turn_bore_mate::a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth` |
| 1.97 |  | `geom-brep::all` | `r1_pxn_probes::the_certified_sup_bounds_the_dense_sampled_true_sup` |
| 1.96 |  | `geom-core::all` | `m10_10_r2_probes::r2_rule_d_agrees_with_a_by_hand_closed_form_across_the_accepted_q` |
| 1.96 |  | `step-import::all` | `review_probes_m7_3::probe_subunit_x_direction_rim_frame_rigidity` |
| 1.94 |  | `sweep::all` | `m5_s10_face_sense::a_convex_arc_c_shape_extrude_and_loft_caps_point_out_and_certify` |
| 1.94 |  | `geom-brep::all` | `props_cone_incidence::between_samples::no_ellipse_answering_incidence_lies_past_zero` |
| 1.94 | 2.12 | `topo::all` | `spline_reanchor_rows::an_inward_cap_offset_re_anchors_the_walls_spline_edges` |
| 1.93 |  | `step-import::all` | `nurbs_import::loft_prism_walls_get_distinct_surface_keys` |
| 1.91 |  | `editor-core::all` | `perf2_name_keying_differential::every_corpus_documents_name_tables_and_persisted_text_are_pinned` |
| 1.90 | 11.02 | `topo::all` | `review_m6_2_probes::a_between_samples_net_corruption_is_refused_at_the_envelope` |
| 1.90 |  | `sweep::all` | `offb_r1_loft_probes::r1_skinned_loft_offset_certificate_contains_dense_sample` |
| 1.89 |  | `editor-core::all` | `m10_1_r2_probes::a_distribution_changes_no_content_key_at_interval` |
| 1.86 |  | `step-import::all` | `review_probes_m7_3::probe_descriptions_native_vs_imported_bitwise` |
| 1.84 |  | `step-import::all` | `nurbs_import::loft_prism_descriptions_land_in_the_native_classes` |
| 1.82 |  | `editor-core::all` | `m5_pr8_bvh_diff::corpus_boolean_operands_superset_pin` |
| 1.81 | 10.26 | `topo::all` | `review_m6_2_probes::a_sub_interval_row_is_caught_by_its_interval` |
| 1.81 | 12.87 | `topo::all` | `review_m6_2_probes::the_foreign_arc_row_fails_on_the_carriers_structure` |
| 1.77 |  | `editor-core::all` | `m10_di_dual_corpus::a_bumped_parameter_at_dual64_recomputes_exactly_its_cone` |
| 1.77 |  | `editor-core::all` | `r2_m10_di_probes::deep_value_channel_identity_interval_vs_dual_interval` |
| 1.74 |  | `editor-core::all` | `docm1_face_frame::face_frame_sense_matches_the_stored_flag_on_every_corpus_face` |
| 1.74 |  | `editor-core::all` | `blend5_r2_probes::the_corpus_band_trim_census_is_the_recorded_one` |
| 1.74 |  | `sweep::all` | `m5_pr12_die::deviation_1_both_doors_compose_and_agree` |
| 1.72 |  | `sweep::all` | `m5_s11_concave_sense::a_lofted_operand_refuses_the_union_check_typed` |
| 1.72 |  | `editor-core::all` | `blend5_r1_probes::the_recorded_band_trim_counts_are_executable` |
| 1.71 |  | `sweep::all` | `m6_surgery::the_composed_die_replays_bit_identically` |
| 1.71 |  | `editor-core::all` | `m4_pr8_corpus::exact_mass_pins_hold` |
| 1.70 |  | `editor-core::all` | `m10_sym_profile_interval::the_plains_ledger_lines_are_the_same_under_every_dial_set` |
| 1.69 |  | `editor-core::all` | `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven` |
| 1.68 |  | `editor-core::all` | `edit_step_segments::every_corpus_step_is_answered_and_the_answers_partition_the_loop` |
| 1.67 |  | `geom-brep::all` | `ssi_limb3_one_arc::a_side_whose_hull_is_loose_reads_clear_at_both_doors` |
| 1.67 |  | `editor-core::all` | `m10_6_r1_probes_interval::report_key_tells_two_budgets_apart` |
| 1.66 |  | `editor-core::all` | `lib_g16_corpus_name_digests::every_corpus_documents_name_tables_are_golden` |
| 1.65 |  | `mesh` | `nurbs_cert::tests::r1_tiny_and_offset_patches_dominated` |
| 1.62 |  | `geom-brep::all` | `m5_pr7_ssi::a_flat_wall_whose_chart_path_bends_answers_as_the_plane_it_is` |
| 1.62 |  | `editor-core::all` | `names_verbatim_edge_evaluator::the_corpus_samples_every_node_kind_with_rows` |
| 1.62 |  | `editor-core::all` | `m10_di_dual_corpus::dual_interval_evaluates_with_the_interval_value_channel` |
| 1.62 |  | `step-import::all` | `review_k3_probe::k6_loft_prism_imports_the_nurbs_frontier_retired` |
| 1.61 |  | `geom-core` | `sym::tests::a_decision_that_closes_only_with_rule_a_off` |
| 1.61 |  | `editor-core::all` | `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_f64` |
| 1.61 |  | `geom-brep` | `ssi::certify::tests::the_chart_tube_records_its_per_axis_pad_and_accounting_banks_its_windows` |
| 1.61 |  | `sweep::all` | `full_turn_bore_mate::a_shaft_off_the_bores_seam_unions` |
| 1.60 |  | `geom-brep::all` | `offset_fit::non_analytic_base_fits_and_the_bound_contains_the_sample` |
| 1.58 |  | `editor-core::all` | `edit_step_segments::every_corpus_step_is_answered_against_its_own_authored_geometry` |
| 1.55 |  | `sweep::all` | `full_turn_bore_mate::a_shaft_a_quarter_turn_off_the_bores_seam_unions` |
| 1.55 |  | `sweep::all` | `operand_gate_pose::a_bar_through_the_cone_wall_refuses_at_every_pose` |
| 1.54 |  | `editor-core::all` | `m4_pr8_corpus::every_document_evaluates_green` |
| 1.52 |  | `geom-brep::all` | `cert5_r1_patch_probes::knots_one_ulp_apart_stay_sound` |
| 1.52 |  | `sweep::all` | `verbs_offc_consumer::an_approx_faced_body_validates_at_tier_three` |
| 1.51 |  | `sweep::all` | `a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge` |
| 1.50 |  | `editor-core::all` | `onb_wall_normal_census::no_corpus_face_sits_on_the_frame_seam` |
| 1.49 |  | `editor-core::all` | `lib_u5_interrogate::the_reachable_ladder_is_driven_through_its_doors` |
| 1.49 |  | `editor-core::all` | `docm8_flat_merged::no_corpus_document_publishes_a_nested_merged_row` |
| 1.49 |  | `editor-core::all` | `edit_step_segments::every_attached_radius_was_keyed_first` |
| 1.48 |  | `sweep::all` | `pis_arc_capped_poses::every_tilted_cut_wall_reads_its_truth` |
| 1.47 |  | `geom-brep::all` | `ssi_limb3_one_arc::a_carrier_along_a_side_whose_locus_wanders_past_eps_refuses_at_rest` |
| 1.47 |  | `editor-core::all` | `names_verbatim_edge_evaluator::every_node_publishes_the_heads_its_verbatim_edge_requires` |
| 1.45 |  | `topo` | `boolean::reduce::line_cone_rows::roots_beside_the_apex_are_placed_along_the_edge` |
| 1.44 |  | `geom-brep` | `pcurve_cache::envelope_lemma_fuzz::every_arm_and_term_is_load_bearing_in_a_pinned_sweep` |
| 1.44 |  | `editor-core::all` | `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` |
| 1.43 |  | `editor-core::all` | `edit_placement_corpus_bits::every_corpus_transform_places_its_body_by_the_pinned_bits` |
| 1.43 |  | `geom-brep::all` | `cert5_r1_patch_probes::knots_hugging_the_trim_edges_stay_sound` |
| 1.43 |  | `step-export::all` | `review_k4_probe::k4_sidecar_volume_literals_round_trip_bit_exact` |
| 1.42 |  | `editor-core::all` | `emit_seam_junction::a_crossing_of_a_merged_rim_is_named_the_same_in_every_order_that_fuses` |
| 1.41 |  | `editor-core::all` | `union_pinch_member_order::two_pinches_build_one_body_in_every_member_order` |
| 1.40 |  | `sweep::all` | `verbs_offc_consumer::an_approx_faced_body_moves_under_a_rigid_map` |
| 1.40 |  | `step-export::all` | `m5_pr13_curved::the_export_corpus_obeys_the_exactness_frame_sense_and_nurbs_laws` |
| 1.39 |  | `editor-core::all` | `seat8_split_lowering::the_corpus_has_no_split_with_an_empty_side` |
| 1.39 |  | `step-export::all` | `onb_wall_normal_census::no_fixture_face_sits_on_the_frame_seam` |
| 1.39 |  | `sweep::all` | `run_walls_built::extruded_runs_build_one_wall_each` |
| 1.36 |  | `editor-core::all` | `a_mate_on_a_full_turn_bore_names::a_mate_on_a_full_turn_bore_names` |
| 1.36 |  | `geom-brep::all` | `cert5_arm_and_cells::a_genuine_c0_jump_stays_contained` |
| 1.35 |  | `geom-brep::all` | `review_r1_rational_probes::probe_c0_kink_area` |
| 1.35 |  | `mesh` | `nurbs_cert::tests::first_derivative_sups_dominate_samples_and_refine_upward` |
| 1.35 |  | `sweep::all` | `full_turn_bore_mate::a_shaft_on_the_bores_seam_unions` |
| 1.35 | 7.21 | `topo::all` | `m6_2_fitted_at_rest::the_dual_holds_neither_lane_and_each_refusal_names_its_scalar` |
| 1.34 | 1.32 | `viewer::all` | `index_memo::the_worker_threads_memo_answers_across_landings_and_a_skipped_generation` |
| 1.33 |  | `test-utils::all` | `reader_census::every_site_that_reads_rust_source_is_in_the_ledger` |
| 1.33 |  | `editor-core::all` | `docm3_union::the_dies_union_is_the_chain_it_replaced` |
| 1.32 |  | `pncad-py` | `prose_census::tests::the_census_reads_a_tree_and_not_an_empty_one` |
| 1.32 |  | `geom-brep::all` | `offset_fit::cylinder_fit_matches_the_closed_form_both_signs` |
| 1.32 |  | `geom-brep::all` | `cert5_r1_patch_probes::genuine_c0_jump_is_contained` |
| 1.32 |  | `step-export::all` | `export::committed_fixtures_are_byte_golden` |
| 1.30 |  | `sweep::all` | `bisector_nodes::two_crossing_bores_build_every_operation_at_the_closed_forms` |
| 1.25 |  | `sweep::all` | `band_reads_three_ways::the_three_way_reading_replays_at_interval` |
| 1.25 |  | `geom::all` | `n3r1_probes::n3r1_f64_adversarial_corpus_is_contained_at_zero_pad` |
| 1.25 |  | `geom-brep::all` | `review_r1_rational_probes::probe_moebius_quarter_cylinder` |
| 1.25 |  | `geom-brep::all` | `review_r1_rational_probes::probe_extreme_weight_hypar` |
| 1.25 | 1.14 | `viewer::all` | `focus_highlight::clicking_a_die_face_reaches_the_feature_that_made_it` |
| 1.24 |  | `geom-brep::all` | `cert7_r1_probes::a_hostile_weight_spread_never_yields_a_finite_wrong_bound` |
| 1.24 |  | `editor-core::all` | `msolve8_levered_clash::c2_inverted_coset_never_refuses` |
| 1.22 |  | `topo` | `boolean::r2_probes::r2_lattice_certified_counts_agree_with_the_independent_counter` |
| 1.18 |  | `pncad-py` | `prose_census::tests::no_display_impl_renders_a_brace_shaped_payload_through_debug` |
| 1.18 |  | `mesh` | `nurbs_cert::tests::cell_grid_cert_is_the_covered_cells_componentwise_sup` |
| 1.18 |  | `pncad-py` | `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason` |
| 1.17 |  | `profile::all` | `raw_door_census::no_production_source_writes_a_vertex_table` |
| 1.17 |  | `geom::all` | `surfaces_stretch_floor::the_floors_hold_on_curved_rational_nets` |
| 1.17 |  | `editor-core::all` | `m10_7_r2_probes_interval::r2_a_zero_term_budget_reproduces_the_tier_off_verdict` |
| 1.17 | 0.95 | `viewer::all` | `focus_highlight::each_die_face_is_marked_by_the_feature_that_made_it` |
| 1.16 |  | `profile::all` | `raw_door_census::every_crate_that_names_the_door_reaches_it` |
| 1.16 |  | `editor-core::all` | `emit_union_flush_names::a_boss_on_one_piece_of_a_covered_face_is_cited_in_no_order` |
| 1.15 |  | `editor-core` | `names::emit_topo::touch_reread_rows::a_vertex_read_twice_names_its_result_in_every_op` |
| 1.15 |  | `sweep::all` | `reach_wall_chord_rows::a_diagonal_bar_sunk_into_a_cylinder_cap_answers_its_closed_form` |
| 1.13 |  | `geom-brep::all` | `offset_fit::the_collapse_meter_brackets_the_sphere_s_known_curvature` |
| 1.12 | 1.26 | `topo::all` | `spline_reanchor_rows::an_outward_cap_offset_runs_past_the_spline_carriers_end` |
| 1.11 | 0.56 | `topo::all` | `certified_enclosure_impl_census::every_certifying_scalar_has_a_wiring_row_and_every_row_a_scalar` |
| 1.11 |  | `step-import::all` | `review_probes_m7_3::probe_arm_b_rim_off_wall_arc` |
| 1.11 | 1.04 | `topo` | `seqgen::random_op_sequences::random_op_sequences_hold_all_properties` |
| 1.09 |  | `editor-core::all` | `m10_di_dual_corpus::same_seed_replay_at_dual64_reuses_everything` |
| 1.09 |  | `editor-core::all` | `review_m4_pr2::canceled_partials_are_typed_and_never_pollute_the_memo` |
| 1.08 |  | `mesh` | `nurbs_cert::tests::cert10_whole_face_bound_is_the_per_cell_fold` |
| 1.08 |  | `sweep::all` | `reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order` |
| 1.08 |  | `sweep::all` | `m6_surgery::the_surgery_front_door_refuses_its_named_gaps` |
| 1.05 |  | `sweep::all` | `split_edge_fitted_rows::a_split_carries_the_general_row_to_each_child` |
| 1.05 |  | `geom-brep::all` | `review_r1_rational_probes::probe_extreme_weight_square` |
| 1.04 |  | `editor-core::all` | `e6_drive_over_a_loft_at_interval::a_rotated_frame_loft_bisects_to_its_budget_under_a_box_elsewhere` |
| 1.04 |  | `mesh::all` | `probe_review::z3_fine_nurbs_vs_coarse_planar_neighbor_watertight` |
| 1.03 | 1.15 | `viewer::all` | `focus_highlight::a_node_that_made_nothing_marks_what_passed_through_it` |
| 1.03 |  | `sweep::all` | `a_ruling_through_a_box_corner_edge::a_wall_through_a_box_edge_records_the_rest_on_the_piece_that_holds_it` |
| 1.02 |  | `geom-brep::all` | `offb_r1_probes::r1_an_extreme_weight_rational_base_refuses_or_contains` |
| 1.02 |  | `mesh` | `nurbs_cert::tests::cert10_rational_cell_reading_is_the_signed_hull` |
| 1.02 |  | `sweep::all` | `pis_arc_capped_poses::every_arc_capped_kind_reads_its_truth_at_every_pose` |
| 1.02 |  | `editor-core::all` | `m4_pr7_appearance::attribute_survives_no_flip_parameter_motion_on_the_die` |
| 1.01 |  | `geom-brep::all` | `m5_pr7_ssi::a_wall_turning_back_short_of_a_plane_in_band_answers_empty` |
| 1.01 |  | `editor-core::all` | `docm8_flat_merged::a_four_member_chain_fuses_in_every_order` |
| 1.01 |  | `sweep::all` | `rest_zip_admission::the_reflex_site_off_the_pins_poses_never_ships_the_overlap_twice` |
| 1.01 |  | `sweep::all` | `run_walls_built::revolved_runs_build_one_wall_each` |
| 0.99 |  | `geom-brep::all` | `m5_pr7_ssi::an_offset_inside_the_band_answers_by_its_distance_never_by_its_sign` |
| 0.99 |  | `geom-brep::all` | `cert7_r2_probes::r2_hostile_alternating_weights_contain_or_refuse` |
| 0.98 | 1.02 | `topo::all` | `a_vertex_read_by_two_sector_passes::a_touching_vertex_paired_on_the_face_builds_sound_in_every_op` |
| 0.98 |  | `geom-brep` | `ssi::enclose::tests::a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube` |
| 0.98 |  | `test-utils::all` | `hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration` |
| 0.97 |  | `editor-core::all` | `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere` |
| 0.97 |  | `test-utils::all` | `hand_written_impl_census::every_known_hand_listed_impl_is_still_found` |
| 0.96 |  | `geom-brep::all` | `offb_r2_probes::p4_certify_offset_on_a_rational_fit` |
| 0.95 |  | `geom-brep` | `pcurve_cache::envelope_lemma_fuzz::projected_searches::the_envelope_dominates_a_certified_lower_bound_at_interval` |
| 0.93 |  | `geom-brep::all` | `analytic_rung3_certificate::a_dense_interpolant_of_a_section_certifies_its_tube` |
| 0.92 |  | `editor-core::all` | `m4_pr2_eval_interval::the_die_evaluates_at_interval_and_brackets_the_oracle` |
| 0.92 |  | `sweep::all` | `mate7a_r1_probes::p1_wall1_passes_the_gate_and_the_crossing_layer_and_stops_at_the_join` |
| 0.92 |  | `geom::all` | `curves_speed_upper_bound::the_bound_covers_random_rational_nets` |
| 0.91 |  | `test-utils::all` | `hand_written_impl_census::the_walk_still_sees_every_file_holding_one` |
| 0.91 |  | `step-import::all` | `spiric_roundtrip::a_spiric_rim_exports_as_a_spline_and_its_reimport_door_is_pinned` |
| 0.90 |  | `sweep::all` | `review_4704_flush_round_oracle::the_rims_half_turn_postures_answer_against_the_oracle` |
| 0.90 |  | `sweep::all` | `offc_r1_probes::a_boolean_against_the_twisted_approx_body_refuses_typed` |
| 0.90 |  | `geom-brep::all` | `m5_pr7_ssi::a_line_a_tenth_off_a_sides_direction_traces_however_near_the_band` |
| 0.89 |  | `editor-core::all` | `seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests` |
| 0.89 |  | `mesh` | `nurbs_cert::tests::r1_near_zero_weight_dominated` |
| 0.89 |  | `geom-brep::all` | `m5_pr7_ssi::a_plane_clipping_a_walls_corner_traces_the_clip_however_short` |
| 0.88 | 0.57 | `topo::all` | `certified_enclosure_impl_census::every_door_value_has_a_roster_and_every_roster_a_door` |
| 0.87 |  | `step-import::all` | `review_probes_m7_3::probe_refit_seam_refuses_typed` |
| 0.87 |  | `sweep::all` | `rest_zip_admission::the_tangent_site_keeps_building_pure_contacts` |
| 0.87 |  | `geom-brep::all` | `offb_r1_probes::r1_a_fit_for_the_wrong_sign_refuses` |
| 0.86 |  | `mesh` | `nurbs_cert::tests::every_cell_bound_dominates_its_own_sampled_hessian` |
| 0.86 |  | `geom-brep` | `offset_fit::tests::the_vector_upper_bounds_do_not_move_under_a_rotation` |
| 0.86 |  | `editor-core::all` | `m10_3_driver_interval::the_certification_width_is_no_longer_bounded_by_epsilon` |
| 0.85 |  | `geom-brep::all` | `ssi_limb3_one_arc::a_flush_side_on_a_conical_or_twisted_wall_certifies` |
| 0.85 |  | `geom-core::all` | `certified_endpoint_census::the_door_importers_are_the_gate_s_allowlist` |
| 0.85 |  | `pncad-py` | `prose_census::tests::no_raise_site_composes_a_debug_rendering_into_its_message` |
| 0.84 |  | `sweep::all` | `bisector_lane_stubs::a_right_angle_stub_builds_every_operation_at_its_closed_form` |
| 0.84 |  | `sweep::all` | `m6_surgery::the_composed_die_certifies_and_tessellates_watertight` |
| 0.84 |  | `mesh::all` | `r1_probes_issue1362::r1_hunts_a_near_origin_input_where_the_anchor_flips_a_column` |
| 0.84 |  | `geom-brep` | `patch_bound::tests::the_area_floor_reads_each_cells_own_normal` |
| 0.83 |  | `editor-core::all` | `m10_10_pins_interval::m10_10_the_plate_needs_no_door` |
| 0.82 |  | `geom-core::all` | `m10_10_r2_probes::r2_rule_d_with_x_a_form_carrying_atoms` |
| 0.82 |  | `mesh` | `nurbs_cert::tests::no_cell_exceeds_the_whole_patch_bound` |
| 0.82 |  | `sweep::all` | `join1_r2_probes::r2_probes` |
| 0.82 |  | `sweep::all` | `blend_per_shell_carry::the_sealed_island_request_builds_at_every_pose` |
| 0.82 |  | `editor-core::all` | `docm9_range::the_one_axis_leaves_tile_the_seed` |
| 0.81 |  | `sweep::all` | `encl_curved_loft_shell::the_curved_lofts_cap_moves_its_corners_along_the_slanted_seams` |
| 0.80 |  | `topo` | `boolean::sectors::tests::a_conic_tie_verdict_is_true_of_the_sampled_arc` |
| 0.80 |  | `geom-brep::all` | `offset_fit::a_degraded_fit_fails_the_certificate_and_names_the_limb` |
| 0.80 |  | `geom::all` | `curves_review_m5_pr4_adversarial::f9_e2e_cylinder_fit_project_certify_decide` |
| 0.80 |  | `geom-core::all` | `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` |
| 0.79 |  | `sweep::all` | `verbs_germarms::a_bar_across_a_bore_builds` |
| 0.79 |  | `geom-brep::all` | `cert7_r2_probes::r2_ellipse_of_revolution_wall_through_the_storage_door` |
| 0.79 |  | `mesh` | `nurbs_cert::tests::r1_extreme_weights_dominated` |
| 0.79 |  | `editor-core::all` | `m10_5_r1_probes_interval::the_bvh_prune_decides_inside_the_band_where_the_funnel_would_not` |
| 0.78 |  | `geom-brep::all` | `cert10r2_probes::probe1_quarter_cylinder_signed_cells_enclose_densely` |
| 0.77 |  | `sweep::all` | `m8_4_intersection_iso::an_interior_column_intersection_mints_a_general_image` |
| 0.76 |  | `editor-core::all` | `docm5_subject::the_registry_split_is_measured_at_a_pinned_point` |
| 0.75 |  | `editor-core::all` | `m4_pr3_names_ci::pip_depth_motion_without_flips_leaves_every_table_identical` |
| 0.74 |  | `editor-core::all` | `docm8_flat_merged::a_chain_with_a_disjoint_member_fuses_in_every_order` |
| 0.73 |  | `step-export::all` | `m6_6_sense_gate::die_pips_dimple_flip_refuses` |
| 0.73 | 0.37 | `topo::all` | `contact7_touch_sweeps::a_crossed_ridge_sweep_clears_no_overlap` |
| 0.73 |  | `geom-brep::all` | `approx_surface::a_rational_fit_is_certified_as_the_surface_it_is` |
| 0.73 |  | `geom-brep::all` | `offb_r2_probes::p2_skinned_loft_certificate_contains_a_dense_sample_both_signs` |
| 0.72 |  | `sweep::all` | `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar` |
| 0.72 |  | `sweep::all` | `blend_band_reach_oracle::a_convex_block_meets_a_ball_void_where_the_oracle_says` |
| 0.72 |  | `sweep::all` | `review_pr12_probes::probe_i_door_a_full_tool` |
| 0.72 |  | `topo` | `face_normal::tests::no_source_file_folds_the_bit_by_hand` |
| 0.72 |  | `geom-brep::all` | `m5_pr7_ssi::a_side_the_plane_crosses_inside_the_band_ends_its_arc_at_its_own_root` |
| 0.72 |  | `editor-core::all` | `parallel_node_map_interval::a_session_decides_and_counts_as_the_serial_walk_at_one_and_four_threads` |
| 0.72 |  | `sweep::all` | `m6_surgery::the_shrunk_faces_keep_their_rings_and_senses` |
| 0.71 | 0.30 | `topo` | `splitting::spiric_arc::tests::the_row_reads_the_arcs_region_at_interval` |
| 0.71 |  | `sweep::all` | `run_walls_built::fully_revolved_arc_runs_build_one_wall_each` |
| 0.71 |  | `geom-brep::all` | `offset_fit::a_fit_for_the_wrong_distance_is_refused_by_the_certifying_limb` |
| 0.71 |  | `sweep::all` | `m6_surgery::ring_clearance_trio_definite_pass_definite_refuse_in_band_escalate` |
| 0.71 |  | `editor-core::all` | `m4_pr2_eval::doc_param_edit_recomputes_the_param_cone` |
| 0.71 |  | `geom-brep` | `props::quad::tests::q4_the_planted_envelope_covers_the_displacement_it_states` |
| 0.70 |  | `editor-core::all` | `m10_5_clearance_interval::the_cost_curve_is_measured_at_both_ends` |
| 0.70 |  | `sweep::all` | `s393_start_frame_door::an_identity_start_placement_is_not_the_normal_plane_of_the_s_path` |
| 0.70 |  | `editor-core::all` | `m10_5_r2_probes_interval::every_configuration_lands_in_one_arm_with_a_holding_receipt` |
| 0.70 |  | `sweep::all` | `snowman::a_ball_inside_a_two_sphere_body_builds` |
| 0.70 | 0.70 | `viewer::all` | `story_parametric::the_parametric_living_walk` |
| 0.62 | 0.82 | `topo::all` | `review_m3_pr55::d_die_21_pips_exact` |
| 0.58 | 3.94 | `topo::all` | `m5_pr7_split_meter::a_split_at_the_endpoint_band_escalates_or_refuses_in_metres` |
| 0.10 | 2.36 | `topo::all` | `a_seamless_pipe_cut_by_an_oblique_plane_measures::a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval` |
