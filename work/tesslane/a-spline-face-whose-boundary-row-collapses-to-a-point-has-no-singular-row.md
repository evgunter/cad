---
id: a-spline-face-whose-boundary-row-collapses-to-a-point-has-no-singular-row
kind: issue
title: a spline face whose boundary row collapses to one point lists no singular point, so its lattice CDT can join that point to two nodes of one column
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found by PR 7049's §5 sweep (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`).

## The shape

The T1 triangle was a corner at a chart's singular point and two
corners on one column through it, on a ruled chart triangulated in its
raw chart: three points of one straight line. The chart lane's analytic
arms cannot make it (the cone triangulates in its development, where a
ruling is an exact line through one apex point). The spline arm
triangulates in the lattice plane (`trimmed::CdtPlane::Lattice`) and
lists no singular points (`ChartRow::Spline`), so a NURBS face whose
boundary row collapses to one point (a spline cone or a spline pole)
would put that point on every column, and a triangle of that point and
two nodes of one column is possible. Whether it is of no area depends
on the column being straight; a degree-1 ruled spline's is.

## Not measured

No body is known. The grep of `crates/mesh` and `crates/geom-brep` for
collapsed or degenerate spline rows, spline poles and spline apexes
found no fixture, door or row building one. It did not search the doors
most likely to make one: STEP import (a `B_SPLINE_SURFACE` with a
degenerate edge) and loft (a section shrunk to a point). `[mesh.winding]`'s
refusal at the join is the backstop if one arrives. The settling run is
a body from either door, meshed, with its triangles' corners read.
