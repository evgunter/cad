---
id: a-spline-vertex-on-two-collapsed-sides-keeps-its-polygons-copies-unwalked
kind: issue
title: a spline vertex on two collapsed sides keeps its polygon's copies with no point between: the corner's walls mesh only as coarse as the spline certificate allows
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Filed by TESSLANE's `tesslane/collapsed-side` lane. The trimmed lane walks a
spline face's collapsed side between the two copies of its vertex at the
face's chord step (`crates/mesh/src/trimmed.rs`, `SidePoints`,
`CollapsedSides::points`). A vertex whose stored point is the point of TWO
collapsed sides (two adjacent sides of the knot rectangle, both one point,
meeting at its corner) is not walked. Its loop arrives on one side and leaves
on the other, so the chart path between its copies turns at the corner, and a
straight piece between them would leave both sides and put a copy where the
surface is not the vertex. That vertex keeps its two copies and no point
between them, and so does a vertex the band leaves undecided against
another side. This is sound: each copy is on a side, and the certificate
refuses typed where δ is not met. But it is the shape that, on one side, meshes
only at a coarse δ (the unit's measurement: the v-degree-2 pinned wall refused
`CertificateExceeded` at every δ ≤ 3e-2 with the copies alone).

No door builds such a face today. The loft collapses `u` columns only, one per
pinned corner, and step import has no fixture with a collapsed side. A net
with two collapsed adjacent sides, imported or described, would reach it.

The cure would walk the copies through the corner: along the arriving side to
the corner `(u_side, v_side)` read off the knot rectangle, then along the
leaving side. That takes reading which side each copy lies on, which `topo`
decides at the band (`SideWalk::Corner` in `pcurves/decision.rs`); the mesh
would read the same decision, as it reads the vertex's incidence on the sides
(`chart_table::at_a_singular_point`).
