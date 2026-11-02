---
id: a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident
kind: issue
title: A split whose plane touches a hole's rim at a vertex from the material side refuses SectionGermsUnordered Coincident: the cap's trace and the hole wall's section leave the vertex along one direction
status: closed
opened: 2026-10-11
priority: P1
cost: M
pr: 5951
branch: splitdoor/hole-rim-tangent-germs
closed: 2026-10-28
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: a plane tangent to a hole's rim at its vertex refuses where the solid cylinder builds: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (2026-10-28, PR 5951)

`splitting/insert.rs` `section_chords` orders germs of different sectors that tie at first order by
how each face's section curves off the shared tangent (`geom_brep::implicit_graph_acceleration`,
the sign read by `enters_material_order2`); a tie at second order still refuses `Coincident`, and a
vanishing gradient refuses `SectionGermFault::Singular`. `splitting/join.rs` orders each run of
coincident crossings by the crossing before it (`touch_order`). Every pose of this row builds at its
closed form at three ε; the order is load-bearing at every h (the wrong one builds a different,
valid-looking topology, now caught by the pins' touch-vertex count). FULL single review
(APPROVE-WITH-FIXES, all fixed).
