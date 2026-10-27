---
id: a-point-has-no-place-in-a-nurbs-faces-trim
kind: issue
title: Point-in-face has no NURBS arm: contain::curved_interior_placement answers Trim(None) for a spline face, so a pierce found on one could not be placed
status: closed
opened: 2026-10-26
closed: 2026-10-26
priority: P1
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
branch: splice/point-in-face
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(fork log row 198), on a designer's finding.

`boolean::contain::curved_interior_placement` answers `Trim(None)` for
a `Nurbs` (and `Approx`) face. A meeting the curve × face table finds
inside a spline face's chart is placed in or out of the face by the
face's trim (C5's clause). C4's derived pcurves make that a 2-D
point-in-loop reading at `(u, v)`. Teapot wall 3's pierce needs it.

## A second witness: the dihedral side door's wrong sheet

From the review of PR 5764 (`splice/review-5764b` @ 6d906e2d, row
`a_hairpin_carrier_reads_the_face_half_not_the_overlying_half`, not
landed). `geom_brep::dihedral_side` decides a spline face's foot on the
carrier, and this row is what places it in the face.

The fixture is a hairpin carrier, ruled along `x` (`s ∈ [−1, 1]`) and
degree 1 in `v` with its C⁰ knot at `v = ½`, through the `(y, z)` rows
`(1, 0)`, `(0, 0)`, `(1, h)`, with `h = ε/100` (witness tolerance). The
half `v < ½` is the floor strip `y ∈ [0, 1]`, `z = 0`. The half `v > ½`
folds back over it, rising `h` over the unit run. A face on the lower
half has its edge on the line `y = 3/7`, shared with the plane through
`(0, 3/7, 0)` with normal `∝ (0, 1, 1)`. At `p = (1/7, 3/7, 3h/7)` (a
point on the upper half, `3h/7` inside the band off the face) the
projection's foot is the upper half's, 0.43 m from the crease, and its
normal is the lower half's reversed. `dihedral_side` read
`Ok(Material)` walking `−x`; the lower half alone reads `Outside`. The
row asserted the hairpin reads its face half's side or escalates.

What the door needs from this row is the foot placed in the face's
trim before its normal is read. With that placement, the hairpin's
upper-half foot is out of the face, and the reading falls to the face
half or escalates.

## Closed

`topo::boolean::spline_region` is the NURBS and `Approx` arm of
`contain::curved_interior_placement`, which answered `Trim(None)` for
them. The point is decided on the carrier by a foot, a cell centre of
the chart's subdivided Bézier cells decided within the band, and off
it where every cell's control hull is decided past the band; no
derivative is read. The trim is read in the chart from the face's
lifted pcurve loops by ray parity, every piece decided clear of the
query by more than the band along the chart (the gap metered through
the chart's inf-side arms over a window round the query, the
`chart_region` skew-discounted arm, now one helper both read).

The sheet form: every cell within the band of the point is placed.
The reading is in where a foot within the band lies in the trim, out
where a foot exists and every kept cell lies wholly outside the trim,
and otherwise unsettled (escalated on the in-band decision it stopped
on, else `Trim(None)`). So a foot on a sheet the face does not use
cannot make the point the face's, and a point within the band of the
face on another sheet still falls to the face.

Witnesses (`crates/topo/src/boolean/spline_region_rows.rs`):
`a_point_of_a_spline_face_is_placed_in_or_out_by_its_trim` (in, out,
off the carrier, an in-band point escalating on
`bool_spline_region_carrier`),
`a_point_within_the_band_of_the_trim_is_never_given_a_side` (the
boundary walk's `OnEdge`, and the chart reading near the trim reading
`Near` on `bool_spline_region_trim`),
`a_point_at_a_crease_is_placed_without_a_derivative_across_it`, and
`a_hairpin_carriers_upper_foot_is_out_of_a_lower_half_face` (the
projection's upper-half foot placed out; a point of the upper half over
the face reads in, over the lower half past the trim out). The partial
creased-column iso limb that refused the fixture's sides is evidence on
`pctail/line-seam-limbs-partial-and-rational-column-corners-have-no-measured-caller`.

