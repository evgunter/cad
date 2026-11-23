---
id: the-vertex-alone-plane-leans-along-a-cells-long-metric-side-so-an-anisotropic-pair-splits
kind: issue
title: planes_through's plane at a shared vertex alone is taken across the rectangles' midpoint headings, so on an anisotropic chart it leans along the cell's long metric side and a strictly separable pair splits to the floor
status: closed
opened: 2026-11-17
priority: P0
cost: M
refs: [departure-reads-no-curved-corner-off-its-shared-edge]
pr: 7318
branch: embed/vertex-bisector
closed: 2026-11-23
---

## Finding

`topo::injective`'s `planes_through`, at a vertex the pair shares
alone, builds one plane: `n = unit(hx − hy)`, where `hx` and `hy` are
the headings from the vertex to each presentation's rectangle midpoint
(`heading`). That heading is a midpoint's, not the wedge's. On a chart
whose speeds differ a lot, a cell that is square in the chart is long
and thin on the surface, and the heading points along its long side.

## Evidence

Measured on the review's thin-cylinder fixture (PR 7093's fix pass, a
scratch row not kept). The fixture is two diagonal quadrants at a
vertex alone on a cylinder of radius 0.055, the same pair that
certifies on radius 1. Cells were chart-square, of half-width 0.02
and 0.002.
- Both faces are presented as their quadrants (`Zero::Point`).
- The one plane is `n ≈ (0.046, −0.030, 0.998)`, almost the ruling's
  direction. The quadrant's metric extent along the ruling is 0.020 m,
  against 0.0011 m along the parallel.
- The leaned axis `B` has `z = −0.25`, so `n·B < 0` while `n·A > 0`.
  `sign_on` gives `None` on both faces, and the pair is
  `Split { Departure }` at every half-width tried (0.02, 0.002; the
  review also tried 0.0005 and 0.00005).
- The quadrants are strictly separable, for example by the bisector of
  the two wedges' edge directions.

No body measured refuses on this. The re-posed torus belly
(cylinder radius 0.055) certifies, because its cells reach the vertex
wider along the parallel.

## What a fix needs

A vertex-alone candidate taken from the presentations' wedges rather
than their midpoints. For example, `n` across each face's two corner
directions (the images of the quadrant's axes at the vertex), as the
two-line case already builds one from the edges' directions. Any
direction is exact, so this is a choice of candidate, not a decision.

## 2026-11-21: planar bodies refuse on it (the per-solid census)

The planar lane of the per-solid census (`embed/trace-planar`) traced
this on plane/plane pairs. The row said no body refuses on it, and the
gate now refuses 24 valid bodies on it, so it is re-banded P0 (a valid
body refused at its door).

On the census of main `c2a0c41f40`, **departure, vertex, plane/plane:
24 of the 96 traced** (of 101). Both faces present as convex quadrants
at a vertex they share alone (`Zero::Point` on both), and the pair
splits to the depth floor. Cheapest:
`sweep::all a_keel_tree_in_the_plane::a_deep_keel_tree_in_the_plane_splits_sound_and_sections_with_no_spur`.
The other families are `band_mixed_vertex` (3), `union_pinch_member_order` (4),
`three_solids_on_one_line` (2), `holes_meeting_at_a_vertex` (5),
`join_rc_probes`, `join_reflex_wedge_probes`, `a_flank_touch_at_a_shared_corner`,
`emit_union_borders`, `coincidence_door`, `split_through_an_apex`,
`split_edge_on_faces_poses` and `join_pierce_runs_sweep`.

**Checked by a diagnostic candidate** (`embed/trace-planar-measure`,
not kept). At each plane/plane pair presented `Zero::Point` on both
faces at a vertex alone, `departure` also built `n = unit((a₁ + a₂) −
(b₁ + b₂))`. Here `a₁, a₂` and `b₁, b₂` are the unit images of each
presentation's two corner axes at the vertex, so `n` is the difference
of the two wedges' bisectors. The candidate was asked through the same
`opposite` as `planes_through`'s.

- In 17 of the 24 tests the candidate certified **every** such pair
  that the midpoint-heading plane left split. In the keel trees it
  certified 79 of 79 where the heading plane certified 3, in
  `band_mixed_vertex` 124 of 124 where the heading plane certified 8,
  and in `union_pinch_member_order` 38 of 38 where it certified 1.
- In 7 it did not certify them all: `holes_meeting_at_a_vertex` (5,
  for example 0 of 233 and 7 of 388), `union_pinch_member_order::holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_order`
  (8 of 381) and `split_through_an_apex::a_split_through_an_apex_closes_each_corner_of_its_section`
  (1 of 106). In 6 of the 7 the bisector difference does not part
  wedges that a plane does part, and the hull's nearest point does (the
  section below); in `split_through_an_apex` a rectangle with no width
  across one axis is decided by no plane
  (`a-vertex-alone-quadrant-a-cell-holds-along-one-edge-has-no-width-to-depart-across`).

So the candidate this row asks for closes most of the planar instances,
and the rest want a candidate chosen against both wedges' edges (a
plane strictly separating two convex cones at their common apex, when
it exists), or a reading of their own.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate and prints of
`departure`'s separators (`embed/trace-cone-torus-measure`). Two pairs
sharing a vertex alone, both faces presented by a `Zero::Point`, split
to depth 40 with `planes_through`'s one plane signing the plane face and
not the curved one, and no carrier or graph separator:
- `band_open_mixed_chain::open_mixed_chains_chamfer_whole_at_pappuss_volume_at_interval`
  (`Interval`, plane `3v1` against a chamfer's cone `16v1` at `19v1`):
  `planes = [(Some(Positive), None)]`, the cone's cell
  `u ∈ (3.926991, 3.926991 + 9e-13)`, `v ∈ (0.42371, 0.42426)`;
- `band_planar_cut_off_interval::the_box_rows_carve_at_the_certified_scalar_and_bracket_their_closed_forms`
  (`Interval`, plane `3v1` against sphere `10v3` at `16v1`): the same,
  the sphere's cell `v ∈ (−2.2e-15, 0)` by `6.1e-3` in `u`.

Both curved cells are anisotropic by many orders, the shape this row
describes. That the plane leans along the long side is not printed, so
the match is unchecked. Assigned here by shape, 14 tests: the vertex,
plane/cone bucket (4: the chamfers, three at `Interval`), plane/sphere
(9) and sphere/plane (1). Two traced. Valid bodies.

## 2026-11-21: the wedge plane (embed/vertex-bisector)

**What separates two quadrants at a vertex.** Each face presented as a
quadrant at the vertex leaves it into the cone of its two corner axes,
`a₁, a₂` and `b₁, b₂`. A plane through the vertex with
`n·aᵢ > 0 > n·bⱼ` exists exactly where the origin is not in the hull of
`{a₁, a₂, −b₁, −b₂}` (Gordan): where the cones share no ray and neither
corner is straight (a straight corner's antiparallel axes put the
origin in the hull whatever the other is). The hull's point nearest the
origin is then such an `n`, the one that maximises the least cosine to
the four axes. The bisector difference is the hull's centroid, which a
separable pair need not have on the right side (pinned:
`two_plane_wedges_at_a_vertex_alone_certify_only_where_a_plane_parts_them`'s
straddling wedge). At a vertex alone `planes_through` now offers one
plane per pair: where both faces are quadrants, that wedge plane
(`wedges_apart`, the axes read over each rectangle,
`Presented::corner_axes`, the nearest point by
`geom_core::linalg::min_norm::min_norm_point`); against any other
presentation, the heading plane. `opposite` still decides each face's
sign at the band, so a plane that parts nothing certifies nothing.

Why the heading plane stays there and nowhere else (2026-11-22, the
review of PR 7318), over the 111 census tests below (the 100 and the
11 curved): with the axes read at the corner point, the heading plane
alone certified 27 quadrant cell pairs, all on curved faces; read over
the rectangle, as `sign_on` decides, it certifies none. Dropped
altogether, two curved tests refuse earlier
(`blend_per_shell_carry::the_sealed_island_request_builds_at_every_pose`
at its 9th ask, not its 17th; `corner_patch_chart_poles` at its 1st, not
its 3rd): there a plane quadrant meets a sphere's pole read as its
chart's collapsed side (`Zero::Side`), which has no corner axes, and the
heading plane certifies 348 such cell pairs. The form landed reads all
111 tests at the same ask and verdict as the two planes did.

**Measured** on `embed/vertex-bisector-measure` (main `9bc9d6517` with
`embed/gate-measure` merged and a `CV_NO_WEDGE` switch for the before),
the gate asking, over the census's 101 vertex plane/plane tests (100
ran; `boolean::holdings_rows::every_brick_booleans_face_holds_only_member_faces_on_its_plane`
is in `topo`'s lib binary, which the run's filter missed):

- Before, exactly 24 refuse first at a pair presented `Zero::Point` on
  both faces: the 24 this row holds.
- After, of those 24: 3 pass (`emit_union_borders::a_wall_through_a_notch_apex_names_alike_in_every_order`,
  `split_edge_on_faces_poses::an_edge_at_the_zero_bands_edge_refuses_typed_and_its_lever_builds`,
  `union_pinch_member_order::two_wedges_overlapping_about_one_line_publish_their_apex_rows`);
  7 go on to a `Crosses` (both keel trees at the split, a slit by
  their own account, the door holding no record; `coincidence_door`,
  two of `union_pinch_member_order` and two of
  `three_solids_on_one_line` at a Boolean, the door's records covering
  the crossing in three of them, the two `union_pinch_member_order` and
  `coincidence_door`; the census already holds crossings of these shapes in
  the same families, as class `1-not-embedded?` and `3-in-solid`); 6 go
  on to `Uncertified { Departure }` at an edge (two `band_mixed_vertex`,
  three `holes_meeting_at_a_vertex`,
  `union_pinch_member_order::holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order`);
  2 go on to the depth budget (two `holes_meeting_at_a_vertex`); 2 go on
  to a vertex pair not presented `Zero::Point` on both
  (`a_flank_touch_at_a_shared_corner`, `join_reflex_wedge_probes`); and
  4 still refuse on a `Zero::Point` pair with a parting plane found,
  each with a rectangle of no width across one axis, filed as
  `a-vertex-alone-quadrant-a-cell-holds-along-one-edge-has-no-width-to-depart-across`.
  Every test that moved refuses at a later ask than before; none of the
  100 refuses earlier.
- **Where the 7 crossings lie** (a `CROSS` print on the measuring
  branch: each station's point, the body's nearest vertices, whose they
  are): every one at a body vertex's point, the two faces sharing no
  vertex in M. In both keel trees the split leaves the keel's joint
  `(2, 1, 1)` as two or three distinct vertices within `1.4e-15` of one
  another, on different faces. In the other five a vertex one face holds
  lies on the other face, which does not hold it, at `(1.5, 1, 1.7)`
  (four) and `(1.5, 1, 1.4544)` (`pierces_of_one_point`). The boundary
  meets itself at each: one point of space is two points of M, decided
  by distance, the wedge plane not asked. The review of PR 7318 found
  the witnesses of four of them independently.
- The other 76 refuse first at a vertex pair not presented
  `Zero::Point` on both, before and after: not this row's.

**The curved half** (`embed/apex-separator`'s). Over the 11 `sweep`
tests of the 14 (plane/cone 4, plane/sphere 5, sphere/plane 1,
cone/plane 1), the wedge plane is asked at every `Zero::Point` pair and
found at `f64` at every one; at the refusing pairs it parts neither
face at the band, as the heading plane does not. Each of the 11
refuses at the same ask and on the same shape before and after.


**The tour walk**, all 85 stops, main `620e29f55` against the wedge
plane read over the rectangle: 78 of 85 hold on both, every verdict
alike. Three stops certify in fewer splits: `snowman_lens` 417 → 337,
`snowman_bitten` 481 → 449, `lily_lantern` 20,297 → 20,293 (still
refused on the same edge).
