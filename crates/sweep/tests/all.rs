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

#[path = "a_carried_record_no_reading_covers.rs"]
mod a_carried_record_no_reading_covers;
#[path = "a_circle_of_one_radius_elsewhere_is_no_rim.rs"]
mod a_circle_of_one_radius_elsewhere_is_no_rim;
#[path = "a_curve_off_its_face_at_a_build_is_the_kernels.rs"]
mod a_curve_off_its_face_at_a_build_is_the_kernels;
#[path = "a_curved_keel_in_the_plane.rs"]
mod a_curved_keel_in_the_plane;
#[path = "a_curved_keel_on_the_floor_refuses_its_slit_form.rs"]
mod a_curved_keel_on_the_floor_refuses_its_slit_form;
#[path = "a_cylinder_split_just_off_its_rim_vertex.rs"]
mod a_cylinder_split_just_off_its_rim_vertex;
#[path = "a_face_across_a_wavy_corner.rs"]
mod a_face_across_a_wavy_corner;
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
#[path = "a_keel_on_a_bores_ruling.rs"]
mod a_keel_on_a_bores_ruling;
#[path = "a_keel_ring_in_an_elliptic_section.rs"]
mod a_keel_ring_in_an_elliptic_section;
#[path = "a_loft_computes_its_walls_in_t.rs"]
mod a_loft_computes_its_walls_in_t;
#[path = "a_lofted_lens_blade_certifies_its_sign.rs"]
mod a_lofted_lens_blade_certifies_its_sign;
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
#[path = "a_point_past_a_spline_solids_reach.rs"]
mod a_point_past_a_spline_solids_reach;
#[path = "a_pole_and_an_apex_join_nothing.rs"]
mod a_pole_and_an_apex_join_nothing;
#[path = "a_ray_meets_a_curved_edge_once_for_both_faces.rs"]
mod a_ray_meets_a_curved_edge_once_for_both_faces;
#[path = "a_rigid_turn_keeps_a_slivers_volume.rs"]
mod a_rigid_turn_keeps_a_slivers_volume;
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
#[path = "a_split_through_a_wrap_edges_vertex.rs"]
mod a_split_through_a_wrap_edges_vertex;
#[path = "a_swaying_loft_corner_meters.rs"]
mod a_swaying_loft_corner_meters;
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
#[path = "a_wall_seam_takes_the_column_it_lies_on.rs"]
mod a_wall_seam_takes_the_column_it_lies_on;
#[path = "a_wrap_tip_sits_on_its_faces_chart_singularity.rs"]
mod a_wrap_tip_sits_on_its_faces_chart_singularity;
#[path = "an_annular_tube_through_a_plate.rs"]
mod an_annular_tube_through_a_plate;
#[path = "an_arc_loft_seams_its_rational_wall_at_every_spacing.rs"]
mod an_arc_loft_seams_its_rational_wall_at_every_spacing;
#[path = "an_arc_rim_picks_the_row_it_lies_on.rs"]
mod an_arc_rim_picks_the_row_it_lies_on;
#[path = "an_arc_tangent_at_a_boolean_vertex.rs"]
mod an_arc_tangent_at_a_boolean_vertex;
#[path = "an_arcs_sweep_structure_is_certified_for_every_member.rs"]
mod an_arcs_sweep_structure_is_certified_for_every_member;
#[path = "an_edge_running_past_a_face_reads_its_crossings.rs"]
mod an_edge_running_past_a_face_reads_its_crossings;
#[path = "an_inside_out_thin_tube_operand.rs"]
mod an_inside_out_thin_tube_operand;
#[path = "at_rest_pcurve_faces.rs"]
mod at_rest_pcurve_faces;
#[path = "band_apart_partners_on_a_steep_ellipse.rs"]
mod band_apart_partners_on_a_steep_ellipse;
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
#[path = "unjoined_operand_at_a_dual.rs"]
mod unjoined_operand_at_a_dual;
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
#[path = "band_one_cycle_members.rs"]
mod band_one_cycle_members;
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

#[path = "census_containment_cause.rs"]
mod census_containment_cause;

#[path = "wire_loft_end_profile_lift.rs"]
mod wire_loft_end_profile_lift;

#[path = "wedge_end_doors.rs"]
mod wedge_end_doors;

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
#[path = "a_closed_conic_in_the_partners_face.rs"]
mod a_closed_conic_in_the_partners_face;
#[path = "a_cone_with_a_tilted_seam.rs"]
mod a_cone_with_a_tilted_seam;
#[path = "a_mirrored_body_is_its_image.rs"]
mod a_mirrored_body_is_its_image;
#[path = "a_revolved_wall_arc_join_reads_its_reach.rs"]
mod a_revolved_wall_arc_join_reads_its_reach;
#[path = "a_seam_inside_a_face_is_no_boundary_of_it.rs"]
mod a_seam_inside_a_face_is_no_boundary_of_it;
#[path = "an_aimed_ray_from_far_reads_a_split_side_inside_out.rs"]
mod an_aimed_ray_from_far_reads_a_split_side_inside_out;
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

#[path = "a_plane_near_a_needle_cones_apex.rs"]
mod a_plane_near_a_needle_cones_apex;
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
