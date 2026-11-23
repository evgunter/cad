---
id: a-shared-vertex-the-chart-holds-on-one-clip-only-reads-no-zero
kind: issue
title: A cylinder's cell whose shared vertex lies on one clip only (mid-side between two edges of one line, or at a lone clip's end beside a seam) has no zero in zero_of and reads Local at every depth
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
branch: embed/clip-vertex
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), tracing the
cylinder buckets of the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`). **22 tests** of valid bodies refuse at their
door on it: 9 of the plane/cylinder edge bucket, 2 cylinder/plane edge,
8 cylinder/plane vertex, 3 cylinder/cylinder vertex.

`zero_of` (`crates/topo/src/injective.rs`) gives a chart presentation
its zero: the clips whose every edge is shared, and a shared vertex no
zero side holds (a "loose" one) read as the ONE corner of two clips on
different axes that both end at it. **As traced (main `c2a0c41f40`)** a
loose vertex that was the corner of no such pair had no zero
(`Unpresented::Local`), every cell holding the vertex read the same, and
the pair ended at `Budget(Depth)`, reported
`Uncertified { arm: Departure }` on the shared edge or vertex. **Since
`embed/clip-vertex`** a loose vertex adds a zero and never forbids one
(the dated section at the end). The partner is presented (`Of(Point)` or `Of(Side)`) in
19 of the 22.

Measured with the clips printed at the refusal (`CVZ` rows on
`embed/trace-cylinder-measure`), three shapes, each with `corners = 0`:

- **The vertex mid-side of one clip (10 tests).** Two edges on one
  coordinate line meet at the shared vertex (a vertex minted on a ruling
  or a circle where the partner's face touches it alone), or the line is
  a closed circle whose own vertex it is. The vertex is on that clip's
  edges and on no clip of the other axis. Examples:
  `sweep::all face_face_touch::a_bridged_rod_standing_off_the_plate_discovers_nothing`
  (clip `XHi` at `u = π`, edges `16v1` and `34v1` meeting at `30v1`, the
  vertex the plate's face shares alone), `a_minted_vertex_on_a_curved_face`,
  `a_ruling_lying_on_a_wall` (2), `axis_lap`, `edges_lying_in_a_face_part_wherever_they_lie`,
  `verbs_1031b_arcwind`; and the closed circle with its vertex,
  `a_cylinder_through_a_hole_tangent_to_its_wall` (2),
  `a_closed_conic_in_the_partners_face::a_boss_over_two_holes_tangent_outside…`.
  The step-import row `s58_iso_rectangle::merge_coplanar_faces_no_longer_turns_an_exact_body_into_a_wrong_one`
  reaches the same exit; its clips were not printed.
- **The vertex at a lone clip's end, its other edge no clip in the cell
  (9 tests).** The vertex ends the one clip's edge and the edge leaving
  it on the other axis is no clip: a seam ruling (two uses, never a
  clip; `wedge_through_a_full_turn_collar` 2), or a ruling across the
  seam from the cell (a near-full-turn face's start seen from its end,
  `full_turn_wall`, `review_m2_pr5`, `verbs_rim_r1_probes`; a fillet's
  rim ruling at `u = 0` from a cell at `u ≈ 2π`, `band_turn_overrun_fillet`
  2, `band_open_mixed_chain_shapes`, `band_closed_mixed_chain_shapes`).
  Whether the last group's cell is outside the face, touching its
  corner through the seam (`vacuous-reads-a-cell-at-a-corner-local-at-every-depth`'s
  shape), is not checked.
- **Neither arm (2 tests).** `a_closed_conic_in_the_partners_face::short_arcs_at_a_crescents_tip`
  and `::review3_a_meeting_off_the_carrier_says_so` exit at `zero_of`'s
  last arm (zero sides and a loose vertex together), with the plane
  partner unpresented too (`present_plane`'s edge-count arm). Not
  traced further.

## Valid or in band

Valid. The mid-side vertices are the ordinary trace of a partner face
touching a wall at one point; the seam and near-full-turn shapes are
rims built at their closed forms. A near-full revolve's gap of `1e-3`
rad is far outside the band.

## What a fix needs

For a vertex mid-side of one clip: landed as the clip's whole side
through the vertex (`Zero::Through`), not as a point on it. A point in
the middle of a side has the rectangle on both sides of it along the
side, so no first-order sign is read about it unless the separator
vanishes along the side's whole line; the plane is one that holds that
line, not `planes_through`'s vertex-alone plane.
For a lone clip's end beside a seam: the seam's line placed as a clip
on the cell's sheet, so the vertex is a corner. **Not built**; see
"What is left" below.

## Evidence

- Cheapest reproducer: `sweep::all a_closed_conic_in_the_partners_face::short_arcs_at_a_crescents_tip`
  (0.13 s, last arm); `sweep::all face_face_touch::a_bridged_rod_standing_off_the_plate_discovers_nothing`
  (0.21 s, mid-side).
- Checked: the printed clips and the vertex's edges, in all 21 sweep
  tests; the fix is not.

## Two halves of one cylinder across a seam ruling (EMBED lane `embed/iso-end`, 2026-11-22)

In these 2 the refusing pair is two faces of one cylinder carrier sharing a ruling that ends in the cell. Each face has one clip, its edge stopping at the shared vertex inside the cell's side, so both exit at `sides_are_the_faces`. Neither carrier signs the other, since they are the same carrier.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `editor-core::all emit_union_seam_rank::crossings_of_an_arc_cut_twice_name_apart_by_sense_in_every_member_order`
- `editor-core::all emit_union_seam_rank::crossings_of_seams_with_no_start_name_apart_by_sense_in_every_member_order`

## What landed and what moved (EMBED, lane embed/clip-vertex, 2026-11-22)

**What `zero_of` reads now.** A presentation's zero is the points of
its rectangle that lie on the shared cell, and a loose vertex adds a
zero and never forbids one:

- a lone loose vertex on one clip, on no corner, with no shared edge in
  reach: that clip's side, through the vertex (`Zero::Through`).
  `sign_on` reads it only under a plane that holds the side's whole
  line (`planes_through`: through a ruling), against a partner whose
  zero is none or the corner point at that vertex (`through_read`). It
  is read on a line edge only: a vertex on a circle's side names no
  zero;
- a loose vertex beside a wholly shared side leaves that side the zero;
  of several loose vertices the one at a corner of two clips is the
  point; anything else names no zero (`Zero::None`), read by the
  separator's value over the whole rectangle.

**Measured.** Main `a3d5ce89c` with the harness `ebb2e6f8e`
(`embed/clip-vertex-measure`), before and after on that one base, the
gate logging and passing (`CAD_GATE_PASS=1`), on 55 census tests: every
`2-refused` Departure row with a cylinder, torus/torus or cylinder/torus
pair in the modules this row, the corner-of-no-pair row and the bushing
row name (48 `sweep::all`, 3 `step-import::all`), and 4
`editor-core::all` rows that ask the bushing. 5,006 asks.

| | before | after |
|---|---|---|
| asks `Embedded` | 4,101 | 4,165 |
| asks `Uncertified { Departure }` | 745 | 681 |
| asks `Crosses` / budget / second-order | 64 / 95 / 1 | 64 / 95 / 1 |
| tests whose every ask is embedded | 0 | 5 |
| tests with more asks refused | | 0 |

`zero_of`'s `Local` exits were counted on an earlier run (main
`b99f71c3d`, the lane then stacked on `embed/straight-corner`, whose
plane reading moved none of these counts), in the 51 traced tests:
41,802 before, 11,702 after.

The five: `face_face_touch::a_bridged_rod_standing_off_the_plate_discovers_nothing`
(mid-side, on a ruling), `full_turn_wall::a_near_full_revolves_gap_is_out_at_both_doors`,
`review_m2_pr5::survives_angle_full_range_boundary_rows`,
`verbs_rim_r1_probes::a_near_full_period_open_arc_decides_its_sign_at_the_honest_lever`
(the lone clip's end across a near-full turn's gap) and
`a_closed_conic_in_the_partners_face::review3_a_meeting_off_the_carrier_says_so`
("neither arm"). Four more lose refusals and keep some:
`short_arcs_at_a_crescents_tip` 56 to 34 Departure asks of 128,
`a_boss_inside_a_hole_sharing_a_vertex_beside_the_tip` 70 to 42 of 144,
`axis_lap::an_axis_lap_builds_every_op_as_its_planar_twin_does` 12 to 8
of 34, `s58_iso_rectangle::merge_coplanar_faces…` 3 to 1 of 3. The other
46 keep their counts. The remaining 11,702 `zero_of` exits are all its
last arm (three zero sides, or two on one axis).

**The near-full-turn group's cell is outside the face's part** (the
row's unchecked question): the shared ruling and its vertex are the
face's START, a turn away in its chart and `1e-3` rad away in space, in
reach of the cell's ball at every depth and on no point of the
rectangle. The clips there name a side (or nothing), and the vertex,
which named nothing, used to refuse the cell.

## What is left of this row

Measured on the same run, each with its trace (`CVT`, `CVS` rows):

- **A circle through the vertex, against a plane corner**
  (`a_ruling_lying_on_a_wall::a_half_rods_rim_across_a_rods_wall_moves_with_its_flat_face`,
  `edges_lying_in_a_face_part_wherever_they_lie::a_keel_along_a_cylinder_floors_ruling_refuses_only_its_slit`):
  the wall is presented `Through(YLo)` and signed by its circle's
  plane, and the plane partner's corner point is not
  (`planes = (Some, None)`). **Unchecked** which of the corner's two
  derivatives fails. A corner one of whose edges leaves the vertex
  inside the circle's plane (a flat face resting in the rim's plane)
  would read exactly this, its derivative along that edge being zero,
  and would need that edge's side as its zero under a plane it shares
  no edge with, which `present_plane` does not offer.
- **Two circles through the vertex** (`a_cylinder_through_a_hole_tangent_to_its_wall` 2,
  `a_boss_over_two_holes_tangent_outside…`): both walls are presented
  `Through(YLo)`, and a side through the vertex is not read against a
  partner's own (`planes = (Some, None)`). By the
  tests' construction the two rims are tangent at the vertex, a
  second-order contact; that no first-order separator parts them is
  read off that, not probed.
- **A lone clip's end beside a seam** (`wedge_through_a_full_turn_collar` 2):
  the refusing cells exit at `sides_are_the_faces` now, not at
  `zero_of`; the seam's line is still no clip.
- **The fillet group** (`band_open_mixed_chain_shapes`,
  `band_turn_overrun_fillet`, `band_closed_mixed_chain_shapes`): counts
  unchanged; where `zero_of` was the exit the cell is now presented with
  no zero and the separator's value does not sign it
  (`a_turned_l_plates_run_builds_at_interval`: `planes = (Some, None)`).
  Not traced further.

## After the review (EMBED, lane embed/clip-vertex, 2026-11-23)

- **The circle's own plane is removed.** On the 55 census tests no ask
  is certified by it (switched off, every count is the same), and the
  tour's verdicts are the same without it. It signed the wall in the
  two shapes above and the partner then failed. What would bring it
  back is a body whose partner is strictly across a rim circle's plane
  at a vertex in the middle of the rim: a corner standing on a rim
  point with both its edges leaving the rim's plane, or the seam-clip
  case once a seam's line is a clip.
  `injective::presentation_rows::a_wall_and_a_corner_at_a_vertex_mid_line_certify_under_the_lines_plane`
  pins that such a pair is split today.
- **What "holds" is true of** (on the landing branch `8d34076a9e`).
  The wall row holds at both scalars, pair and whole body, on a dyadic
  build and on one off the grid (radius 0.7, block
  `[0.13, 0.7] × [-0.61, 0] × [0.3, 1.7]`), each as built, turned about
  a generic axis and a hundred units off, and mirrored across an axis
  plane. Before the cover read a joint's two stored ends as one line
  (PR 7322) the off-grid build's lower pair refused at `f64` and held
  at `Interval`: the ruling's two edges store the vertex's height as
  `0.3` and `0.30000000000000004`, and a cell across that ulp was not
  covered (`sides_are_the_faces`, 41 failures in the trace), so the
  wall never reached `zero_of`. Mirrored across a generic plane the
  block holds at `f64` and refuses at `Interval`, and the ring refuses
  at both, upstream of this reading
  (PCERT's `a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64`, its third sighting: the mirrored rim rows store a rounding in the axial channel).
- **The one corner among several loose vertices is signed only where it
  is the first shared vertex** (review, by inspection): the
  vertex-alone plane and `anchor` take `near.vertices.first()`, and a
  corner point is zero only under a plane that lists its vertex. Not
  fixed and not pinned by a row; the reading is named and unsigned for
  the other order.
- **A flap tangent to the wall at the vertex only** (the right verdict
  is `Holds`) reads `Budget` at both scalars (the review's probe). No
  first-order separator parts a tangent pair.
- **Two circles through one vertex** are both cells with no zero now
  (a circle's side is not read), and are split as before. No test row
  of their own; `a_cylinder_through_a_hole_tangent_to_its_wall` is the
  census body.
