---
id: a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident
kind: issue
title: A split whose plane touches a hole's rim at a vertex from the material side refuses SectionGermsUnordered Coincident: the cap's trace and the hole wall's section leave the vertex along one direction
status: open
opened: 2026-10-11
priority: P3
cost: M
---


Found by the sweep of `a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it`
(branch `cleave/wrap-edge-split`). Not a wrap-edge defect: the two-arc
hole refuses alike.

## What

The block `[-2, 2]² × [0, 2]` with a circle hole of radius 1 about the
z axis, its vertex at `(1, 0)`, split by a plane through the bottom
vertex `(1, 0, 0)` whose normal leans toward it, `(h, 0, 1)`: the
plane's trace on the bottom face is the line `x = 1`, tangent to the
hole's rim at the vertex, and the material lies on both sides of the
trace there. `topo::split` refuses
`Reduce(SectionGermsUnordered { vertex, fault: Coincident })` at
h = 0.4, 1.0 and 1.6, at ε = 1e-9, 1e-6 and 1e-12, for the one-segment
hole (either winding) and for the two-arc hole (vertices at `(±1, 0)`)
alike.

The same poses on the solid cylinder, whose cap the trace touches from
outside the material, build (`a_split_through_a_wrap_edges_vertex.rs`,
pose "round, leaning to the vertex", and the pinched and disc poses
beside it). The hole's poses that cross its rim (`β ≠ 0`) build too and
are pinned by
`a_split_through_a_hole_walls_strut_vertex_keeps_one_face_where_the_piece_goes_round`.

## Reading

The refusal's text: "the split plane leaves two faces round a vertex
along one direction". Here the two directions are the trace on the
bottom face and the hole wall's section ellipse, which is tangent to
the trace at the vertex: an exact tangency of decidable order, not two
section vertices within ε of each other, which is what `Coincident`
was written for (`crates/topo/tests/split_through_an_apex.rs`
`germs_within_the_band_or_on_one_line_refuse_typed`). Unmeasured: whether
ordering the two germs one order down (the curvature of the ellipse
against the straight trace), as the conic departure does
(`neighborhood.rs`, `enters_material_order2`), is all it needs.
