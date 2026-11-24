---
id: thirty-six-sweep-slow-rows-were-last-run-before-the-loft-pass-finished
kind: issue
title: 36 sweep slow-set rows over lofts and sweeps were last run on a tree before PR 7292's final state
status: open
opened: 2026-11-23
priority: P2
cost: C
refs: [f64-built-data-is-built-at-t]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-23.

PR 7292 changed how a loft and a path sweep are built at an enclosure scalar, and how a v-degree-1 loft's slab walls are built at every scalar. These `sweep::all` slow-set rows sit in files that build a loft or a sweep. The first 33 passed locally on 2026-11-21, on the tree as it stood after the degree-1 change and before the row edits, the fixture, the stated-knots change and four merges of main (pinned corners and absent walls among them). The last three never finished: they were stopped after four hours on a loaded machine. The machine's build mutex did not come free within an hour at the end of the pass, and the PR gate runs none of them (it ran, and passed, the 37 other candidates, which live in files the PR edits).

Owed: one run on main after PR 7292 lands,

`cargo nextest run -p sweep --test all --ignore-default-filter --no-fail-fast -E '<the names below>'`

and a red row among them read against that PR first.

Passed on the earlier tree:

- `a_split_and_reminted_loft_holds_its_volume::split_lofts_hold_their_volumes_at_interval`
- `a_sweep_keeps_the_frame_it_is_handed::a_partial_revolve_on_a_tilted_plane_stores_its_frames_exactly`
- `a_sweep_keeps_the_frame_it_is_handed::an_arc_wall_whose_radial_turns_with_a_boxed_bulge_stores_every_members_direction`
- `clearance_reads_an_iso_bounded_wall::curved_walls_are_violated_just_above_their_separation`
- `common::crossed_cylinders::the_oracle_holds_its_closed_forms`
- `encl_curved_loft_shell::a_moved_curved_fitted_cap_builds_where_its_fit_certifies`
- `encl_curved_loft_shell::a_moved_fitted_cap_stands_its_corners_on_the_held_sides`
- `encl_curved_loft_shell::a_moved_plane_meets_a_fitted_cap_along_their_certified_section`
- `encl_curved_loft_shell::a_moved_planes_section_with_a_held_fit_follows_the_plane`
- `encl_curved_loft_shell::a_tilted_caps_marched_rim_refuses_at_its_certificate`
- `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit`
- `encl_curved_loft_shell::shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row`
- `encl_curved_loft_shell::the_curved_lofts_cap_moves_its_corners_along_the_slanted_seams`
- `encl_curved_loft_shell::the_shelled_lofts_wall_seams_derive_where_its_fits_certify`
- `m6_loft_body::certified::the_loft_body_certifies_and_encloses_nine_at_interval`
- `m6_loft_body::cut_loft_refuses_typed_naming_the_missing_boolean_layer`
- `m6_loft_body::shape_iii_volume_matches_the_derived_closed_form`
- `m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`
- `m8_4_intersection_iso::a_cap_rim_run_back_along_an_off_dyadic_row_certifies_as_the_row`
- `m8_4_intersection_iso::a_degree_two_widening_measures_against_the_oracle`
- `m8_4_intersection_iso::a_degree_two_widening_tessellates_against_the_oracle`
- `m8_4_intersection_iso::an_interior_column_intersection_mints_a_general_image`
- `m8_4_intersection_iso::the_operand_order_is_not_a_fact_about_the_locus`
- `split_edge_fitted_rows::a_split_carries_an_attached_fitted_row_to_each_child`
- `split_edge_fitted_rows::a_split_carries_an_attached_fitted_row_to_each_child_at_the_interval_scalar`
- `split_edge_fitted_rows::a_split_carries_the_general_row_to_each_child`
- `split_edge_fitted_rows::a_split_carries_the_general_row_to_each_child_at_the_interval_scalar`
- `split_edge_fitted_rows::the_mint_rederives_the_carried_rows_and_the_trimmed_lane_refuses_their_sub_range`
- `verbs_offc_consumer::an_approx_faced_body_moves_under_a_rigid_map`
- `verbs_offc_consumer::an_approx_faced_body_validates_at_tier_three`
- `verbs_offc_consumer::the_boolean_gate_refuses_an_approx_operand_by_kind`
- `verbs_offc_consumer::the_props_lanes_verdict_is_the_same_either_side_of_the_map`
- `verbs_offc_consumer::the_walls_a_placed_approx_capped_part_still_meets`

Never finished:

- `sign_walk_plus_v::an_undecided_sign_with_the_schedule_run_out_refuses`
- `spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong`
- `spline_subtract_intersect_census::every_op_of_a_spline_operand_refuses_typed_or_builds_right`
