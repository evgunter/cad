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

`zero_of` (`crates/topo/src/injective.rs`, `:2219`) gives a chart
presentation its zero: the clips whose every edge is shared, and a
shared vertex no zero side holds (a "loose" one) read as the ONE corner
of two clips on different axes that both end at it. A loose vertex that
is the corner of no such pair has no zero: `Unpresented::Local`. Every
cell holding the vertex reads the same, so the pair ends at
`Budget(Depth)`, reported `Uncertified { arm: Departure }` on the shared
edge or vertex. The partner is presented (`Of(Point)` or `Of(Side)`) in
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

For a vertex mid-side of one clip: a zero at a point of a side
(`Zero::Point` on one side, not a corner), read by the plane through
the vertex (`planes_through`'s vertex-alone plane) as a corner point is.
For a lone clip's end beside a seam: the seam's line placed as a clip
on the cell's sheet, so the vertex is a corner. `zero_of`, cost M.

## Evidence

- Cheapest reproducer: `sweep::all a_closed_conic_in_the_partners_face::short_arcs_at_a_crescents_tip`
  (0.13 s, last arm); `sweep::all face_face_touch::a_bridged_rod_standing_off_the_plate_discovers_nothing`
  (0.21 s, mid-side).
- Checked: the printed clips and the vertex's edges, in all 21 sweep
  tests; the fix is not.
