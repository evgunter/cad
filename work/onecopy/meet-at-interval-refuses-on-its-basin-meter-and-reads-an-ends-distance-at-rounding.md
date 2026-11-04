---
id: meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding
kind: issue
title: curve_pair::meet at Interval refuses on its basin meter where the region rows decided, reads an end's distance at rounding, and costs 3-6x the rows
status: open
opened: 2026-11-04
priority: P1
cost: M
refs: [region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet]
---

Found by `onecopy/meet-pose` (PR 6491, now a draft), which routed the four region `landings` (`topo/src/boolean/{region,sphere_region,torus_region,ruled_region}.rs`) through `geom_brep::curve_pair::meet`. On `Interval` and at a fine band, `meet` decides less than the hand rows it replaced and costs more. **The landings route (PR 6491) waits on this row.**

## (i) The basin meter refuses at Interval

`topo/tests/props_sphere_cap_door.rs` `a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars`, at ε 1e-12 (band 1e-12/1e-11), δ 1e-2, at `Interval`:
- 124 of 128 (leg, arc) meetings refuse `MeetFault::Unsettled(Unsettled::Meter)`, from the polish's basin meter (`curve_pair/pose.rs` `Polish::closest`) on interval enclosures;
- the other 4 escalate on `curve_carrier_gap` (enclosures `[0, 2.6e-12]`).

The pole then reads `ChartSingularityUndecided` where the rows read `ChartSingularityNotAVertex { lies: Interior }`. At f64 the same scene decides.

## (ii) An end's distance read at rounding

`sweep/tests/one_region_rows.rs` `points_a_band_off_a_lunes_pole_vertex_read_at_the_ends_precision`, band 1e-12/1e-11, point `(0, 1, 1e-11)` on the 90° lune:
- The point lies on the first arc's own great circle, `1e-11` past its end (the pole).
- Every leg from it meets that carrier at its start.
- `meet`'s span-end chord (`curve_pair_span_end_chord`, `curve_pair/span.rs` `Span::place`) reads `9.9998e-12` against the escalation `1e-11`. That difference is coordinate rounding (~1e-16 on unit coordinates), so every leg escalates.
- The row it replaced read the root's angle to the end as the raw root less the end's parameter, at the end's relative precision, and decided `Out`. That is what the pin is for.

## (iii) Cost

CI `test (kernel)` on PR 6491 head `650d63fa96`: 21 fast-set rows went over the 1 s bar, nearly all certified-scalar region rows, for example:
- `one_region_rows::a_torus_face_refuses_a_ring_off_it_or_inside_another_at_the_certified_scalar` 6.46 s;
- `verbs_tubewall_r2_probes::certified::r2_interval_rows_over_varied_radii` 4.20 s;
- `one_region_rows::torus_windows_a_hair_apart_in_tube_angle_validate_at_the_certified_scalar` 3.51 s;
- `snowman::the_snowman_builds_at_the_interval_scalar` 2.68 s;
- `verbs_tubewall::certified::the_hollow_elbow_certifies_at_interval` 2.11 s;
- `pis_arc_capped_poses::the_cut_cylinder_reads_its_truth` 2.00 s;
- `cell_region_bodies::a_ball_is_sound_and_its_sphere_decides_its_cells` 2.00 s;
- `pis_arc_capped_poses::the_cut_cylinders_ellipse_face_takes_nothing_away` 1.96 s;
- `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar` 1.82 s;
- `cert_m2r1_passes::m2r1_passes_interval` 1.79 s;
- `one_region_rows::a_sphere_face_refuses_a_ring_inside_another_at_the_certified_scalar` 1.78 s;
- `one_region_rows::two_windows_at_opposite_azimuths_validate` 1.74 s;
- `fillet_h5_hostless_rim_interval::the_hostless_rim_carves_at_the_certified_scalar_on_both_material_sides` 1.65 s;
- `tilted_sphere_pair::a_tilted_sphere_pair_builds_at_the_interval_scalar` 1.53 s;
- `one_segment_loop::one_segment_circles_revolve_at_interval` 1.42 s;
- `verbs_tubewall_r1_probes::certified::enclosures_contain_and_stay_tight` 1.41 s;
- `a_vertex_on_a_grazing_plane::an_unresolved_section_reading_keeps_the_offsets_verdict` 1.24 s;
- `one_region_rows::windows_on_an_azimuth_grid_validate_at_the_certified_scalar` 1.21 s;
- `one_region_rows::a_torus_window_read_in_band_escalates_with_its_margin_at_the_certified_scalar` 1.17 s;
- `sf2b_interval_probe::interval_offset_charts_together_sphere_zone` 1.14 s;
- `contact11_torus_chart_l::a_notched_half_donut_answers_its_volume` 1.03 s.

CI `test (app)` flagged `editor-core` `m10_5_r1_probes_interval::a_block_with_a_rounded_bump_certifies_strictly_positive` at 2.44 s.

Locally, on a box shared with another lane: `sweep` `a_cone_apex_meets_a_vertex::two_cones_through_one_apex_build_across_the_pose_families` took 126 s with the landings through `meet`, against 47 s with the pose change alone.

## Asked

Make `meet` at least as decisive as the region rows at `Interval`, where the band allows a decision:
- a basin meter that does not refuse on enclosure width alone;
- an end's distance read at the end's precision, not as a chord of unit-sized coordinates.

Bring its per-pair cost near the rows' cost. Then re-run PR 6491's landings route against these rows.


## Also owed (2026-11-04, from the orchestrator's read of PR 6492)

- `pose::conic_conic`'s parallel screen reads `constant(row::PARALLEL, c0, swing + noise)` and drops a gap read through `matches!(.., Ok(true))`. When the swing is level, the same margin is read again on `row::OFFSET`. So a gap height records two escalations for one margin, and the PARALLEL one is unanswered when a later route decides. Read the margin once and act on it twice. This belongs to the class in `a-heuristics-choice-has-no-door-of-its-own` (PR 6483).
