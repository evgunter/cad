---
id: plane-cylinder-section-reads-its-gap-at-the-stored-origin
kind: issue
title: the plane×cylinder and equal-cylinder sections read their gaps at a stored origin their tilt lever does not reach
status: open
opened: 2026-10-03
priority: P2
cost: M
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
