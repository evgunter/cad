//! Aggregated integration-test binary for `sweep`.
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
// `tests/common/mod.rs`, `tests/mate2_common/mod.rs`, `tests/revolve_common/mod.rs` — and every consumer
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
mod mate2_common;
mod revolve_common;
mod seam_region_common;

#[path = "a_blend_leaves_a_smooth_edge_as_it_is.rs"]
mod a_blend_leaves_a_smooth_edge_as_it_is;
#[path = "a_blend_reads_the_wedge_through_the_one_fold.rs"]
mod a_blend_reads_the_wedge_through_the_one_fold;
#[path = "a_carried_record_no_reading_covers.rs"]
mod a_carried_record_no_reading_covers;
#[path = "a_circle_of_one_radius_elsewhere_is_no_rim.rs"]
mod a_circle_of_one_radius_elsewhere_is_no_rim;
#[path = "a_closed_rider_lying_in_a_face_refuses_its_slit_form.rs"]
mod a_closed_rider_lying_in_a_face_refuses_its_slit_form;
#[path = "a_conical_ended_football_revolves.rs"]
mod a_conical_ended_football_revolves;
#[path = "a_corner_cut_on_a_far_wall_reads_its_sections_width.rs"]
mod a_corner_cut_on_a_far_wall_reads_its_sections_width;
#[path = "a_curve_off_its_face_at_a_build_is_the_kernels.rs"]
mod a_curve_off_its_face_at_a_build_is_the_kernels;
#[path = "a_curved_keel_in_the_plane.rs"]
mod a_curved_keel_in_the_plane;
#[path = "a_curved_keel_on_the_floor_refuses_its_slit_form.rs"]
mod a_curved_keel_on_the_floor_refuses_its_slit_form;
#[path = "a_cut_by_a_curved_seat_refuses_its_material.rs"]
mod a_cut_by_a_curved_seat_refuses_its_material;
#[path = "a_cylinder_split_just_off_its_rim_vertex.rs"]
mod a_cylinder_split_just_off_its_rim_vertex;
#[path = "a_face_across_a_wavy_corner.rs"]
mod a_face_across_a_wavy_corner;
#[path = "a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest.rs"]
mod a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest;
#[path = "a_face_off_the_tangent_ruling.rs"]
mod a_face_off_the_tangent_ruling;
#[path = "a_face_reads_its_chart_image.rs"]
mod a_face_reads_its_chart_image;
#[path = "a_flank_touch_at_a_shared_corner.rs"]
mod a_flank_touch_at_a_shared_corner;
#[path = "a_flat_frustum_revolves.rs"]
mod a_flat_frustum_revolves;
#[path = "a_flipped_shallow_dimple.rs"]
mod a_flipped_shallow_dimple;
#[path = "a_groove_at_a_shafts_root_continues_its_wall.rs"]
mod a_groove_at_a_shafts_root_continues_its_wall;
#[path = "a_keel_on_a_bores_ruling.rs"]
mod a_keel_on_a_bores_ruling;
#[path = "a_keel_ring_in_an_elliptic_section.rs"]
mod a_keel_ring_in_an_elliptic_section;
#[path = "a_keel_tree_in_the_plane.rs"]
mod a_keel_tree_in_the_plane;
#[path = "a_loft_computes_its_walls_in_t.rs"]
mod a_loft_computes_its_walls_in_t;
#[path = "a_lofted_lens_blade_certifies_its_sign.rs"]
mod a_lofted_lens_blade_certifies_its_sign;
#[path = "a_lofts_chart_ends_read_as_placed.rs"]
mod a_lofts_chart_ends_read_as_placed;
#[path = "a_long_bound_beside_a_short_arm.rs"]
mod a_long_bound_beside_a_short_arm;
#[path = "a_minted_vertex_on_a_curved_face.rs"]
mod a_minted_vertex_on_a_curved_face;
#[path = "a_mirrored_loft_placement_is_no_sketch_plane.rs"]
mod a_mirrored_loft_placement_is_no_sketch_plane;
#[path = "a_move_through_a_neighbour_inverts_the_body.rs"]
mod a_move_through_a_neighbour_inverts_the_body;
#[path = "a_pierce_held_in_a_curved_face.rs"]
mod a_pierce_held_in_a_curved_face;
#[path = "a_plane_across_a_one_face_wall.rs"]
mod a_plane_across_a_one_face_wall;
#[path = "a_plane_along_an_elliptic_cut.rs"]
mod a_plane_along_an_elliptic_cut;
#[path = "a_plane_holding_a_conic_tangent_at_a_vertex.rs"]
mod a_plane_holding_a_conic_tangent_at_a_vertex;
#[path = "a_plane_moved_to_a_cone_apex.rs"]
mod a_plane_moved_to_a_cone_apex;
#[path = "a_plane_through_a_cones_apex.rs"]
mod a_plane_through_a_cones_apex;
#[path = "a_plane_through_a_pole_cuts_a_cap.rs"]
mod a_plane_through_a_pole_cuts_a_cap;
#[path = "a_point_past_a_spline_solids_reach.rs"]
mod a_point_past_a_spline_solids_reach;
#[path = "a_pole_and_an_apex_join_nothing.rs"]
mod a_pole_and_an_apex_join_nothing;
#[path = "a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk.rs"]
mod a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk;
#[path = "a_ray_meets_a_curved_edge_once_for_both_faces.rs"]
mod a_ray_meets_a_curved_edge_once_for_both_faces;
#[path = "a_rigid_turn_keeps_a_slivers_volume.rs"]
mod a_rigid_turn_keeps_a_slivers_volume;
#[path = "a_rim_arc_on_a_cylinder_beside_a_spline_wall.rs"]
mod a_rim_arc_on_a_cylinder_beside_a_spline_wall;
#[path = "a_rim_covering_part_of_its_row_re_mints_the_rows_arc.rs"]
mod a_rim_covering_part_of_its_row_re_mints_the_rows_arc;
#[path = "a_rim_in_a_face_beside_a_resting_ridge.rs"]
mod a_rim_in_a_face_beside_a_resting_ridge;
#[path = "a_ring_on_a_cone_face.rs"]
mod a_ring_on_a_cone_face;
#[path = "a_ring_on_a_sphere_face.rs"]
mod a_ring_on_a_sphere_face;
#[path = "a_ruling_lying_on_a_wall.rs"]
mod a_ruling_lying_on_a_wall;
#[path = "a_ruling_through_a_box_corner_edge.rs"]
mod a_ruling_through_a_box_corner_edge;
#[path = "a_seam_region_carrying_both_poles.rs"]
mod a_seam_region_carrying_both_poles;
#[path = "a_seam_region_covering_the_sphere_twice.rs"]
mod a_seam_region_covering_the_sphere_twice;
#[path = "a_sheath_grafted_on_its_blade.rs"]
mod a_sheath_grafted_on_its_blade;
#[path = "a_short_shallow_strut_between_wide_walls_is_a_corner.rs"]
mod a_short_shallow_strut_between_wide_walls_is_a_corner;
#[path = "a_slit_tip_on_a_curved_face_reads_its_full_turn.rs"]
mod a_slit_tip_on_a_curved_face_reads_its_full_turn;
#[path = "a_spiric_cut_far_off_and_turned_reads_as_at_home.rs"]
mod a_spiric_cut_far_off_and_turned_reads_as_at_home;
#[path = "a_split_and_reminted_loft_holds_its_volume.rs"]
mod a_split_and_reminted_loft_holds_its_volume;
#[path = "a_split_edge_joins_back_on_its_carrier.rs"]
mod a_split_edge_joins_back_on_its_carrier;
#[path = "a_split_operand_does_not_finish.rs"]
mod a_split_operand_does_not_finish;
#[path = "a_split_through_a_wrap_edges_vertex.rs"]
mod a_split_through_a_wrap_edges_vertex;
#[path = "a_swaying_loft_corner_meters.rs"]
mod a_swaying_loft_corner_meters;
#[path = "a_sweep_keeps_the_frame_it_is_handed.rs"]
mod a_sweep_keeps_the_frame_it_is_handed;
#[path = "a_swept_cusp_is_legal_at_rest.rs"]
mod a_swept_cusp_is_legal_at_rest;
#[path = "a_tangent_arc_at_other_boolean_vertices.rs"]
mod a_tangent_arc_at_other_boolean_vertices;
#[path = "a_tangent_joint_on_near_concentric_carriers.rs"]
mod a_tangent_joint_on_near_concentric_carriers;
#[path = "a_thin_cap_keeps_its_digits.rs"]
mod a_thin_cap_keeps_its_digits;
#[path = "a_torus_off_its_ring_reads_as_at_rest.rs"]
mod a_torus_off_its_ring_reads_as_at_rest;
#[path = "a_tube_ending_on_a_ball.rs"]
mod a_tube_ending_on_a_ball;
#[path = "a_vertex_on_a_grazing_plane.rs"]
mod a_vertex_on_a_grazing_plane;
#[path = "a_vertex_piercing_a_lens_wall.rs"]
mod a_vertex_piercing_a_lens_wall;
#[path = "a_wall_row_read_in_band_takes_the_lane_a_miss_takes.rs"]
mod a_wall_row_read_in_band_takes_the_lane_a_miss_takes;
#[path = "a_wall_seam_takes_the_column_it_lies_on.rs"]
mod a_wall_seam_takes_the_column_it_lies_on;
#[path = "a_wrap_tip_reads_its_full_turn_in_the_boolean.rs"]
mod a_wrap_tip_reads_its_full_turn_in_the_boolean;
#[path = "a_wrap_tip_sits_on_its_faces_chart_singularity.rs"]
mod a_wrap_tip_sits_on_its_faces_chart_singularity;
#[path = "an_annular_tube_through_a_plate.rs"]
mod an_annular_tube_through_a_plate;
#[path = "an_apex_lying_in_a_partners_face.rs"]
mod an_apex_lying_in_a_partners_face;
#[path = "an_arc_loft_seams_its_rational_wall_at_every_spacing.rs"]
mod an_arc_loft_seams_its_rational_wall_at_every_spacing;
#[path = "an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read.rs"]
mod an_arc_off_a_plane_or_cylinder_face_ends_as_its_incidence_read;
#[path = "an_arc_rim_picks_the_row_it_lies_on.rs"]
mod an_arc_rim_picks_the_row_it_lies_on;
#[path = "an_arc_tangent_at_a_boolean_vertex.rs"]
mod an_arc_tangent_at_a_boolean_vertex;
#[path = "an_arcs_sweep_structure_is_certified_for_every_member.rs"]
mod an_arcs_sweep_structure_is_certified_for_every_member;
#[path = "an_edge_on_a_ruling_lying_in_its_own_face.rs"]
mod an_edge_on_a_ruling_lying_in_its_own_face;
#[path = "an_edge_running_past_a_face_reads_its_crossings.rs"]
mod an_edge_running_past_a_face_reads_its_crossings;
#[path = "an_inside_out_thin_tube_operand.rs"]
mod an_inside_out_thin_tube_operand;
#[path = "at_rest_pcurve_faces.rs"]
mod at_rest_pcurve_faces;
#[path = "band_apart_partners_on_a_steep_ellipse.rs"]
mod band_apart_partners_on_a_steep_ellipse;
#[path = "band_chamfer_frustum.rs"]
mod band_chamfer_frustum;
#[path = "band_closed_mixed_chain.rs"]
mod band_closed_mixed_chain;
#[path = "band_closed_mixed_chain_shapes.rs"]
mod band_closed_mixed_chain_shapes;
#[path = "band_mixed_corner.rs"]
mod band_mixed_corner;
#[path = "band_mixed_corner_odd_plus_one.rs"]
mod band_mixed_corner_odd_plus_one;
#[path = "band_mixed_corner_reach.rs"]
mod band_mixed_corner_reach;
#[path = "band_mixed_rim_shared_supports.rs"]
mod band_mixed_rim_shared_supports;
#[path = "band_subdivided_side_walls.rs"]
mod band_subdivided_side_walls;
#[path = "bisector_lane_stubs.rs"]
mod bisector_lane_stubs;
#[path = "bisector_nodes.rs"]
mod bisector_nodes;
#[path = "blend_arms_centre_placement.rs"]
mod blend_arms_centre_placement;
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
#[path = "bool5r2_wedge_reach.rs"]
mod bool5r2_wedge_reach;
#[path = "bool6_per_slab_stacking.rs"]
mod bool6_per_slab_stacking;
#[path = "bool6_r2_probes.rs"]
mod bool6_r2_probes;
#[path = "bool6r1_probes.rs"]
mod bool6r1_probes;
#[path = "bool6r1_probes_interval.rs"]
mod bool6r1_probes_interval;
#[path = "boolean_tangent_rim_review.rs"]
mod boolean_tangent_rim_review;
#[path = "carved_sphere_operand.rs"]
mod carved_sphere_operand;
#[path = "cell_region_bodies.rs"]
mod cell_region_bodies;
#[path = "clearance_reads_an_iso_bounded_wall.rs"]
mod clearance_reads_an_iso_bounded_wall;
#[path = "cone_join_lane.rs"]
mod cone_join_lane;
#[path = "cone_operand_rows.rs"]
mod cone_operand_rows;
#[path = "continuation_pose.rs"]
mod continuation_pose;
#[path = "crossing_equal_rods.rs"]
mod crossing_equal_rods;
#[path = "curved_copy_against_itself.rs"]
mod curved_copy_against_itself;
#[path = "curved_copy_review_probes.rs"]
mod curved_copy_review_probes;
#[path = "curved_edge_corpus.rs"]
mod curved_edge_corpus;
#[path = "curved_edge_ray_probes.rs"]
mod curved_edge_ray_probes;
#[path = "curved_trims_build_at_their_volumes.rs"]
mod curved_trims_build_at_their_volumes;
#[path = "cylinder_sphere_frame.rs"]
mod cylinder_sphere_frame;
#[path = "degree1_loft_slabs.rs"]
mod degree1_loft_slabs;
#[path = "four_crossings_on_one_section_circle.rs"]
mod four_crossings_on_one_section_circle;
#[path = "general_circle_octant_dual.rs"]
mod general_circle_octant_dual;
#[path = "join_door_is_whole.rs"]
mod join_door_is_whole;
#[path = "lane1_r2_probes.rs"]
mod lane1_r2_probes;
#[path = "offb_r1_loft_probes.rs"]
mod offb_r1_loft_probes;
#[path = "offc_r1_probes.rs"]
mod offc_r1_probes;
#[path = "offd2_r1_probes.rs"]
mod offd2_r1_probes;
#[path = "offd_r1_probes.rs"]
mod offd_r1_probes;
#[path = "offset_axial_door_reads_charts_not_material.rs"]
mod offset_axial_door_reads_charts_not_material;
#[path = "offset_doors_end_with_the_join.rs"]
mod offset_doors_end_with_the_join;
#[path = "offset_restates_a_neighbour_chart_rim.rs"]
mod offset_restates_a_neighbour_chart_rim;
#[path = "one_door_for_coincident_sections.rs"]
mod one_door_for_coincident_sections;
#[path = "one_region_rows.rs"]
mod one_region_rows;
#[path = "one_segment_loop.rs"]
mod one_segment_loop;
#[path = "onpair_rim_pcurves.rs"]
mod onpair_rim_pcurves;
#[path = "operand_gate_pose.rs"]
mod operand_gate_pose;
#[path = "operand_gate_support_plates.rs"]
mod operand_gate_support_plates;
#[path = "p1b_r1_probes.rs"]
mod p1b_r1_probes;
#[path = "parallel_cylinder_join.rs"]
mod parallel_cylinder_join;
#[path = "pcurve_p1b_r2_probes.rs"]
mod pcurve_p1b_r2_probes;
#[path = "pieces_oblique_bore.rs"]
mod pieces_oblique_bore;
#[path = "pinned_corners_at_the_loft_door.rs"]
mod pinned_corners_at_the_loft_door;
#[path = "pins_of_todays_blend_refusals.rs"]
mod pins_of_todays_blend_refusals;
#[path = "pipeline_null_edge_rows.rs"]
mod pipeline_null_edge_rows;
#[path = "pis_arc_capped_poses.rs"]
mod pis_arc_capped_poses;
#[path = "pis_cut_cavity.rs"]
mod pis_cut_cavity;
#[path = "placeholder_loop_windings.rs"]
mod placeholder_loop_windings;
#[path = "planar_ring_arc_closure.rs"]
mod planar_ring_arc_closure;
#[path = "pocket_ring_steep_ellipse.rs"]
mod pocket_ring_steep_ellipse;
#[path = "pocket_wall_crossing_a_side_face.rs"]
mod pocket_wall_crossing_a_side_face;
#[path = "point_in_loop_arc_cap.rs"]
mod point_in_loop_arc_cap;
#[path = "point_touch_records.rs"]
mod point_touch_records;
#[path = "pole_slit_window.rs"]
mod pole_slit_window;
#[path = "quad_frame_remeasure.rs"]
mod quad_frame_remeasure;
#[path = "r1_lane0_e2e.rs"]
mod r1_lane0_e2e;
#[path = "r1_mate3_probes.rs"]
mod r1_mate3_probes;
#[path = "r2_mate3_probes.rs"]
mod r2_mate3_probes;
#[path = "r2_mesh1_donut_probes.rs"]
mod r2_mesh1_donut_probes;
#[path = "rehome_rings_lune.rs"]
mod rehome_rings_lune;
#[path = "review_4704_flush_round_oracle.rs"]
mod review_4704_flush_round_oracle;
#[path = "review_4996_blend_cap.rs"]
mod review_4996_blend_cap;
#[path = "review_5956_r1_probes.rs"]
mod review_5956_r1_probes;
#[path = "review_5956_r2_probes.rs"]
mod review_5956_r2_probes;
#[path = "review_arm_vs_reach_probes.rs"]
mod review_arm_vs_reach_probes;
#[path = "review_r2_vv_probes.rs"]
mod review_r2_vv_probes;
#[path = "review_sixx_r2_probes.rs"]
mod review_sixx_r2_probes;
#[path = "saddle_wall_offset_fit.rs"]
mod saddle_wall_offset_fit;
#[path = "scaffolding_on_a_blend_support_face_does_not_finish.rs"]
mod scaffolding_on_a_blend_support_face_does_not_finish;
#[path = "seam_region_cover_census.rs"]
mod seam_region_cover_census;
#[path = "self_intersection_proves_injectivity.rs"]
mod self_intersection_proves_injectivity;
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
#[path = "shallow_arc_extrude_grid_interval.rs"]
mod shallow_arc_extrude_grid_interval;
#[path = "sheets_part_on_slanted_rims.rs"]
mod sheets_part_on_slanted_rims;
#[path = "shellfix1_bitdump.rs"]
mod shellfix1_bitdump;
#[path = "shellfix1_r1_probes.rs"]
mod shellfix1_r1_probes;
#[path = "spheres_crossing_off_every_edge.rs"]
mod spheres_crossing_off_every_edge;
#[path = "spiric_faces_fuzz.rs"]
mod spiric_faces_fuzz;
#[path = "sym11_far_placement_rows.rs"]
mod sym11_far_placement_rows;
#[path = "the_door_reads_seam_regions_as_tier_3_does.rs"]
mod the_door_reads_seam_regions_as_tier_3_does;
#[path = "tilted_sphere_pair.rs"]
mod tilted_sphere_pair;
#[path = "tilted_sphere_pair_k_rows.rs"]
mod tilted_sphere_pair_k_rows;
#[path = "topo_ring_nesting.rs"]
mod topo_ring_nesting;
#[path = "torax_axial.rs"]
mod torax_axial;
#[path = "torax_interval.rs"]
mod torax_interval;
#[path = "torus_doors_join.rs"]
mod torus_doors_join;
#[path = "torus_region_probe.rs"]
mod torus_region_probe;
#[path = "transform_nurbs_walls.rs"]
mod transform_nurbs_walls;
#[path = "trimmed_sphere_aims.rs"]
mod trimmed_sphere_aims;
#[path = "tube_rim_touch_rows.rs"]
mod tube_rim_touch_rows;
#[path = "two_rims_review_probes.rs"]
mod two_rims_review_probes;
#[path = "two_tubes_in_the_zero_band.rs"]
mod two_tubes_in_the_zero_band;
#[path = "two_wedge_ends_on_one_line.rs"]
mod two_wedge_ends_on_one_line;
#[path = "verbs_offc_consumer.rs"]
mod verbs_offc_consumer;
#[path = "verbs_offd.rs"]
mod verbs_offd;
#[path = "verbs_shell.rs"]
mod verbs_shell;

#[path = "a_thin_wall_bounds_the_band.rs"]
mod a_thin_wall_bounds_the_band;
#[path = "a_tiny_lune_on_a_wide_sphere_reads_its_own_extent.rs"]
mod a_tiny_lune_on_a_wide_sphere_reads_its_own_extent;
#[path = "axis_lap.rs"]
mod axis_lap;
#[path = "band_annulus_host_boundary.rs"]
mod band_annulus_host_boundary;
#[path = "band_co_requested_boundary.rs"]
mod band_co_requested_boundary;
#[path = "band_concave_bar.rs"]
mod band_concave_bar;
#[path = "band_dihedral_cups.rs"]
mod band_dihedral_cups;
#[path = "band_mixed_vertex.rs"]
mod band_mixed_vertex;
#[path = "band_one_cycle_members.rs"]
mod band_one_cycle_members;
#[path = "band_open_mixed_chain.rs"]
mod band_open_mixed_chain;
#[path = "band_open_mixed_chain_shapes.rs"]
mod band_open_mixed_chain_shapes;
#[path = "band_planar_consume.rs"]
mod band_planar_consume;
#[path = "band_planar_cut_off.rs"]
mod band_planar_cut_off;
#[path = "band_planar_cut_off_interval.rs"]
mod band_planar_cut_off_interval;
#[path = "band_planar_cut_off_meters.rs"]
mod band_planar_cut_off_meters;
#[path = "band_planar_cut_off_shapes.rs"]
mod band_planar_cut_off_shapes;
#[path = "band_planar_mitre.rs"]
mod band_planar_mitre;
#[path = "band_planar_oblique_fillet.rs"]
mod band_planar_oblique_fillet;
#[path = "band_reads_three_ways.rs"]
mod band_reads_three_ways;
#[path = "band_ruled_cap_ring.rs"]
mod band_ruled_cap_ring;
#[path = "band_ruled_d_hole.rs"]
mod band_ruled_d_hole;
#[path = "band_strip_mouth_cups.rs"]
mod band_strip_mouth_cups;
#[path = "band_turn_overrun_chamfer.rs"]
mod band_turn_overrun_chamfer;
#[path = "band_turn_overrun_fillet.rs"]
mod band_turn_overrun_fillet;
#[path = "band_turn_overrun_fillet_model.rs"]
mod band_turn_overrun_fillet_model;
#[path = "band_turn_overrun_oracles.rs"]
mod band_turn_overrun_oracles;
#[path = "band_turn_overrun_probe.rs"]
mod band_turn_overrun_probe;
#[path = "band_turn_overrun_verdict.rs"]
mod band_turn_overrun_verdict;
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
#[path = "blend_ball_side_bits.rs"]
mod blend_ball_side_bits;
#[path = "blend_band_reach.rs"]
mod blend_band_reach;
#[path = "blend_band_reach_chain_ends.rs"]
mod blend_band_reach_chain_ends;
#[path = "blend_band_reach_oracle.rs"]
mod blend_band_reach_oracle;
#[path = "blend_band_reach_rows.rs"]
mod blend_band_reach_rows;
#[path = "blend_bore_two_rims.rs"]
mod blend_bore_two_rims;
#[path = "blend_dual_tangent.rs"]
mod blend_dual_tangent;
#[path = "blend_margin_payload_interval.rs"]
mod blend_margin_payload_interval;
#[path = "blend_operand_gate.rs"]
mod blend_operand_gate;
#[path = "blend_seam_split_rim.rs"]
mod blend_seam_split_rim;
#[path = "blend_tworims.rs"]
mod blend_tworims;
#[path = "blend_value_decided_rows.rs"]
mod blend_value_decided_rows;
#[path = "bool5r1_probes.rs"]
mod bool5r1_probes;
#[path = "census_reads_conic_records.rs"]
mod census_reads_conic_records;
#[path = "cert5_offgrid_knot_rational.rs"]
mod cert5_offgrid_knot_rational;
#[path = "cert8_r1_probes.rs"]
mod cert8_r1_probes;
#[path = "closed_chain_junctions.rs"]
mod closed_chain_junctions;
#[path = "conic_edge_curved_face.rs"]
mod conic_edge_curved_face;
#[path = "contact11_torus_chart_l.rs"]
mod contact11_torus_chart_l;
#[path = "contact_edge_must_carry.rs"]
mod contact_edge_must_carry;
#[path = "contained_flush_cylinder.rs"]
mod contained_flush_cylinder;
#[path = "contfp_reads_arcs_on_their_carriers.rs"]
mod contfp_reads_arcs_on_their_carriers;
#[path = "copied_carriers_at_interval.rs"]
mod copied_carriers_at_interval;
#[path = "corner_patch_chart_poles.rs"]
mod corner_patch_chart_poles;
#[path = "edges_lying_in_a_face_part_wherever_they_lie.rs"]
mod edges_lying_in_a_face_part_wherever_they_lie;
#[path = "ellipse_torus.rs"]
mod ellipse_torus;
#[path = "encl_curved_loft_shell.rs"]
mod encl_curved_loft_shell;
#[path = "euler_site_row_frontiers.rs"]
mod euler_site_row_frontiers;
#[path = "euler_site_rows_equal_the_pass.rs"]
mod euler_site_rows_equal_the_pass;
#[path = "extrude_acceptance.rs"]
mod extrude_acceptance;
#[path = "extrude_interval.rs"]
mod extrude_interval;
#[path = "face_face_touch.rs"]
mod face_face_touch;
#[path = "issue93_az_intersect.rs"]
mod issue93_az_intersect;
#[path = "join1_r2_probes.rs"]
mod join1_r2_probes;
#[path = "join1_r2_rand.rs"]
mod join1_r2_rand;
#[path = "join_whole_orbit_cylinder.rs"]
mod join_whole_orbit_cylinder;
#[path = "k_report.rs"]
mod k_report;
#[path = "ladder_split_key.rs"]
mod ladder_split_key;
#[path = "lamina_annulus.rs"]
mod lamina_annulus;
#[path = "lib_u3_sections.rs"]
mod lib_u3_sections;
#[path = "loft_profiles_door.rs"]
mod loft_profiles_door;
#[path = "loft_v_is_the_section_set.rs"]
mod loft_v_is_the_section_set;
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
#[path = "must_carry_rule.rs"]
mod must_carry_rule;
#[path = "near_circular_sections.rs"]
mod near_circular_sections;
#[path = "r1_probes_issue1362_donut.rs"]
mod r1_probes_issue1362_donut;
#[path = "r2_sense_fold_probes.rs"]
mod r2_sense_fold_probes;
#[path = "radius_headroom_sides.rs"]
mod radius_headroom_sides;
#[path = "ray_wall_margin_twins.rs"]
mod ray_wall_margin_twins;
#[path = "reach_aligned_half_rods.rs"]
mod reach_aligned_half_rods;
#[path = "reach_cone_root_lane.rs"]
mod reach_cone_root_lane;
#[path = "reach_cone_split.rs"]
mod reach_cone_split;
#[path = "reach_continuation.rs"]
mod reach_continuation;
#[path = "reach_split_gate_azimuth.rs"]
mod reach_split_gate_azimuth;
#[path = "reach_split_gate_per_face.rs"]
mod reach_split_gate_per_face;
#[path = "reach_split_gate_pose.rs"]
mod reach_split_gate_pose;
#[path = "reach_split_gate_window.rs"]
mod reach_split_gate_window;
#[path = "reach_wall_chord_rows.rs"]
mod reach_wall_chord_rows;
#[path = "readback_doors.rs"]
mod readback_doors;
#[path = "recourse_roster.rs"]
mod recourse_roster;
#[path = "review_4704_r1_probes.rs"]
mod review_4704_r1_probes;
#[path = "review_4781_r1_probes.rs"]
mod review_4781_r1_probes;
#[path = "review_4781_r2_probes.rs"]
mod review_4781_r2_probes;
#[path = "review_arceval_r1_probes.rs"]
mod review_arceval_r1_probes;
#[path = "review_arms2_r1_probes.rs"]
mod review_arms2_r1_probes;
#[path = "review_arms3_r1_probes.rs"]
mod review_arms3_r1_probes;
#[path = "review_band_ruled_ring_probes.rs"]
mod review_band_ruled_ring_probes;
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
#[path = "review_cleave_wrongarc.rs"]
mod review_cleave_wrongarc;
#[path = "review_closed_chain_junctions_r2_probes.rs"]
mod review_closed_chain_junctions_r2_probes;
#[path = "review_contact_edge_must_carry_r2_probes.rs"]
mod review_contact_edge_must_carry_r2_probes;
#[path = "review_d2_adv_probes.rs"]
mod review_d2_adv_probes;
#[path = "review_d2_recourse_at_the_site.rs"]
mod review_d2_recourse_at_the_site;
#[path = "review_d8_consumer_differential.rs"]
mod review_d8_consumer_differential;
#[path = "review_fillet_e1_probes.rs"]
mod review_fillet_e1_probes;
#[path = "review_fillet_e3_probes.rs"]
mod review_fillet_e3_probes;
#[path = "review_ladder_split_key_r1_probes.rs"]
mod review_ladder_split_key_r1_probes;
#[path = "review_ladder_split_key_r2_probes.rs"]
mod review_ladder_split_key_r2_probes;
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
#[path = "review_must_carry_rule_r1_probes.rs"]
mod review_must_carry_rule_r1_probes;
#[path = "review_must_carry_rule_r2_probes.rs"]
mod review_must_carry_rule_r2_probes;
#[path = "review_pr12_probes.rs"]
mod review_pr12_probes;
#[path = "review_recourse_roster_r2_probes.rs"]
mod review_recourse_roster_r2_probes;
#[path = "review_s11_adv.rs"]
mod review_s11_adv;
#[path = "review_s12_adv.rs"]
mod review_s12_adv;
#[path = "review_s6_probe.rs"]
mod review_s6_probe;
#[path = "review_verbs_rim_lever_probes.rs"]
mod review_verbs_rim_lever_probes;
#[path = "revolve_axis_gaps.rs"]
mod revolve_axis_gaps;
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
#[path = "round_tube_on_plate.rs"]
mod round_tube_on_plate;
#[path = "s16_box_soundness.rs"]
mod s16_box_soundness;
#[path = "s393_start_frame_door.rs"]
mod s393_start_frame_door;
#[path = "s49_census_jurisdiction.rs"]
mod s49_census_jurisdiction;
#[path = "seam_vertex_sites.rs"]
mod seam_vertex_sites;
#[path = "shell_cavity_embedded.rs"]
mod shell_cavity_embedded;
#[path = "shell_general_door.rs"]
mod shell_general_door;
#[path = "strut_cover_on_cylinder_pairs.rs"]
mod strut_cover_on_cylinder_pairs;

#[path = "run_walls_built.rs"]
mod run_walls_built;
#[path = "section_ellipse_seam_poses.rs"]
mod section_ellipse_seam_poses;
#[path = "section_end_poses.rs"]
mod section_end_poses;
#[path = "split_across_a_revolve_seam.rs"]
mod split_across_a_revolve_seam;
#[path = "split_along_a_face_plane.rs"]
mod split_along_a_face_plane;
#[path = "split_cylindrical_feature_box.rs"]
mod split_cylindrical_feature_box;
#[path = "split_edge_fitted_rows.rs"]
mod split_edge_fitted_rows;
#[path = "split_edge_loft_charts.rs"]
mod split_edge_loft_charts;
#[path = "split_edge_on_faces_poses.rs"]
mod split_edge_on_faces_poses;
#[path = "split_seam_vanishing_tilt.rs"]
mod split_seam_vanishing_tilt;
#[path = "split_section_rings.rs"]
mod split_section_rings;
#[path = "split_tangent_edge_curved.rs"]
mod split_tangent_edge_curved;
#[path = "split_through_a_ruling.rs"]
mod split_through_a_ruling;
#[path = "turning_orientation.rs"]
mod turning_orientation;
#[path = "verbs_1031b_arcwind.rs"]
mod verbs_1031b_arcwind;
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
#[path = "verbs_cylsph_opening.rs"]
mod verbs_cylsph_opening;
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
#[path = "wall_face_tangent_reach.rs"]
mod wall_face_tangent_reach;

test_utils::every_suite_file_is_aggregated!();

#[path = "m8_3_rational_volume.rs"]
mod m8_3_rational_volume;

#[path = "mass_props_are_thread_count_invariant.rs"]
mod mass_props_are_thread_count_invariant;

#[path = "thread_count_probe_populations.rs"]
mod thread_count_probe_populations;

#[path = "sign_walk_plus_v.rs"]
mod sign_walk_plus_v;

#[path = "shell_census_is_thread_count_invariant.rs"]
mod shell_census_is_thread_count_invariant;
#[path = "shell_winding_curved.rs"]
mod shell_winding_curved;

#[path = "continuation_is_thread_count_invariant.rs"]
mod continuation_is_thread_count_invariant;

#[path = "m8_4_intersection_iso.rs"]
mod m8_4_intersection_iso;

#[path = "m9_2_chart_region_loft.rs"]
mod m9_2_chart_region_loft;

#[path = "r2_probe_cert8.rs"]
mod r2_probe_cert8;

#[path = "m9_3_wall_door.rs"]
mod m9_3_wall_door;

#[path = "a_boolean_on_a_pole_touching_octant.rs"]
mod a_boolean_on_a_pole_touching_octant;
#[path = "a_ringed_wall_divided_by_a_carried_chord.rs"]
mod a_ringed_wall_divided_by_a_carried_chord;
#[path = "extent_scan_off_face_tangency.rs"]
mod extent_scan_off_face_tangency;
#[path = "far_walls_read_their_whole_turns.rs"]
mod far_walls_read_their_whole_turns;
#[path = "full_turn_wall.rs"]
mod full_turn_wall;
#[path = "germ_circle_torus.rs"]
mod germ_circle_torus;
#[path = "germ_cone_apex_closure.rs"]
mod germ_cone_apex_closure;
#[path = "germ_cone_doors.rs"]
mod germ_cone_doors;
#[path = "germ_cone_section_rows.rs"]
mod germ_cone_section_rows;
#[path = "germ_conic_plane_roots.rs"]
mod germ_conic_plane_roots;
#[path = "germ_coplanar_conic.rs"]
mod germ_coplanar_conic;
#[path = "germ_interior_oval.rs"]
mod germ_interior_oval;
#[path = "germ_interior_saddle.rs"]
mod germ_interior_saddle;
#[path = "germ_plane_torus_chord.rs"]
mod germ_plane_torus_chord;
#[path = "germ_plane_torus_frame.rs"]
mod germ_plane_torus_frame;
#[path = "germ_plane_torus_spiric.rs"]
mod germ_plane_torus_spiric;
#[path = "germ_radial_hole.rs"]
mod germ_radial_hole;
#[path = "germ_sphere_no_crossings.rs"]
mod germ_sphere_no_crossings;
#[path = "germ_sphere_torus_placements.rs"]
mod germ_sphere_torus_placements;
#[path = "germ_tilted_rod.rs"]
mod germ_tilted_rod;
#[path = "germ_torus_components.rs"]
mod germ_torus_components;
#[path = "germ_torus_doors.rs"]
mod germ_torus_doors;
#[path = "germ_torus_rods.rs"]
mod germ_torus_rods;
#[path = "join1_delta_probes.rs"]
mod join1_delta_probes;
#[path = "join1_mechanisms.rs"]
mod join1_mechanisms;
#[path = "join1_r1_probes.rs"]
mod join1_r1_probes;
#[path = "join1_r1_rows.rs"]
mod join1_r1_rows;
#[path = "join2_r1_probes.rs"]
mod join2_r1_probes;
#[path = "join2_r2_probes.rs"]
mod join2_r2_probes;
#[path = "join3_r2_probes.rs"]
mod join3_r2_probes;
#[path = "join3_review_r1.rs"]
mod join3_review_r1;
#[path = "join_pierce_runs_sweep.rs"]
mod join_pierce_runs_sweep;
#[path = "join_pierce_strut_facing.rs"]
mod join_pierce_strut_facing;
#[path = "join_rc_probes.rs"]
mod join_rc_probes;
#[path = "join_reflex_wedge_probes.rs"]
mod join_reflex_wedge_probes;
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
#[path = "mate7a_torus_rest.rs"]
mod mate7a_torus_rest;
#[path = "near_tangent_census_edge_face_lane.rs"]
mod near_tangent_census_edge_face_lane;
#[path = "near_tangent_pierce_poses.rs"]
mod near_tangent_pierce_poses;
#[path = "nurbs_plane_guard.rs"]
mod nurbs_plane_guard;
#[path = "pi_seam_and_kiss_through_the_boolean.rs"]
mod pi_seam_and_kiss_through_the_boolean;
#[path = "pierce_tangent_off_face.rs"]
mod pierce_tangent_off_face;
#[path = "snowman.rs"]
mod snowman;
#[path = "tang_circle_cylinder.rs"]
mod tang_circle_cylinder;
#[path = "torus_touch_off_faces.rs"]
mod torus_touch_off_faces;

#[path = "review_probes_m8_4.rs"]
mod review_probes_m8_4;
#[path = "review_ring_clearance_r1_probes.rs"]
mod review_ring_clearance_r1_probes;
#[path = "review_ring_clearance_r2_probes.rs"]
mod review_ring_clearance_r2_probes;
#[path = "ring_clearance_forms.rs"]
mod ring_clearance_forms;
#[path = "vrev_acceptance_set.rs"]
mod vrev_acceptance_set;
#[path = "vrev_reversed_chart_hazard.rs"]
mod vrev_reversed_chart_hazard;

#[path = "r1_probes_m9_3.rs"]
mod r1_probes_m9_3;

#[path = "curved_mergedoor.rs"]
mod curved_mergedoor;

#[path = "a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta.rs"]
mod a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta;
#[path = "curved_coincident_fates.rs"]
mod curved_coincident_fates;
#[path = "curved_copies_filling_one_volume.rs"]
mod curved_copies_filling_one_volume;
#[path = "curved_notch_pinch.rs"]
mod curved_notch_pinch;

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
#[path = "cert_m2r1_head.rs"]
mod cert_m2r1_head;
#[path = "cert_m2r1_passes.rs"]
mod cert_m2r1_passes;
#[path = "r1_area_gauge_probes.rs"]
mod r1_area_gauge_probes;

#[path = "tcost_k3_certificate.rs"]
mod tcost_k3_certificate;

#[path = "fillet_h4_concave_rim.rs"]
mod fillet_h4_concave_rim;
#[path = "fillet_h4_concave_rim_interval.rs"]
mod fillet_h4_concave_rim_interval;
#[path = "fillet_h5_hostless_rim.rs"]
mod fillet_h5_hostless_rim;
#[path = "fillet_h5_hostless_rim_interval.rs"]
mod fillet_h5_hostless_rim_interval;
#[path = "fillet_h5_r2_probes.rs"]
mod fillet_h5_r2_probes;
#[path = "review_fillet_h5_r1_probes.rs"]
mod review_fillet_h5_r1_probes;

#[path = "blend_carries_records.rs"]
mod blend_carries_records;
#[path = "blend_per_shell.rs"]
mod blend_per_shell;
#[path = "blend_per_shell_carry.rs"]
mod blend_per_shell_carry;
#[path = "blend_recourse_followability.rs"]
mod blend_recourse_followability;
#[path = "review_blend3_r3_probes.rs"]
mod review_blend3_r3_probes;
#[path = "shell_carries_records.rs"]
mod shell_carries_records;

#[path = "review_fillet_e2_probes.rs"]
mod review_fillet_e2_probes;
#[path = "ring_carry_through_by_piece.rs"]
mod ring_carry_through_by_piece;

#[path = "review_h4_r1_probes.rs"]
mod review_h4_r1_probes;

#[path = "fillet_h4_r2_probes.rs"]
mod fillet_h4_r2_probes;
#[path = "rim_of_rows.rs"]
mod rim_of_rows;

#[path = "rim_of_rows_interval.rs"]
mod rim_of_rows_interval;

#[path = "rim_of_r1_probes.rs"]
mod rim_of_r1_probes;

#[path = "rim_of_structural_review_probes.rs"]
mod rim_of_structural_review_probes;

#[path = "fillet_h6_cap_rim.rs"]
mod fillet_h6_cap_rim;
#[path = "n3r1_d31.rs"]
mod n3r1_d31;
#[path = "n3r1_prune.rs"]
mod n3r1_prune;
#[path = "n3r2_d31_bitid.rs"]
mod n3r2_d31_bitid;

#[path = "review_blend_e2_r1_probes.rs"]
mod review_blend_e2_r1_probes;
#[path = "review_fillet_h6_r1_probes.rs"]
mod review_fillet_h6_r1_probes;

#[path = "review_fillet_h6_r2_probes.rs"]
mod review_fillet_h6_r2_probes;

#[path = "review_blend_k_rk_probes.rs"]
mod review_blend_k_rk_probes;

#[path = "fillet_h7_transverse_cap.rs"]
mod fillet_h7_transverse_cap;
#[path = "fillet_h7_transverse_cap_interval.rs"]
mod fillet_h7_transverse_cap_interval;

#[path = "review_blend1_r1_probes.rs"]
mod review_blend1_r1_probes;
#[path = "review_closed_chain_junctions_r1_probes.rs"]
mod review_closed_chain_junctions_r1_probes;
#[path = "review_fillet_h7_r1_probes.rs"]
mod review_fillet_h7_r1_probes;
#[path = "review_fillet_split_r2_probes.rs"]
mod review_fillet_split_r2_probes;
#[path = "review_fillet_t_r1_probes.rs"]
mod review_fillet_t_r1_probes;

#[path = "review_fillet_h7_r2_probes.rs"]
mod review_fillet_h7_r2_probes;

#[path = "review_fillet_t_r2_probes.rs"]
mod review_fillet_t_r2_probes;

#[path = "review_blend5_r5_probes.rs"]
mod review_blend5_r5_probes;

#[path = "review_blend4_r4_probes.rs"]
mod review_blend4_r4_probes;
#[path = "review_contact_edge_must_carry_r1_probes.rs"]
mod review_contact_edge_must_carry_r1_probes;
#[path = "shell5_r1_dump.rs"]
mod shell5_r1_dump;
#[path = "shell5_r1_probes.rs"]
mod shell5_r1_probes;
#[path = "shell5_r2_probes.rs"]
mod shell5_r2_probes;
#[path = "shell6_nappe_home.rs"]
mod shell6_nappe_home;
#[path = "shell6_r1_probes.rs"]
mod shell6_r1_probes;
#[path = "shell6_r2_probes.rs"]
mod shell6_r2_probes;

#[path = "shell7_common.rs"]
mod shell7_common;

#[path = "shell7_dump.rs"]
mod shell7_dump;
#[path = "shell8_common.rs"]
mod shell8_common;
#[path = "shell8_dump.rs"]
mod shell8_dump;
#[path = "shell8_multi_solid.rs"]
mod shell8_multi_solid;
#[path = "shell8_r1_probes.rs"]
mod shell8_r1_probes;
#[path = "shell8_r2_probes.rs"]
mod shell8_r2_probes;

#[path = "shell7_seam_corner.rs"]
mod shell7_seam_corner;
#[path = "spiric_rim.rs"]
mod spiric_rim;
#[path = "spline_spiric_operands.rs"]
mod spline_spiric_operands;
#[path = "spline_subtract_intersect_census.rs"]
mod spline_subtract_intersect_census;

#[path = "shell7_r1_diff.rs"]
mod shell7_r1_diff;

#[path = "shell7_r2_probes.rs"]
mod shell7_r2_probes;

#[path = "shell9_probe.rs"]
mod shell9_probe;

#[path = "revert_plane_charts.rs"]
mod revert_plane_charts;

#[path = "revert_periodic_wrap.rs"]
mod revert_periodic_wrap;

#[path = "shell9_rows.rs"]
mod shell9_rows;

#[path = "shell9_r1_probes.rs"]
mod shell9_r1_probes;

#[path = "shell9_r2_probes.rs"]
mod shell9_r2_probes;

#[path = "shell9_r2_dump.rs"]
mod shell9_r2_dump;

#[path = "shell10_scoped_walks.rs"]
mod shell10_scoped_walks;

#[path = "shell10_r1_probes.rs"]
mod shell10_r1_probes;

#[path = "shell10_r2_probes.rs"]
mod shell10_r2_probes;

#[path = "shell10_r2_cost.rs"]
mod shell10_r2_cost;

#[path = "shell10_r2_dump.rs"]
mod shell10_r2_dump;

#[path = "cross_solid_apart.rs"]
mod cross_solid_apart;

#[path = "holdings_of_a_sunk_boss.rs"]
mod holdings_of_a_sunk_boss;

#[path = "census_containment_cause.rs"]
mod census_containment_cause;

#[path = "wire_loft_end_profile_lift.rs"]
mod wire_loft_end_profile_lift;

#[path = "wedge_end_doors.rs"]
mod wedge_end_doors;

#[path = "a_knife_edge_fillet_builds_where_its_ball_is_defined.rs"]
mod a_knife_edge_fillet_builds_where_its_ball_is_defined;

#[path = "review_4831_probes.rs"]
mod review_4831_probes;

#[path = "review_3701_probes.rs"]
mod review_3701_probes;

#[path = "review_ring2_r1_e2e.rs"]
mod review_ring2_r1_e2e;

#[path = "full_turn_bore_mate.rs"]
mod full_turn_bore_mate;

#[path = "wedge_through_a_full_turn_collar.rs"]
mod wedge_through_a_full_turn_collar;

#[path = "rest_mate_every_op.rs"]
mod rest_mate_every_op;
#[path = "witness_ladder.rs"]
mod witness_ladder;

#[path = "boss_flush_with_a_block_edge.rs"]
mod boss_flush_with_a_block_edge;
#[path = "far_thin_disc_sign.rs"]
mod far_thin_disc_sign;
#[path = "pinch_faces_tessellate.rs"]
mod pinch_faces_tessellate;
#[path = "rest_nested_strut.rs"]
mod rest_nested_strut;
#[path = "rest_zip_admission.rs"]
mod rest_zip_admission;
#[path = "session_box_reach.rs"]
mod session_box_reach;

#[path = "a_grazed_rim_never_pinches.rs"]
mod a_grazed_rim_never_pinches;
#[path = "letterform_intersects_in_every_order.rs"]
mod letterform_intersects_in_every_order;
#[path = "pole_ball_shells.rs"]
mod pole_ball_shells;
#[path = "seam_siblings_flipped_together.rs"]
mod seam_siblings_flipped_together;
#[path = "shell_curved_mouth.rs"]
mod shell_curved_mouth;
#[path = "sphere_face_flips_are_refused.rs"]
mod sphere_face_flips_are_refused;

#[path = "a_body_built_turned_is_the_body_turned.rs"]
mod a_body_built_turned_is_the_body_turned;
#[path = "a_boxed_motion_moves_a_finished_body.rs"]
mod a_boxed_motion_moves_a_finished_body;
#[path = "a_closed_conic_in_the_partners_face.rs"]
mod a_closed_conic_in_the_partners_face;
#[path = "a_cone_with_a_tilted_seam.rs"]
mod a_cone_with_a_tilted_seam;
#[path = "a_cylinder_through_a_hole_tangent_to_its_wall.rs"]
mod a_cylinder_through_a_hole_tangent_to_its_wall;
#[path = "a_frame_within_its_rounding_is_kept_and_one_beyond_it_refuses.rs"]
mod a_frame_within_its_rounding_is_kept_and_one_beyond_it_refuses;
#[path = "a_mirrored_body_is_its_image.rs"]
mod a_mirrored_body_is_its_image;
#[path = "a_moved_body_keeps_its_verdict.rs"]
mod a_moved_body_keeps_its_verdict;
#[path = "a_near_right_angle_drum_pair.rs"]
mod a_near_right_angle_drum_pair;
#[path = "a_revolved_wall_arc_join_reads_its_reach.rs"]
mod a_revolved_wall_arc_join_reads_its_reach;
#[path = "a_seam_inside_a_face_is_no_boundary_of_it.rs"]
mod a_seam_inside_a_face_is_no_boundary_of_it;
#[path = "an_aimed_ray_from_far_reads_a_split_side_right.rs"]
mod an_aimed_ray_from_far_reads_a_split_side_right;
#[path = "an_aimed_ray_keeps_its_line_over_a_box.rs"]
mod an_aimed_ray_keeps_its_line_over_a_box;
#[path = "an_aimed_ray_over_boxed_door_bodies.rs"]
mod an_aimed_ray_over_boxed_door_bodies;
#[path = "join_lever_is_the_joined_edge.rs"]
mod join_lever_is_the_joined_edge;
#[path = "review_4737_span_probes.rs"]
mod review_4737_span_probes;
#[path = "side_reader_aims_ask_nothing.rs"]
mod side_reader_aims_ask_nothing;
#[path = "sphere_aims_reader_ties.rs"]
mod sphere_aims_reader_ties;

#[path = "fallback_rows_review.rs"]
mod fallback_rows_review;

#[path = "a_box_along_a_fillets_line_arc_junction.rs"]
mod a_box_along_a_fillets_line_arc_junction;
#[path = "a_cone_apex_meets_a_vertex.rs"]
mod a_cone_apex_meets_a_vertex;
#[path = "a_cone_apex_on_a_plane_through_it.rs"]
mod a_cone_apex_on_a_plane_through_it;
#[path = "a_cone_apex_on_an_edge.rs"]
mod a_cone_apex_on_an_edge;
#[path = "a_cylinder_resting_on_a_box_top_grazes_along_a_line.rs"]
mod a_cylinder_resting_on_a_box_top_grazes_along_a_line;
#[path = "a_join_on_one_description_records_nothing.rs"]
mod a_join_on_one_description_records_nothing;
#[path = "a_plane_near_a_needle_cones_apex.rs"]
mod a_plane_near_a_needle_cones_apex;
#[path = "a_saddle_cut_by_its_tangent_plane.rs"]
mod a_saddle_cut_by_its_tangent_plane;
#[path = "a_tangent_seam_on_a_spline_wall_ends_as_its_incidence_read.rs"]
mod a_tangent_seam_on_a_spline_wall_ends_as_its_incidence_read;
#[path = "fallback_rows_review_head.rs"]
mod fallback_rows_review_head;
#[path = "review_5956_delta_needle.rs"]
mod review_5956_delta_needle;
#[path = "review_5993_probe.rs"]
mod review_5993_probe;
#[path = "review_6002_needle_join.rs"]
mod review_6002_needle_join;
#[path = "review_6020_seam_probes.rs"]
mod review_6020_seam_probes;
#[path = "review_r1_roots_split.rs"]
mod review_r1_roots_split;
#[path = "the_football_revolves.rs"]
mod the_football_revolves;

#[path = "a_ball_with_no_room_at_its_rim_refuses_typed.rs"]
mod a_ball_with_no_room_at_its_rim_refuses_typed;
#[path = "a_loops_lift_runs_through_its_poles.rs"]
mod a_loops_lift_runs_through_its_poles;
#[path = "a_partner_across_a_lofted_wall_is_joined_on_its_section_value.rs"]
mod a_partner_across_a_lofted_wall_is_joined_on_its_section_value;
#[path = "a_peg_in_a_two_segment_bore_moves_only_its_wall.rs"]
mod a_peg_in_a_two_segment_bore_moves_only_its_wall;
#[path = "a_station_frame_is_minted_to_its_floor.rs"]
mod a_station_frame_is_minted_to_its_floor;
// Review probe for PR 7264, not for merge: spells the slow set so CI runs it.
// a_370_degree_flat_ribbon_reads_violated a_ball_across_a_cylinder_wall_centre_inside a_ball_across_a_cylinder_wall_centre_on_the_wall a_ball_across_a_cylinder_wall_centre_outside a_ball_across_a_cylinder_wall_in_two_loops a_ball_across_a_cylinder_wall_in_two_loops_turned a_ball_across_a_cylinder_wall_r_0_1_across_the_rim a_ball_across_a_cylinder_wall_r_0_2_across_the_rim a_ball_across_a_cylinder_wall_turned_chart a_ball_across_a_cylinder_wall_wider_than_the_wall a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest a_ball_across_two_close_arcs_of_a_lofted_wall_refuses_by_name a_ball_against_a_donut_carries_onto_the_torus_and_builds a_ball_crossed_by_two_partners_builds a_ball_crossing_both_faces_of_a_lens_builds a_ball_escaping_through_a_plane_and_a_sphere a_ball_inside_a_two_sphere_body_builds a_ball_near_tangency_builds_at_the_default_band a_ball_on_the_centre_line_builds_at_every_radius_and_depth a_ball_round_the_donuts_axis_passes_the_germ_mint a_ball_touching_the_outer_equator_off_its_face_builds a_band_pose_under_the_tubes_top_reaches_the_donuts_door a_bar_a_hair_across_the_fills_boundary_crosses_and_one_inside_is_absorbed a_bar_across_a_bore_builds a_bar_an_escalate_width_past_the_arc_is_never_absorbed a_bar_inside_a_concave_bands_fill_is_absorbed_at_every_lean a_bar_inside_a_concave_bands_fill_is_absorbed_at_every_scale a_bar_inside_a_concave_bands_fill_is_absorbed_at_interval a_bar_over_a_sunk_boss_holds_both_wall_halves_in_every_order a_bar_through_a_ball_winds_both_rings a_bar_through_the_cone_wall_refuses_at_every_pose a_big_thin_walled_cylinder_shell_certifies_within_the_default_budget a_blind_bore_and_a_pin_go_with_the_band_at_every_scale a_blind_bore_and_a_pin_go_with_the_band_at_interval a_blind_bore_from_a_leaning_end_wall_goes_with_the_band a_blind_shaft_unions_on_and_off_the_bores_seam a_block_in_a_plates_notch_passes_until_it_touches a_block_over_a_plates_top_still_refuses a_boolean_against_the_twisted_approx_body_refuses_typed a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound a_boolean_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round a_bore_a_hair_across_the_chord_at_interval_crosses_and_one_inside_goes a_bore_a_hair_across_the_end_curve_of_a_non_right_dihedral_crosses_and_one_inside_goes a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval a_bore_inside_the_planar_bands_sliver_goes_with_it_at_every_lean a_bore_inside_the_planar_bands_sliver_goes_with_it_at_every_scale a_bore_inside_the_planar_bands_sliver_goes_with_it_at_interval a_bore_reaching_a_mates_strip_refuses_at_the_meter a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth a_bore_split_on_its_own_carrier_unions_off_the_seam_at_every_span a_bore_wholly_behind_a_pair_edges_line_clears_its_ring_meter a_bored_brick_splits_at_every_tilt_and_offset a_boss_beside_a_holes_long_edge_at_every_angle a_boss_inside_a_hole_sharing_a_vertex_beside_the_tip a_boss_over_a_hole_by_pose a_boss_over_a_hole_tangent_to_it_turned
// a_boss_over_two_holes_tangent_outside_with_no_vertex_at_either_touch a_boundary_column_intersection_mints_its_iso_image a_boundary_edge_on_the_edges_line_refuses_as_an_overlap a_box_along_a_tangent_wall_beside_a_falling_side a_box_along_an_inflection_ruling a_box_along_the_junction_at_rest a_box_along_the_junction_turned a_box_corner_at_the_apex_holds_the_cones_quadrant a_box_cornered_at_the_apex_off_the_axis_holds_the_cone_in_its_wedge a_box_edge_piercing_a_one_face_torus_builds_its_true_body a_box_edge_through_a_cone_wall_crosses_it_at_the_rings_corners a_box_face_through_a_virtual_apex_builds_every_op a_box_past_both_rims_builds_every_op_at_its_closed_form a_box_round_the_slits_tip_cuts_it a_box_whose_bottom_holds_the_slits_tip_cuts_the_bore a_box_whose_bottom_holds_the_slits_tip_cuts_the_slot a_brick_across_a_lofted_wall_is_joined_on_its_carried_arcs a_brick_beside_a_ball_builds_at_every_pose a_brick_edge_grazing_a_torus_in_its_mouth_builds a_brick_edge_touching_a_torus_inner_equator_in_its_mouth_builds a_brick_inside_a_lofted_walls_face_closes_a_ring_on_the_wall a_brick_whose_plane_touches_the_outer_equator_off_its_face_builds a_bumped_block_is_sound_on_every_face a_cap_cup_straddling_the_band_surface_refuses_at_every_dihedral a_cap_cut_again_through_its_pole_halves a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge a_cap_on_another_plane_across_a_plates_top_refuses a_cap_rim_run_back_along_an_off_dyadic_row_certifies_as_the_row a_carried_region_at_the_certified_scalar_holds_its_volume a_cavity_the_engine_cannot_certify_refuses_uncertified a_circle_across_an_earlier_cut_takes_none_of_its_own a_circle_beside_a_pole_builds a_circle_near_the_pole_meshes_at_a_coarse_delta a_circular_band_meters_a_washer_through_its_ring_round_or_square a_clip_holds_its_face_at_every_pose a_closed_edge_in_a_cylinder_or_cone_face_holds_every_op a_closed_edge_on_a_torus_parallel_holds_every_op a_closed_edge_on_a_walls_wrap_edge_holds_every_op a_coaxial_seam_on_the_torus_is_placed_by_its_arc a_collapse_names_the_move_that_made_it a_comb_across_a_torus_of_one_face_pair_builds_its_true_body a_compound_corner_of_two_arcs_builds a_concave_boss_foot_refuses_a_hovering_wall_in_its_reach a_concave_cavity_meets_a_ball_island_where_the_oracle_says a_cone_against_its_copy_builds_every_op a_cone_frustum_and_its_cavity_hold_at_both_scalars a_cone_wall_clear_of_a_bar_builds_at_every_pose_and_scale a_cone_wall_decides_its_cells a_conical_and_a_toroidal_wall_hold_within_the_default_budget a_continuation_on_a_wall_bounded_by_an_ellipse_builds_every_op a_convex_arc_c_shape_extrude_and_loft_caps_point_out_and_certify a_convex_block_meets_a_ball_void_where_the_oracle_says a_convex_rim_refuses_a_void_in_its_material_and_builds_beside_one_just_clear a_copy_moved_within_or_past_the_band_builds_right_or_refuses a_counterbore_half_with_two_faces_on_each_other_refuses_at_the_point a_crescents_bodies_read_right_and_pinch_at_one_vertex_pair
// a_cross_drilled_plates_top_rim_fillets_with_the_bore_on_its_mates a_crossing_at_a_shallow_angle_builds_or_refuses_typed a_cube_on_its_corner_inside_a_plates_top_refuses a_cubic_skin_with_six_offgrid_knots_certifies a_cup_leaning_across_the_fillets_disk_crosses_and_the_chamfer_takes_it a_cup_reaching_into_the_cusp_at_60_degrees_refuses_and_a_shallow_one_goes a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside a_cups_floor_a_hair_across_the_band_surface_crosses_and_one_inside_goes a_cups_head_a_hair_across_the_band_surface_crosses_and_one_inside_goes a_cups_mouth_a_hair_across_the_sliver_crosses_and_one_inside_goes a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes a_cups_mouth_a_hair_from_the_edge_and_an_end_wall_goes_and_one_across_the_end_refuses a_curl_past_a_full_turn_builds_a_spine_that_revisits_itself a_curved_approx_walled_body_validates_at_tier_three a_curved_body_against_its_copy_in_another_chart_is_one_body a_curved_keel_closed_at_both_ends_refuses_as_a_construction_gap a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur a_curved_mate_with_a_ring_refuses_naming_it a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold a_cusp_loft_passes_with_its_nurbs_seam_unjudged_by_kind a_cut_along_two_rulings_through_a_wrap_tipped_apex_refuses_the_pinch_by_name a_cut_clear_of_the_face_splits_in_every_pose a_cut_clear_of_the_faces_window_splits a_cut_ends_at_the_nearest_of_several_boundary_hits a_cut_into_the_faces_window_refuses_at_the_gate a_cylinder_cut_parallel_to_its_tilted_axis_answers a_cylinder_cylinder_tee_builds_and_its_injectivity_holds_where_two_arcs_of_its_ellipse_meet a_cylinder_through_a_hole_tangent_to_its_wall_beside_other_holes a_cylinder_through_a_hole_tangent_to_its_wall_by_pose a_cylinder_through_a_hole_tangent_to_its_wall_turned a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order a_d_prism_beside_a_cylinder_builds_under_every_boolean a_d_turned_about_its_carriers_tangent_off_the_arc_builds a_degree_one_loft_that_changes_scale_is_its_frusta a_degree_one_path_sweep_is_one_wall_per_slab a_degree_two_widening_measures_against_the_oracle a_degree_two_widening_tessellates_against_the_oracle a_diagonal_bar_sunk_into_a_cylinder_cap_answers_its_closed_form a_dimple_and_a_boss_go_with_the_band_at_interval a_dimple_and_a_boss_go_with_the_band_on_either_support_at_every_scale a_disc_is_sound a_dome_and_a_lantern_against_their_copies_are_one_body a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared a_double_split_re_mints_every_piece a_drum_against_a_moved_donut_passes_the_germ_mint a_drum_cavitys_cap_disc_on_two_half_circles_holds a_feature_across_a_requested_edges_line_is_clear_at_the_certified_scalar a_feature_across_a_requested_edges_line_is_clear_of_its_strip a_feature_entering_the_flush_corners_patch_refuses_and_one_clear_of_it_builds
// a_feature_entering_the_flush_corners_patch_refuses_and_one_clear_of_it_builds_at_interval a_feature_entering_the_patch_at_a_non_right_plan_angle_refuses a_feature_in_what_the_patch_leaves_as_it_was_refuses_as_a_bound a_fillet_foot_line_carries_the_bands_axis a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar a_filleted_corners_ball_holds_against_a_face_at_its_pole a_filleted_drum_certifies_across_its_fillet a_flank_touch_glues_the_half_turn_it_leaves a_flared_frustum_split_through_a_ruling_answers a_flipped_chorded_sphere_face_is_refused_as_a_lamina a_folded_oblique_cut_reads_violated_off_its_corners a_folded_torus_cut_reads_violated_off_its_spiric_edges a_football_revolved_part_way_is_its_lune a_fork_inside_a_lofted_walls_face_closes_two_rings_on_the_wall a_fork_inside_and_across_the_rim_of_a_lofted_wall_refuses_its_re_homing_by_name a_four_and_a_two_crossing_pair_at_the_notchs_corner_build_every_op a_four_arc_barrel_fillets_whole a_four_germ_vertex_pair_on_the_cubes_edge_builds_every_op a_frustum_about_a_tilted_axis_split_through_a_ruling_answers a_frustum_split_through_a_ruling_answers a_full_torus_certifies_by_kind a_full_turn_helical_sweep_builds_and_certifies a_full_turn_helical_sweeps_walls_face_out a_general_pose_refuses_at_its_frame_naming_its_loops a_half_cone_agrees_with_the_closed_form a_half_space_flush_with_a_frustum_wedge_cap_builds_the_union a_half_space_through_the_tip_cuts_each_tipped_body a_half_turn_helical_sweep_builds_and_certifies a_half_turn_helical_sweeps_walls_face_out a_head_crossing_between_its_boundary_samples_refuses_as_a_bound a_hemisphere_base_and_a_brick_bottom_are_cut_alike_in_either_order a_hole_at_a_junctions_section_clears_its_trims_until_its_margin_turns a_hole_beside_a_corner_reaching_a_links_strip_refuses_at_its_depth a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent a_horns_cusp_revolved_part_way_is_its_lune a_keel_along_a_cylinder_floors_ruling_refuses_only_its_slit a_keel_on_a_cylinders_ruling_through_its_face_refuses_at_tier_3_prime a_keel_ring_nests_inside_an_elliptic_section a_keel_ring_on_an_island_in_a_hole_nests_on_the_island a_keel_with_a_wall_near_tangent_to_the_bore_refuses_typed a_kept_rim_leaving_along_the_dropped_faces_boundary_cuts_where_it_bends a_kernel_built_oblique_cut_holds_across_its_curved_corners a_keyed_cylinder_from_subtract_splits_into_counter_clockwise_sections a_large_ball_near_external_tangency_is_pinned a_lattice_drawn_pockets_mouth_off_the_origin_fillets_whole a_lattice_drawn_rounded_plate_fillets_whole a_lattice_on_the_tube_reads_the_truth a_line_kiss_beside_a_tangent_site_records_its_contact a_line_piece_the_closed_form_crosses_at_its_quarter_point_takes_its_row a_loft_whose_sections_disagree_passes_with_the_seam_unjudged_by_kind a_loft_with_holes_is_the_same_under_every_spelling_and_its_holes_do_not_move_v a_lofted_arc_wall_is_a_function_of_the_sections_bulge a_lofted_lens_blade_certifies_its_sign_and_its_number
// a_lofted_one_segment_section_is_one_spline_wall_with_a_wrap_strut a_lofted_one_segment_wall_measures_as_the_two_arc_wall a_lofted_operand_reaches_the_fallbacks_re_gate a_lofted_operand_refuses_the_union_check_typed a_long_helical_sweep_certifies_within_the_default_budget a_mixed_corner_at_a_non_right_plan_angle_builds_at_its_closed_form a_moved_body_measures_as_its_source_does a_moved_closed_walls_wrap_edge_is_a_row_of_its_fit a_moved_curved_fitted_cap_builds_where_its_fit_certifies a_moved_fitted_cap_stands_its_corners_on_the_held_sides a_moved_plane_meets_a_fitted_cap_along_their_certified_section a_moved_planes_section_with_a_held_fit_follows_the_plane a_multi_solid_certificate_reads_each_face_once a_multi_solid_certificate_split_across_rounds_continues_to_the_measurement a_near_square_frustum_minus_a_box_builds a_near_square_plane_cuts_a_cylinder_in_its_exact_ellipse a_near_square_plane_cuts_a_frustum_in_its_exact_ellipse a_near_square_rod_minus_a_box_builds a_near_square_split_of_a_frustum_validates_its_halves a_nearest_tie_in_band_on_a_bar_across_the_hole_escalates_by_name a_needle_cone_cut_off_its_apex_lands_its_closed_forms a_neighbour_hole_clears_the_mouths_trim_until_its_margin_turns a_notched_block_is_sound_on_every_face a_nurbs_graze_behind_crossings_is_refused_on_every_op a_one_face_torus_cut_certifies_its_caps_by_the_wall_alone a_one_segment_strut_runs_its_column_back_on_the_negated_knots a_partial_revolve_on_a_tilted_plane_stores_its_frames_exactly a_partly_shared_or_scaled_curved_operand_builds_right_or_refuses a_pebble_buried_in_the_collar_beside_the_mate_is_its_own_shell a_peg_in_a_pocket_tangent_to_its_rim_builds_its_unions_and_intersections a_piece_the_closed_form_crosses_at_its_quarter_point_takes_the_rows_arc a_pin_from_a_leaning_end_wall_goes_with_the_fill a_pin_through_the_base_and_a_coaxial_pin_build_their_closed_forms a_pinched_operands_pierces_of_one_point_mint_one_ring a_pinchs_unions_from_every_root_of_the_ring a_pip_with_its_seam_in_the_cubes_top_builds_at_its_closed_form a_placement_in_t_stays_a_function_of_its_parameter a_planar_arc_spine_lofts_past_pi a_plane_close_by_the_wrap_edges_end a_plane_cutting_the_blades_loft_clears a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order a_plane_holding_the_axis_cuts_each_revolved_cone_at_its_closed_form a_plane_holding_the_axis_cuts_each_tipped_cone_at_its_closed_form a_plane_meeting_a_tipped_cone_at_its_apex_alone_keeps_it_whole a_plane_missing_a_spline_body_returns_it_whole a_plane_near_the_inner_equator_builds_its_true_bodies a_plane_off_a_hole_wall_reads_each_decision_across_the_band a_plane_tangent_to_a_holes_rim_at_a_vertex_splits_sound a_plane_through_a_30_degree_cones_apex_cuts_its_closed_forms a_plane_through_a_45_degree_cones_apex_cuts_its_closed_forms a_plane_through_a_60_degree_cones_apex_cuts_its_closed_forms a_plane_through_the_pole_cuts_each_hemisphere_at_its_cap a_plane_touching_a_bore_rim_splits_alike_under_either_normal
// a_plate_with_a_round_hole_drops_the_cells_in_the_hole a_plug_clear_of_a_brick_on_its_planes_reads_alike_at_every_pose a_plug_in_its_hole_reads_alike_at_every_pose a_pocket_whose_wall_crosses_a_side_face_builds_at_its_closed_form_volume a_pockets_mouth_and_a_hole_beside_it_fillet_in_one_call a_point_past_the_prisms_reach_is_outside_and_none_within_it_is_read a_pole_struts_halves_face_their_own_meridians a_pole_to_pole_cut_of_a_ball_wedge_builds a_post_through_the_near_wall_piece_refuses_the_ring a_prism_edge_on_the_tubes_wall_builds_every_op_but_its_parting a_prism_with_a_station_on_its_arc_rims_through_a_slab_builds_every_op a_prism_with_arc_walls_through_a_slab_builds_every_op a_proud_tilted_plug_unites_to_the_slab_and_its_stub a_puck_on_a_torus_parallel_holds_every_op a_quarter_annulus_revolved_either_way_is_sound a_quarter_through_the_tip_cuts_each_tipped_body a_radial_hole_through_the_tube_builds_at_its_volume a_rational_quarter_torus_answers_a_bracket_at_the_wall_not_a_refusal a_rational_section_on_a_curved_path_faces_out_along_the_turn a_rational_section_on_a_curved_path_meters_at_the_span_meter a_ray_beside_a_door_slabs_moving_silhouette_edge_reads_no_wrong_side a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk a_ray_given_further_along_its_line_reads_a_curved_body_the_same a_ray_shallow_to_a_face_past_its_curved_edge_reads_right a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line a_reflex_union_at_a_tangent_site_never_ships_the_overlap_twice a_refusing_arm_returns_no_properties_through_either_door a_region_bounded_off_one_plane_is_carried_about_its_own_point a_region_near_its_shells_point_is_read_about_it_bit_for_bit a_regions_carry_moves_the_volume_by_what_it_says_with_its_sign a_relation_refines_a_quadrature_face_and_settles_or_stops_in_the_band a_relation_run_out_at_the_wall_is_the_refusal_carrying_the_bracket a_ridge_on_a_cylinders_ruling_resting_on_its_floor_passes_tier_3_prime a_right_angle_stub_builds_every_operation_at_its_closed_form a_rim_across_a_walls_ellipse_builds_every_op_in_both_orders a_rim_arc_beside_a_bent_spline_wall_is_taken_on_the_cylinder a_rim_crossing_builds_in_every_op a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit a_rim_in_the_band_takes_its_general_image_at_interval a_rim_inside_a_ball_face a_ring_beside_an_outer_loop_on_the_run_is_read_from_an_edge_midpoint a_ring_inside_a_sphere_island_moves_and_builds a_ringed_wall_holding_the_pierce_builds_either_operand_first_and_pocketed_last a_rod_across_the_inner_bound_builds_at_its_volume a_rod_across_the_outer_equator_builds_at_its_volume a_rod_across_the_tube_builds_at_its_volume a_rod_against_a_bore_builds_every_operation a_rod_cut_obliquely_and_glued_back_is_the_rod a_rod_glued_in_its_bore_answers_alike_in_either_order a_rod_inside_the_tubes_shadow_builds_at_its_volume a_rod_less_a_tilted_slab_holds_across_its_curved_corners a_rod_off_the_axis_line_builds_at_its_volume
// a_rod_rim_on_the_dome_across_its_seam_meridian_builds_every_op_undeclared a_rod_standing_on_a_step_builds_every_op a_rod_taller_than_a_turn_less_a_tilted_slab_holds a_rod_through_a_ball_pairs_both_loops_alike a_rod_through_a_drum_wall_builds_at_its_sliced_volume a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door a_rod_whose_wall_touches_the_outer_equator_past_its_end_builds a_root_two_half_walls_share_is_counted_once a_round_keel_in_the_plane_reads_the_side_it_is_read_from a_round_tube_certifies_its_sign_while_its_number_is_open a_rounded_plate_chamfers_all_round_at_pappuss_volume a_rounded_plate_chamfers_all_round_at_pappuss_volume_at_interval a_rounded_plate_filleted_all_round_removes_pappuss_volume_at_interval a_rounded_plate_with_a_round_hole_fillets_every_edge_in_one_call a_rounded_plate_with_a_rounded_through_hole_chamfers_every_edge a_rounded_pockets_floor_and_mouth_chamfer_whole a_ruling_across_an_ellipse_builds_every_op_but_its_slit a_saddle_cut_by_its_tangent_plane_refuses_at_its_node_arcs_under_every_op a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap a_saddle_wall_fits_at_the_runs_eps_or_stops_at_the_cap a_seam_builds_from_twice_root_band_off_a_node a_seam_split_a_hair_off_tangency_reads_no_side_inside_out a_section_circle_through_the_pole_at_a_notch_tip_is_cut_or_refused a_section_circle_touched_by_a_notch_tip_is_cut_or_refused a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_split_and_its_lever_builds a_section_near_a_pole_meshes_at_every_delta a_section_touching_a_rim_splits_at_the_closed_form a_setback_beside_the_arcs_radius_builds_below_it_and_refuses_past_it a_shaft_a_quarter_turn_off_the_bores_seam_unions a_shaft_a_tenth_of_eps_under_its_bore_mates_in_every_op a_shaft_off_the_bores_seam_unions a_shaft_on_the_bores_seam_unions a_sharp_plate_offset_over_a_rounded_one_unions_in_either_order a_sheared_fillets_foot_lines_certify_with_headroom a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim a_shell_joined_along_a_long_face_reads_alike_whichever_face_it_stores_first a_shell_whose_regions_all_lie_near_its_point_reads_as_about_one_point_bit_for_bit a_shell_whose_tori_meet_along_circles_keeps_each_torus_a_region a_shells_pad_reads_alike_whichever_quadrature_face_it_stores_first a_short_odd_or_pair_edge_refuses_at_its_meter a_sided_result_is_sided_again_against_a_third_rod a_single_cap_just_past_the_band_builds_as_the_parting_leaves_it a_skinned_base_approx_face_earns_the_germ_pair_refusal a_slab_across_a_round_tube_on_a_plate_unions_in_every_order a_slab_across_a_torus_of_one_face_pair_builds_its_true_body a_slab_cutting_a_cap_off_a_pole_strut_carve_builds a_slab_face_nearly_along_a_ruling_builds a_slab_round_the_prism_closes_a_ring_on_each_of_its_big_faces a_slab_tilted_about_x_through_the_tip_cuts_both_blocks a_slab_topped_through_the_slits_tip_on_the_bore_cuts_it
// a_slab_with_two_holes_is_sound_on_every_face a_small_box_whose_side_holds_the_wrap_seam_builds_every_op a_sphere_octant_touching_the_pole_is_sound a_sphere_off_a_circular_band_s_axis_meters_alike_at_both_scalars a_spine_curled_past_pi_still_faces_out_everywhere a_spiric_bounded_face_answers_off_its_spiric_edge a_spline_capped_box_moves_every_chart_onto_its_closed_form a_spline_loft_mirrors_to_its_image a_spline_witness_by_an_edge_is_placed_on_the_widened_band a_split_carries_an_attached_fitted_row_to_each_child a_split_carries_an_attached_fitted_row_to_each_child_at_the_interval_scalar a_split_carries_the_general_row_to_each_child a_split_carries_the_general_row_to_each_child_at_the_interval_scalar a_split_on_a_spline_chart_carries_the_parents_image a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round a_split_through_a_seam_vertex_returns_no_side_tier_three_refuses a_split_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round a_split_through_the_keel_caps_around_the_trench a_split_through_the_seam_builds_as_the_two_arc_form_does a_split_through_the_slits_tip_on_the_bore_builds a_spool_is_sound_and_its_torus_decides_its_cells a_spun_snowman_builds_under_every_boolean a_spun_tilted_plug_in_its_hole_reads_as_plugged a_square_and_a_round_dimple_leaning_off_their_supports_normal_go a_square_bar_inside_a_concave_bands_fill_is_absorbed a_stem_window_reaching_over_a_loop_does_not_glue a_stepped_rings_floor_holds_against_its_end_caps a_strut_beside_its_pairs_fan_builds_every_op a_swaying_loft_builds_closed a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order a_tangent_sites_controls_build_sound a_three_arc_ring_is_metered_arc_by_arc_in_every_subtraction_order a_tight_bend_tube_sweeps_from_any_start a_tilted_bar_inside_at_both_end_walls_through_the_band_surface_refuses a_tilted_cap_builds_its_true_body_wherever_its_origin_is_stored a_tilted_caps_marched_rim_refuses_at_its_certificate a_tilted_circle_elbow_just_clear_of_its_tangent_builds a_tilted_cut_of_a_ball_is_an_operand a_tilted_d_whose_arc_rows_step_back_builds a_tilted_plane_across_a_one_segment_cylinder a_tilted_plane_through_the_apex_cuts_each_revolved_cone_at_its_closed_form a_tilted_plane_through_the_apex_cuts_each_tipped_cone_at_its_closed_form a_tilted_plug_in_its_hole_reads_alike_at_every_pose a_tilted_rod_through_a_rim_builds_at_its_sliced_volume a_tilted_slab_against_the_lens_builds_off_the_seam a_tilted_tube_builds_at_its_closed_form a_tipped_rod_refuses_alike_in_both_operand_orders a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings a_torsion_bearing_path_sweep_faces_out_along_the_twist a_torus_crossing_the_rim_reaches_the_join a_torus_cut_off_its_axis_holds_across_its_spiric_caps a_torus_just_clear_of_the_rim_is_decided_by_its_roots a_torus_operand_passes_the_pair_gate_and_refuses_at_the_crossing_layer a_transversal_pose_builds_in_both_poses a_trimmed_face_cut_beside_its_pole_builds
// a_tube_through_a_ball_across_its_wrap_edge_builds_every_op a_tube_through_a_balls_centre_closes_both_sections_at_its_wrap_edge a_tunnel_apex_crossing_a_pocket_floors_arcs_parts_in_every_op a_turn_a_microradian_short_of_square_builds_at_its_sliced_volume a_turned_and_moved_mixed_corner_reads_as_it_does_in_place a_turned_l_pockets_top_mouth_and_floor_chamfer_in_one_call a_turned_near_square_rod_minus_a_box_builds a_turned_near_square_split_of_a_rod_validates_its_halves a_turned_rim_on_a_wall_bounded_by_an_ellipse_builds_every_op_undeclared a_turned_tube_reads_its_side_in_its_seam_plane a_twisted_loft_holds_along_its_seams_at_both_scalars a_two_ball_union_holds_with_its_poles_read a_two_turn_helical_sweeps_walls_face_out a_vertex_on_a_curved_face_is_confirmed_by_its_trim a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate a_void_dilated_through_the_curved_wall_refuses_with_a_witness a_void_in_the_overruns_volume_refuses a_void_read_off_its_volume_is_refined_past_round_zero a_wall_holding_one_balls_ring_is_divided_by_the_others_chord a_wall_ringed_by_ellipse_arcs_measures a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order a_wall_through_a_box_edge_builds_every_op_but_the_ring a_wedge_of_the_cone_cut_through_its_apex_holds_its_closed_form a_whole_body_sense_inversion_refuses_at_every_planar_cap_arcs_included a_whole_rim_re_mints_the_closed_form a_windowed_tubes_caps_hold_against_its_walls_at_both_scalars adversarial_outlines_build_where_their_bands_fit_and_refuse_where_they_overlap an_80_degree_cones_top_against_a_box_clear_of_its_rim_builds an_acute_strip_cup_whose_wall_crosses_the_kept_disk_refuses_as_a_bound an_analytic_oracle_sweep_over_the_donut an_annulus_rim_around_a_mixed_rim_refuses_and_composes_sequentially an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose an_approx_faced_body_moves_under_a_rigid_map an_approx_faced_body_validates_at_tier_three an_arc_resting_in_a_cylinder_or_sphere_face_refuses_only_its_slit an_arc_spine_past_a_full_turn_reads_violated an_arc_spine_short_of_a_full_turn_holds an_arc_wall_whose_radial_turns_with_a_boxed_bulge_stores_every_members_direction an_arch_started_inside_the_stem_glues_to_the_same_volume an_edge_across_the_apex_holds_half_the_cones_top an_edge_along_the_axis_holds_a_quarter_of_the_cones_top an_edge_in_the_partners_face_plane_records_its_rests_where_the_result_keeps_them an_edge_lying_in_a_wall_away_from_its_midpoint_parts_in_every_op an_edge_near_a_node_crosses_through_the_bisector_planes an_edge_through_the_apex_builds_box_first an_eight_crossing_pair_at_a_shared_vertex_builds_in_both_orders an_elbow_builds_from_every_spelling an_elbow_whose_planes_meet_outside_the_circle_builds_from_any_start an_engraved_one_arc_c_builds_at_every_sweep an_hourglass_touches_at_the_apex_alone an_identity_start_placement_is_not_the_normal_plane_of_the_s_path an_inside_out_thin_tube_refuses_at_rest an_interior_arc_refined_by_its_neighbours_lofts
// an_interior_column_intersection_mints_a_general_image an_island_entering_the_inset_corners_patch_refuses_and_one_clear_of_it_builds an_island_entering_the_inset_patch_at_a_non_right_plan_angle_refuses an_island_holding_the_far_caps_pole_winds an_l_plates_rounded_corner_run_fillets_whole_at_interval an_l_void_crossing_the_arc_crosses_in_every_order_and_pose an_oblique_cavity_s_corners_meet_a_ball_island_where_the_oracle_says an_oblique_cylinder_section_decides_beside_its_ellipse an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms an_oblique_seam_near_a_node_builds an_off_centre_split_re_mints_the_parents_row an_offset_cap_keeps_its_verdict_under_a_move an_open_arc_link_s_reach_is_its_whole_turn an_outline_through_a_disc_of_a_few_arcs_fillets_at_pappus an_s_run_of_a_convex_and_a_reflex_arc_builds an_uncertain_root_count_sets_its_ray_aside an_undecided_sign_with_the_schedule_run_out_refuses an_upright_cone_splits_at_every_pose_through_its_apex_faces an_x_bar_sunk_into_either_cap_answers_its_closed_form arc_spine_walls_with_knots_ulps_off_their_splits_hold_against_themselves arc_split_bore_intersect_and_differences_are_the_closed_form arc_split_bore_unions_are_the_closed_form arcs_flanking_a_short_flat_fillet_whole arcs_of_a_half_turn_or_more_build at_1e_12_the_slanted_bores_refuse_every_cell_and_the_glued_rod_escalates_in_both_orders bodies_built_before_the_session_trip_the_guard_while_disp_still_reads_zero both_halves_of_a_split_rim_re_mint_the_parents_row both_witnesses_build_under_every_boolean box_voids_in_the_bands_corner_read_true cap_cups_go_at_acute_and_obtuse_dihedrals coaxial_poses_build_every_op_at_their_volumes cones_sharing_a_carrier_at_different_extents_build crossing_rods_at_rest_spins_hold_a_bounded_enclosure crossing_rods_build_past_the_ellipse_edge_copy_at_every_spin crossing_washers_with_radii_in_band_are_no_rim cups_beside_a_bore_under_two_bands_and_by_a_mitre_go cups_on_both_supports_beside_a_cap_cup_and_beside_a_convex_band_go curved_path_sweep_body_builds_validates_and_brackets_pappus curved_walls_are_violated_just_above_their_separation cut_loft_refuses_typed_naming_the_missing_boolean_layer d2_no_input_reaches_a_panic d2_reached_variants declared_equal_walls_meeting_at_a_sliver_are_the_bisector_ellipses deviation_1_both_doors_compose_and_agree each_analytic_kind_joins_back_exactly_across_a_grid_of_splits each_poses_body_is_its_closed_form_in_every_op edge_edge_sites_at_a_reflex_wedge_build_sound edges_in_a_face_run_into_a_walls_foot_build_every_op eight_crossing_corners_nest_two_deep_and_build_every_op ellipse_rims_tangent_inside_build_every_op ellipse_rims_tangent_outside_build_every_op equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms equal_wider_and_nested_rods_join_along_their_rulings every_arc_capped_kind_reads_its_truth_at_every_pose every_circle_edge_of_the_public_door_corpus_answers_against_the_oracle every_circle_loft_the_row_measured_meshes
// every_corner_patch_encodes_its_side_and_holds_no_pole_off_a_vertex every_crossing_of_a_ray_about_a_curved_edge_counts_right every_crossing_of_a_ray_about_the_corpuss_edges_counts_right every_cut_clear_of_a_partial_turn_sphere_face_splits every_cut_through_a_bore_reads_its_truth_at_every_pose every_door_reads_a_carried_region_about_its_own_point every_op_builds_at_its_closed_form every_op_far_off_and_turned_keeps_its_pads_at_home every_op_far_off_and_turned_measures_its_volume_at_home every_op_of_a_spline_operand_refuses_typed_or_builds_right every_op_on_a_cone_wall_answers_its_truth_or_refuses_typed every_point_of_the_spiric_box_on_the_one_face_torus_reads_its_side every_pose_of_the_family_builds_at_its_closed_form every_reachable_crossing_pose_refuses_typed_or_answers_correctly every_segment_joins_neighbouring_sites_along_its_loop every_spacing_lofts_at_degree_two_and_three every_sphere_face_flip_is_refused_and_named every_sphere_offer_passes_just_below_it every_spiric_face_meets_its_target_at_the_round_it_does_at_home every_spiric_face_reads_near_and_far_against_a_dense_boundary every_square_loft_the_row_measured_meshes every_steep_pose_of_the_cylinder_is_one_section_face_per_half every_suite_file_is_aggregated every_surface_class_keeps_its_verdict_under_a_move every_tilted_cut_wall_reads_its_truth every_wedge_through_a_full_turn_collar_builds_under_every_boolean extruded_arc_runs_build_one_wall_each extruded_runs_build_one_wall_each faces_of_one_carrier_that_only_abut_are_no_interference far_tilted_cut_discs_are_read_by_their_exact_volume features_about_the_arc_build_clear_and_refuse_where_they_straddle features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say full_turn_bore_intersect_and_differences_are_the_closed_form full_turn_bore_unions_are_the_closed_form fully_revolved_arc_runs_build_one_wall_each generic_hexahedra_build_and_fill_at_the_oracle h_t_c_intersect_in_every_nesting halves_off_by_a_few_eps_are_decided_past_the_band_only holes_at_an_arcs_wedge_build_clear_and_refuse_overlapping holes_at_an_arcs_wedge_build_clear_and_refuse_overlapping_under_a_chamfer holes_beside_a_corner_build_in_one_call_as_in_sequence in_face_rings_pair_along_the_wall_whatever_the_arc_and_the_gap intersect_and_differences_answer_the_closed_form interval_the_klein_elbow_hollows_to_its_measured_volume ladder_and_seam_split_rims_chamfer_at_their_frustum_rings leaning_prisms_build_at_the_oracle_over_leans_radii_and_scales leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales lilys_cap_pairs_and_the_blades_cuts_are_proved_loop_free lilys_four_cap_pairs_clear loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below lofted_continuation_walls_carry_one_key_per_segment many_stacked_sections_loft_at_interval near_misses_answer_by_the_closed_form_distance near_parallel_walls_cost_the_same_pairs_turned_45_degrees near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed nested_inside_and_clear_poses_build_their_closed_forms
// no_aim_of_a_boxed_door_body_reads_in no_ray_about_a_boxed_door_body_reads_in no_spline_or_spiric_operand_builds_wrong one_edge_annulus_rims_chamfer_at_their_frustum_rings_at_the_certified_scalar one_face_walls_read_their_axes_as_an_angle one_solid_in_two_face_orders_carries_nearly_one_pad open_mixed_chains_chamfer_whole_at_pappuss_volume_at_interval other_spins_and_steeper_tilts_refuse_at_their_germs overlaps_are_never_read_clear_and_abutments_never_interfere p10_the_sliver_family p11_the_new_rows_in_the_other_order p12_a_plane_through_the_middle_of_a_diamond_holes_edge p15_a_bore_inside_a_disc_staggered p1_a_bore_inside_a_tall_disc_touching_its_wall p1_wall1_glues_the_stem_into_one_body p2_a_bore_inside_a_bigger_bore p3_a_nested_touch_just_off_tangent p6_the_nested_aligned_touch_by_membership p8_a_long_thin_disc_nested_in_a_fat_one p9_a_rod_seated_in_a_plates_bore parallel_cylinders_that_pierce_build_at_the_closed_form pins_stacked_end_to_end_reach_no_witness_search planes_nearly_along_the_axis_through_a_seam_vertex_split planes_parallel_to_a_circular_band_meter_alike_at_both_scalars planes_through_a_seam_cap_corner_chord_inside_each_wall_face planes_through_the_axis_nearly_along_it_split planted_subtraction_results_break_the_volume_relations plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op probe_e_reversed_chart_takes_the_backward_candidate probe_i_door_a_full_tool probe_random_charts_never_break_the_certified_arms props_meets_an_independent_quadrature_on_every_band r1_a_cone_on_a_cylinder r1_a_frustum_through_its_virtual_apex r1_apex_down r1_cap_rim_measured_map_certifies r1_e2e_gauge_is_silent_on_an_honest_body_through_the_public_door r1_general_image_is_operand_order_blind r1_general_image_is_the_measured_column_among_many r1_oracle_sweep_prints_its_own_numbers r1_other_half_angles r1_partial_windows r1_skinned_loft_offset_certificate_contains_dense_sample r1_wall_seam_arm_mints_and_certifies_the_interior_column r2_a_spine_past_a_full_turn_builds_a_self_overlapping_body r2_cone_on_a_cylinder r2_cones_apex_down r2_cones_at_10_and_80_degrees r2_every_same_side_touch_pose_answers_as_a_touch r2_frustum_through_its_virtual_apex r2_oracle_sweep_over_three_cone_bodies r2_partial_cones r2_partial_over_a_half_turn_never_panics r2_pit_secants r2_probes r2_random_zprism_pairs r3_seeded_random_apex_poses_never_panic_or_mislead random_copied_carriers_extrude_at_interval_without_contradiction random_queries_read_the_truth random_rods_through_the_tube_never_answer_a_wrong_body rays_grazing_a_torus_rim_count_no_crossing_of_the_tube_elsewhere rectangular_holes_ending_at_a_corners_window_build_at_pappus reflex_corner_edge_in_face_poses_build_every_op_undeclared review_4781_kiss_with_overlap review_4781_near_tangent_box_on_seam review_4781_near_tangent_cylinders review_4781_touch_pairs_every_op review_6020_boolean_seam_poses review_6020_seam_poses review_curved review_curved_plate review_lengths review_reflex review_tilted
// revolved_runs_build_one_wall_each rims_crossing_on_tangent_walls_build_every_op ring2_r1_end_to_end_certificate_bits ring_clearance_trio_definite_pass_definite_refuse_in_band_escalate rods_across_the_wall_join_along_their_rulings rods_overlapping_over_part_of_their_length_build rods_sharing_a_wall_at_different_lengths_are_sided_by_their_extent rounded_continuations_inside_a_wall_build_every_op rounded_outlines_at_their_limits_chamfer_whole rounded_outlines_drawn_in_arc_pieces_chamfer_whole rounded_outlines_of_other_shapes_chamfer_whole rounded_polygons_fillet_whole seams_lying_on_the_walls_read_the_axes_across_the_region seams_off_the_pinch_build_every_operation_at_the_closed_form shape_iii_volume_matches_the_derived_closed_form shapes_the_carve_does_not_build_refuse_conservatively shell7_dump_corpus shell7_r1_diff_corpus shelling_a_spline_capped_box_refuses_past_the_door shelling_the_curved_loft_refuses_at_a_walls_fit shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row short_arcs_at_a_crescents_tip short_arcs_at_a_turned_tip_at_the_finest_band six_stations_with_offgrid_knots_certify slanted_bores_build_every_operation_one_difference_pinched_to_itself sphere_pairs_tilted_against_both_charts_build_under_every_boolean split_lofts_hold_their_volumes_at_interval steep_cuts_of_the_cylinder_at_many_seam_turns_chord_inside_each_wall_face steep_cuts_of_tubes_chord_inside_each_bore_face steep_ellipse_poses_build_sound steep_tube_neighbours_of_the_check_5_pose_answer_and_are_right strip_cups_go_at_acute_and_obtuse_dihedrals subtract_and_intersect_reach_the_doors_union_does sym11_the_far_placement_is_a_counted_dispute_at_sym_f64 sym11_the_far_placement_never_contradicts_at_sym_interval tangent_charts_shell_onto_their_closed_forms the_annulus_builds_at_every_witness_azimuth_pair the_annulus_builds_at_random_azimuth_pairs the_axis_normal_slab_builds_its_closed_forms the_bodies_at_the_slanted_rims_touch_read_right_point_by_point the_boolean_gate_refuses_an_approx_operand_by_kind the_box_less_the_block_refuses_the_four_sheets_at_the_keel the_bracket_builds_at_the_witness_sizes_and_up_to_its_real_limit the_bracket_chamfers_with_the_odd_edge_and_one_other_at_every_mixed_corner the_bracket_fillets_all_round_at_its_closed_form the_built_body_is_the_model_point_by_point the_carve_spacings_validate_at_tier_three the_census_reads_the_decision_tables_as_the_types_do the_certified_doors_form_at_the_symbolic_tier_and_answer_the_base_bits the_clamp_floor_clears_the_shell_about_the_top_circle the_collinear_cap_drum_itself_runs_out_of_depth_off_its_half_circle the_composed_die_certifies_and_tessellates_watertight the_composed_die_replays_bit_identically the_concave_skewed_box_fills_the_model the_corpus_pot_holds_at_both_scalars the_curved_lofts_cap_moves_its_corners_along_the_slanted_seams the_cut_cylinder_reads_its_truth the_cut_cylinders_ellipse_face_takes_nothing_away the_die_slab_above_the_ball_builds_at_its_closed_form
// the_donuts_meridian_and_concentric_poses_build_their_true_bodies the_donuts_spiric_poses_build_their_true_bodies the_door_names_the_flipped_face_tier_3_names the_door_names_the_flipped_face_tier_3_names_against_the_rod the_door_names_the_flipped_face_tier_3_names_against_the_stub the_door_names_the_flipped_face_tier_3_names_the_tilted_ball_against_the_turned_rod the_door_names_the_flipped_face_tier_3_names_the_unit_ball_against_the_turned_rod the_edge_face_lane_escalates_no_proxy_on_the_near_tangent_poses the_f64_seam_answers_every_public_door the_family_re_posed_builds_the_same the_fat_corner_bars_lens_is_the_true_body the_fenced_poses_keep_their_own_outcomes the_filed_turn_escalates_its_first_order_at_the_default_band the_filleted_l_plate_is_the_rolling_balls_body the_fold_mints_two_loft_stacking_verdicts_per_slab the_gate_certifies_a_sign_and_the_continuation_certifies_the_number the_gates_certificate_is_the_measurement_and_costs_one_quadrature the_guard_reads_the_walls_with_their_shared_rims_handed_in the_half_turn_helixs_caps_hold_against_its_walls_at_both_scalars the_inflecting_duct_is_one_solid_whatever_the_start_frames_roll the_klein_elbows_cavity_holds_across_its_spiric_caps the_lilys_tori_exhaust_the_doors_cell_budget the_line_meter_is_the_least_of_g_and_never_reads_past_omega the_loft_body_certifies_and_encloses_nine_at_interval the_lune_fillets_place_every_lattice_point the_mate_classes_the_shafts_edges_at_the_rims the_meeting_axes_pose_builds_at_the_certified_scalar the_mint_pass_re_derives_a_split_spline_wall the_mint_rederives_the_carried_rows_and_the_trimmed_lane_refuses_their_sub_range the_mixed_corner_recourse_builds_where_it_says the_model_reads_the_box_corner_by_hand_and_every_witness the_near_node_poses_at_the_witness_band the_node_cap_refuses_one_doubling_short_of_the_tolerance the_notch_against_a_wedge_pinch_builds_every_op the_odd_edge_with_one_other_refuses_the_fillet_and_an_oblique_shared_face the_odd_edge_with_one_pair_edge_chamfers_at_a_non_right_plan_angle the_odd_edge_with_one_pair_edge_chamfers_at_its_closed_form the_odd_plus_one_chamfer_moves_the_material_its_strips_bound the_odd_plus_one_turn_meters_its_pair_strip_from_the_odd_strips_plane the_off_face_touches_build_at_every_scale the_off_face_touches_build_in_a_tilted_frame the_one_face_and_seamed_tori_s_spiric_poses_answer_as_the_donuts the_operand_order_is_not_a_fact_about_the_locus the_oracle_holds_its_closed_forms the_output_stage_merge_keeps_every_slab_wall the_pip_caps_measure_through_the_closed_form the_pipped_cube_fillets_in_place_with_rings_carried the_pips_cut_in_one_group_operation_on_all_six_faces the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar the_probe_family_ships_no_wrong_body the_props_lanes_verdict_is_the_same_either_side_of_the_map the_reach_replays_at_the_interval_scalar the_reflex_site_off_the_pins_poses_never_ships_the_overlap_twice the_remaining_wall_is_per_slab_turn_pi
// the_reorder_leg_builds_where_the_boolean_serves_the_band the_rim_crossing_rods_build_in_every_op the_rims_half_turn_postures_answer_against_the_oracle the_ring_lane_builds_at_the_certified_scalar the_ringed_wall_rows_hold_off_the_origin the_rings_volume_brackets_its_closed_form_at_the_certified_scalar the_rings_volume_brackets_its_closed_form_through_point_maps the_rod_short_of_the_inner_equator_answers_every_op the_s_duct_walls_hold_against_themselves the_saddle_is_a_certified_interior_loop_naming_both_walls the_sealed_island_request_builds_at_every_pose the_seam_and_the_open_bands_export_one_public_item_and_widen_nothing_else the_seam_split_census_reads_no_side_inside_out the_session_receipt_does_not_move_with_the_thread_count the_shelled_lofts_wall_seams_derive_where_its_fits_certify the_short_and_close_witnesses_build_at_their_closed_forms the_short_and_close_witnesses_build_at_their_closed_forms_at_interval the_shrunk_faces_keep_their_rings_and_senses the_side_readers_reach_bounds_a_section_arc_not_its_ellipse the_sign_walk_takes_every_open_solid_past_the_numbers_resolution the_sign_walks_narrower_bracket_holds_the_closed_form the_sphere_capped_tube_builds_with_its_hemisphere_turned the_spiric_body_exports_one_spline_per_rim_with_its_bound_stated the_start_vertex_and_the_authored_winding_change_nothing the_steeper_tilts_union_stops_by_identity the_straddle_set_builds_and_the_sampling_wall_refuses the_support_at_depth_never_separates_a_pair_closer_than_the_bound the_surgery_front_door_refuses_its_named_gaps the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms the_sweep_fills_concave_overruns the_sweep_subset_ships_no_bad_body the_tangent_site_keeps_building_pure_contacts the_three_way_reading_replays_at_interval the_tilted_planes_ellipse_builds_its_closed_forms the_tipped_cones_apex_splits_and_cuts_through_the_boolean the_trapezoids_back_face_reads_alike_at_interval the_tube_walls_u_joint_is_tangent_continuous the_verdict_escalates_only_in_a_narrow_band_around_a_coincidence the_verdict_reads_the_probes_closed_forms_over_the_face_angle_sweep the_vessel_holds the_walk_matches_the_serial_golden_at_four_threads the_walk_matches_the_serial_golden_at_one_thread the_walk_reads_its_charts_and_edges_through_their_memos the_walls_a_placed_approx_capped_part_still_meets the_well_at_other_poses_builds_at_the_seam the_witness_brackets_its_closed_forms_at_the_certified_scalar the_witnesses_build_at_every_pose thin_nested_rings_build three_pairs_at_one_vertex_build_on_one_key_and_every_op_is_sound tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion tilted_and_spun_charts_build trimmed_sphere_shells_read_their_side_off_aimed_rays turned_and_moved_rounded_rectangles_chamfer_whole turned_rounded_outlines_fillet_whole two_arc_circles_agree_with_one_segment_ones two_arcs_in_one_run_build two_bars_over_a_sunk_boss_leave_each_wall_piece_one_half_in_every_order two_cones_through_one_apex_build_across_the_pose_families
// two_cones_through_one_apex_cross_along_their_common_rulings two_crossing_bores_build_every_operation_at_the_closed_forms two_cups_and_a_square_pin_go_with_the_band two_cut_ins_on_one_face_build_in_either_order two_donuts_in_general_pose_pass_the_germ_mint two_loops_on_one_face_pair_build_their_true_bodies two_one_site_loops_on_one_face_pair_pass_the_dispatch two_rims_the_band_reads_coplanar_are_one_curve_only_within_it two_slabs_beside_a_short_arm two_slanted_bottoms_crossing_on_one_wall_build_every_op two_slanted_rims_tangent_on_the_ruling_build_at_their_touch two_trimmed_spheres_build two_turns_on_one_l_match_the_slab_oracle two_wedges_sharing_the_tangent_plane_face_answer_every_op unions_through_ring_vertices_and_like_far_ends_build_sound valid_bodies_shell_at_both_certifying_scalars walls_whose_sides_turn_across_the_shared_band_build whole_rims_build_at_the_clip_oracle
