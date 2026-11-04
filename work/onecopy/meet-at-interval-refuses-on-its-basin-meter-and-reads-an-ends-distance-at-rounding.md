---
id: meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding
kind: issue
title: curve_pair::meet at Interval refuses on its basin meter where the region rows decided, reads an end's distance at rounding, and costs 3-6x the rows
status: open
opened: 2026-11-04
priority: P1
cost: M
refs: [6516, region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet, curve-pair-polish-at-interval-reads-the-iterates-width, inline-bracket-midpoints-are-spelled-in-two-forms]
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


## Measured on `onecopy/meet-interval` (2026-11-04)

Commits `3e18c0ba64`, `d52d7e161e` take what needs no ruling:
- `polish::pair` reads each step from a point (an enclosure's middle) and solves it in the cross-product form `s = (w × b′)·n/|n|²`, `n = a′ × b′`. The normal-equations form read the rounding of the gap over `sin² φ`.
- `Span` evaluates its end points once (`Span::end_point`), and an endless span has no ends (`Span::ends`): no end candidates, no witnesses anchored at its seam.
- `conic_conic` reads its height screen once and acts on it twice.

Against PR 6491's head with these applied: (i) `a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars` passes; (ii) still escalates; the cost (iii) falls but stays at about 4x main's on six sampled rows (15.5 s against main's 3.5 s, 18.4 s before).

**Question 1: what the polish's meters certify at an enclosing scalar.** In `verbs_tubewall_r2_probes::certified::r2_interval_rows_over_varied_radii`, 13,000 of 24,000 `meet` calls refuse `Unsettled::Meter`, each sending its path to the next one: 11,344 on `curve_pair_pose_travel` (margin `[9.3e-9, 1.25e-7]` at band `(1e-9, 1e-8)`) and 1,640 on `curve_pair_pose_polish` (step `[0, 1.2e-9]`). A step read from one point still encloses the carriers' own spread divided by the sine between the tangents. So the convergence meter compares where along the curves the closest pair sits with the zero band, when the reading needs how far apart the curves are. Several answers are viable:
- (a) run the search and its meters on the enclosures' midpoints (the f64-structure lift of `geom::projection_policy`), and leave the deciding to `Span::contact` at `T`;
- (b) meter what the reading needs: the last step's metres times the sine, or the gap's change;
- (c) an interval Newton (Krawczyk) existence test for the closest pair at an enclosing scalar.
The same shows in `meet_rows::circles_on_one_sphere_meet_at_the_certified_scalar` at tilt 1e-3 (step `[0, 2e-12]` at band `(1e-12, 1e-11)`, from point-interval circles). The committed row pins only tilt 1e-2.

**Question 2: the precision pin sits on the band's closed edge.** In `points_a_band_off_a_lunes_pole_vertex_read_at_the_ends_precision`, every leg from `p = (0, 1, 1e-11)` is a great circle through `-p`, which lies `1e-11` from the south pole inside the 90° meridian. That edge's span ends at `t1 = π_f64`, whose point is `(7.5e-33, -1, -1.2246e-16)`. So the meeting at `-p` lies `1e-11 − 1.22e-16 = 9.99988e-12` from the span's end, exactly for the data. The hand row read `π_f64 − fl(π − 1e-11) = 1.0000000827e-11`: the root's rounding to the `π`-grid cancelled the end's. That is coincidence, not precision. With endless legs reading no seam, the first refusal is now `curve_carrier_gap` at `9.99988e-12`, the north pole against a leg that passes `1e-11` from it. A point at exactly the escalation width of a vertex is decided by `1e-16` either way. Either the pin moves off the edge, or `meet` reads an end's distance in parameter space, which would be a second reading of a fact the chord reads.

Question 2 was ruled a test, not a design question (ONECOPY orchestrator, 2026-11-04). The pin is re-posed on `onecopy/meet-interval` as `points_two_escalations_off_a_lunes_pole_vertex_read_on_their_side`, with its points `2e-11` off the vertex. Question 1 is with the designers.

## The review's fix pass (PR 6516, 2026-11-04)

**One finding, two homes.** The polish fix is also CPAIR's
`curve-pair-polish-at-interval-reads-the-iterates-width`, which PR 6516
closes. Its pin
(`band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval`,
pinned red at 1e-12) would have gone red on the nightly after merging,
and `band_planar_cut_off_interval`'s `tier_3_prime` admission would have
stayed with nothing using it. Both are lifted. The pin cited its row,
and `scripts/gates/pin-cites-an-open-row.sh` reds a pin whose row is
closed, so closing the CPAIR row is what makes the gate demand the
un-pin. What was missed is the step before that: this row was filed
without the CPAIR row in its `refs`, so fixing it did not close that
row. A gate already links pins to rows, so no row is filed for the
link.

**The precision pin, again.** The `2e-11` re-pose no longer guards the
landing's precision branch (`sphere_region.rs`, `to_end`): with
`to_end` always the half-width less the angle from the middle (M-a),
the old pin goes red and the `2e-11` one stays green. No row can guard
it at an honest offset. Rows at `1e-11 + 1e-16` and `1e-11 + 3e-16` are
green at head and green under M-a, and any offset small enough to read
the two forms apart lies inside the end's own rounding (`1.22e-16`), so
a row there pins that rounding, which Question 2 ruled coincidence. The
branch's comment records this; the `2e-11` pin is kept (orchestrator's
ruling, option (a)).

**Still owed:**
- `Span::new` evaluates both end points eagerly. That includes
  `topo/src/boolean/reduce.rs`'s split, which builds a `Span` only to
  call `branch`. Calling `Curve3::branch_near` there instead would
  spell `Span::branch`'s composition a second time. Making the span
  lazy is a larger change and buys little: `Span::new` already reads
  the carrier's speed range eagerly, which costs more than two
  evaluations.
- The transverse route's parallel straddle (n3 of the review).
- `geom_brep::krawczyk::midpoint` was `lo + (hi − lo)·0.5`, bitwise
  `geom::bracket_mid`'s form, and now reads it. `pair_section/
  spline_implicit/sweep/critical.rs`'s `mid` is `½(lo + hi)`, which
  differs in the last bit, so it stays: it belongs to
  `inline-bracket-midpoints-are-spelled-in-two-forms`.


## State after PR 6516 (2026-11-04)

PR 6516 fixes regression (i) at its root. Gauss–Newton now steps from a point, ends are read once, and an endless span has no ends. Question 2's ruling also lands: the lune pin is re-posed, and its precision branch is recorded as unguardable at an honest offset.

What is still owed on this row is the polish-meters decision above, "Decided: what the polish certifies at Interval". Its implementation is the next unit. PR 6491's landings wait on it.
