---
id: plane-cylinder-section-reads-its-gap-at-the-stored-origin
kind: issue
title: plane_cylinder_section reads pc_parallel_gap at the cylinder's stored origin, which its tilt lever does not reach
status: open
opened: 2026-10-03
priority: P2
cost: M
---


## What

`plane_cylinder_section`'s axis-in-plane lane
(`plane_cylinder_ruled_section`, `crates/geom-brep/src/intersect.rs`)
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

## The fix's shape

The classifier takes an `ExtentBall` and reads the gap at the foot of
its centre on the axis, levering the tilt from there — the witness
lane's construction, moved into the classifier so every caller gets
it. Found reading only, by the
`tangent-locus-re-meters-the-section-classifiers-tangency` lane;
nothing measured.
