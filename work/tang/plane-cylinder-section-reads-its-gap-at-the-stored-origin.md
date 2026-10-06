---
id: plane-cylinder-section-reads-its-gap-at-the-stored-origin
kind: issue
title: the plane×cylinder and equal-cylinder sections read their gaps at a stored origin their tilt lever does not reach
status: closed
opened: 2026-10-03
priority: P2
cost: M
closed: 2026-10-06
---


## What

`plane_cylinder_section`'s axis-in-plane lane
(`plane_cylinder_ruled`, `crates/geom-brep/src/intersect.rs`)
decides `pc_axis_plane_parallel` on `(axis·n)·extent`, then reads
`pc_parallel_gap` as `r − |(o − q)·n|` at the cylinder's STORED origin
`o`. `extent` is a bare scalar with no pivot, so the tilt the axis row
bridges (up to `zero/extent`) moves the gap by `s·|axis·n|` between `o`
and a consumed point `s` metres along the axis from it. Nothing bounds
`s` by `extent`: a cylinder whose stored origin stands 1000 m along its
axis from the faces the verdict is consumed on, with the axis row
reading `0.5·zero` at a 1 m extent, carries a gap error of `500·zero`
into the row that decides crossing, tangency or clearance.

The tangent-locus lane does not hit this: it re-bases the cylinder to
the foot of its consumed extent's centre before calling the lane
(`tangent_locus`, `crates/geom-brep/src/locus.rs`). The other callers
pass a scalar and the stored surface: `chord_join`'s
`plane_cylinder_section(plane_s, wall, extent, band)`
(`crates/topo/src/chord_join.rs`) and the germ frame's
`plane_cylinder_section(plane_s, cyl_s, radius, band)`
(`crates/topo/src/boolean/join.rs`, levered at the radius).

`cylinder_cylinder_section`'s parallel arm has the same shape, measured
(the PR 3949 review, reproduced in-lane at the witness band): `cc_coaxial`
and `cc_parallel_gap` read `d` from `o2 − o1` at the STORED origins, the
tilt levered by a pivot-free scalar `extent`. Two radius-1 cylinders,
axes 2 apart at `x = 0`, the second's axis tilted `0.5·zero` toward the
first and its stored origin 1000 m along it, `extent` 1:
`cylinder_cylinder_section(c1, c2, …)` answers `Empty` and
`cylinder_cylinder_section(c2, c1, …)` answers `TangentLine`.

The witness lane reads the same pair differently in each order too, on
its lever rather than its gap. `tangent_locus` reads the gap from the
foot of the extent's centre on the SECOND cylinder's axis to the first's
axis line, which is where the extent is in either order; but it levers
`cc_axes_parallel` from that foot, so the lever counts the second
axis's perpendicular offset from the extent's centre. On the pair above,
`tangent_locus(c1, c2, …)` escalates on `cc_axes_parallel` (margin
`1.5·zero`, a lever of 3 m) and `tangent_locus(c2, c1, …)` mints the
ruling (a lever of 1 m). The escalating order is the conservative one.

One row name is now metered from two inputs: `pc_parallel_gap` (and
`cc_parallel_gap`) at the foot of the consumed extent when the witness
lane asks, at the stored origin when a section caller asks. The fix has
to close that as well, so that a verdict log's row means one reading.

## The fix's shape

Both classifiers take an `ExtentBall` and read their gap at the foot
of its centre on the (or each) axis, levering the tilt from there, with
a pivot that does not depend on operand order — the witness lane's
construction, moved into the classifiers so every caller, the witness
included, gets one reading. The plane×cylinder half was found reading only, by the
`tangent-locus-re-meters-the-section-classifiers-tangency` lane; the
cylinder half was measured as above.

## Review tier

SINGLE, FULL: the classifiers' pivot moves every caller's band reading;
one full review with a differential.

## Closed

Both classifiers take an `ExtentBall` for the faces their verdict is
consumed on and read their gap where it is:

- `plane_cylinder_section` reads `pc_parallel_gap` at the foot of the
  ball's centre on the axis and levers `pc_axis_plane_parallel` from
  that foot (`ExtentBall::lever_from`), the witness lane's construction
  moved into `plane_cylinder_ruled`.
- `cylinder_cylinder_section` reads `cc_coaxial` and `cc_parallel_gap`
  as the distance between the two axes' feet, which is the same number
  in either order, and levers `cc_axes_parallel` from the foot nearer
  the ball's centre (`ExtentBall::lever_between`). Holding either axis
  and turning the other about its own foot bounds the same
  displacement, so the lesser bound holds; the pivot is a property of
  the pair and the ball, not of which operand is named first.

`tangent_locus` calls the same helpers (`plane_cylinder_ruled`,
`parallel_axes_at`, `lever_between`), so `pc_parallel_gap` and
`cc_parallel_gap` are one reading whichever lane logs them. Callers now
hand a `Reach`: where to read the gap, and a lever. No lever is a ball
chosen around the consumed region (two full reviews; each ball a caller
tried decided an in-band tilt as served):

- `route_pose` hands the edge's span (`Reach::Span`), levered by its
  per-carrier farthest distance from each pivot. On each axis the pivot
  is the point that makes that distance least, so no lever is longer
  than main's from the stored origin (a third review found a NURBS
  ruling read at its control-point mean levered longer).
- chord_join hands its base vertex and `face_extent`
  (`Reach::Measured`), the length it levered by before, floored at the
  pivot's distance from the vertex.
- The germ frame hands the centre of the curved face's boundary
  vertices and the length it levered by before: the radius for the
  plane×cylinder pair (filed: it under-states a long wall), the larger
  radius or the walls' span for the cylinder pair.
- The tangent-locus witness reads the ball its callers hand it
  (`Reach::Ball`), as before.

Rows pin each caller: the plane×cone near-parabola and a ruling edge
(`route_pose`), a coin on edge on a table (the germ frame), a wall's
base vertex on the tangent ruling (chord_join). Each was red on the
ball levers and is green on the exact ones. The crossing lane and the
germ frame ask the table's own `cc_axes_coplanar` and
`cc_axes_parallel` (`geom_brep::cylinder_axes_coplanar`,
`cylinder_axes_parallel`), read between the feet.

The class sweep's three siblings (cone×cylinder's `coc_coaxial`, the
join's parallel radical plane, `chart_region_cyl_offset`) are filed as
`cylinder-offsets-read-at-a-stored-origin-off-the-reach`, with
`carrier_cyl_reach`'s operand-2 pivot and `route_pose`'s cylinder
anchor added in the fix passes. Also filed:
`chord-join-face-reach-misses-a-curved-edges-bulge` (both directions of
chord_join's lever) and
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.
