# PR 4358 review probes (head d08b08ca)

## Mutants (topo sectors+vtxfac rows, germ oracle, cone fuzz seeds 1 and 4 at effort 3; `review_probe` = the exact apart probe)

- BASE: SURVIVED; exact probe: n/a; by []
- M1_apart_never: KILLED; exact probe: killed; by ['a_plane_through_a_reference_and_a_bound_is_read_at_the_shorter_arm', 'an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line', 'an_arc_in_a_faces_plane_is_read_apart_from_its_sector', 'an_arc_past_a_bound_it_reads_no_side_of_is_read_apart', 'every_edge_a_vertex_read_again_reads_is_classed_against_the_germ']
- M2_no_apart_p_in_band: KILLED; exact probe: n/a; by ['a_plane_through_a_reference_and_a_bound_is_read_at_the_shorter_arm']
- M3_no_apart_crossing: KILLED; exact probe: n/a; by ['an_arc_in_a_faces_plane_is_read_apart_from_its_sector', 'every_edge_a_vertex_read_again_reads_is_classed_against_the_germ']
- M4_no_apart_arc_side: KILLED; exact probe: n/a; by ['an_arc_past_a_bound_it_reads_no_side_of_is_read_apart']
- M5_no_past_u: KILLED; exact probe: survived; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M6_no_past_v: KILLED; exact probe: survived; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M7_no_past_ends: KILLED; exact probe: survived; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M8_no_past_d: KILLED; exact probe: survived; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M9_no_past_p: KILLED; exact probe: survived; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M10_unlevered: KILLED; exact probe: killed; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- M11_refusal_reverted: KILLED; exact probe: survived; by ['a_vertex_read_again_layers_its_partners', 'the_polygon_cone_reader_never_contradicts_the_exact_oracle']
- M12_inside_none_falls_back: KILLED; exact probe: survived; by ['a_vertex_read_again_layers_its_partners']
- M13_span_longer_arm: KILLED; exact probe: survived; by ['a_plane_through_a_reference_and_a_bound_is_read_at_the_shorter_arm']
- X1_lever_min_reach: SURVIVED; exact probe: killed; by []
- X2_lever_max_reach: SURVIVED; exact probe: killed; by []
- X3_strict_sign_no_band: COMPILE-ERROR; exact probe: n/a; by []
- X4_one_sided_past_u: KILLED; exact probe: killed; by ['the_polygon_cone_reader_never_contradicts_the_exact_oracle', 'a_thin_fin_reads_as_its_polygon_cone', 'an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line', 'an_arc_through_a_link_vertex_passes_over_its_reference', 'an_arcs_side_of_a_bound_is_decided_at_its_least_deviation', 'every_edge_a_vertex_read_again_reads_is_classed_against_the_germ', 'the_polygon_cone_reader_never_contradicts_the_exact_oracle']
- X5_one_sided_past_d: KILLED; exact probe: killed; by ['the_polygon_cone_reader_never_contradicts_the_exact_oracle', 'a_direction_beside_a_face_reads_the_arcs_crossing_point', 'a_thin_fin_reads_as_its_polygon_cone', 'an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line', 'an_arc_through_a_link_vertex_passes_over_its_reference', 'an_arcs_side_of_a_bound_is_decided_at_its_least_deviation', 'every_edge_a_vertex_read_again_reads_is_classed_against_the_germ', 'the_polygon_cone_reader_never_contradicts_the_exact_oracle']
- X6_apart_skips_bound_keeps_crosses: KILLED; exact probe: n/a; by ['the_polygon_cone_reader_never_contradicts_the_exact_oracle', 'the_polygon_cone_reader_never_contradicts_the_exact_oracle']
- X7_past_d_wrong_side: KILLED; exact probe: killed; by ['the_polygon_cone_reader_never_contradicts_the_exact_oracle', 'a_direction_beside_a_face_reads_the_arcs_crossing_point', 'a_thin_fin_reads_as_its_polygon_cone', 'an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line', 'an_arc_through_a_link_vertex_passes_over_its_reference', 'an_arcs_side_of_a_bound_is_decided_at_its_least_deviation', 'every_edge_a_vertex_read_again_reads_is_classed_against_the_germ', 'the_polygon_cone_reader_never_contradicts_the_exact_oracle']
- X8_inband_counts_as_side: COMPILE-ERROR; exact probe: n/a; by []
- X3_strict_sign_no_band: KILLED; exact probe: killed; by ['an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line']
- X8_inband_counts_as_side: SURVIVED; exact probe: killed; by []
- X3_strict_sign_no_band (rewritten): KILLED by an_arc_and_a_sector_in_one_plane_are_apart_across_a_bound_line; exact probe: killed
- X8_inband_counts_as_side (rewritten): SURVIVED every PR row; exact probe: killed

## Instrumented main (b7e31045), topo+sweep+editor-core, default eps, 8033 tests

163033 pair_classes calls: 158650 lone, 1966 lone with an unread partner, 2117 layered, 300 fallback,
0 refuse-unread, 0 refuse-inside (what the head would refuse).

