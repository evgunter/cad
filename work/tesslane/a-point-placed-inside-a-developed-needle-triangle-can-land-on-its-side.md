---
id: a-point-placed-inside-a-developed-needle-triangle-can-land-on-its-side
kind: issue
title: a point the cone arm places inside a kept triangle can land on its side when the triangle is thinner than the development's direction cut
status: open
opened: 2026-11-17
priority: P3
cost: M
---



Found by the fix pass of PR 7043 (the clearance and winding unit), on a reviewer's question; measured, not cured.

## What

`mesh::trimmed::TrimmedFace::inside_point` is the point the ear cure and the certificate refinement insert in a kept triangle. In the cone's development it is the centroid of the developed corners, read back to the chart (`geom::Azimuth::near`), and it enters the next attempt's CDT developed again, through `developed`'s cut (26 significant bits of the direction, 27 of the radius, "under `2⁻²⁵` of its radius"). The round trip is not the identity, so on a triangle thinner than the cut the point need not be inside it.

## Measured

`trimmed::tests::a_point_placed_inside_a_kept_triangle_is_strictly_inside_it`, held to spade's exact point location.

- In the chart (4,050 needle triangles): strictly inside whenever the height is at least four f64 epsilons of the largest coordinate; the 336 misses are all under half of one.
- In the development (21,870 needle triangles, half-angles 0.02, 0.5 and 1.4, both nappes): strictly inside whenever the developed height over the longest side is at least `2⁻²⁵` of the farthest corner's radius. Under it, 744 misses, the thickest at `7.1·10⁻⁹` of the radius, and 586 of them on a side or a vertex of the triangle.
- The pinned one: half-angle 0.5, chart corners `(6, 0)` (the apex), `(6, 50)` and `(6.00000001, 50.45)`; its point `(6.000000008625545, 33.4833…)` lands on an edge.

## What it costs

A point on a side used to be dropped and the face rebuilt (which ended in the refusal of whatever asked for the point, after the retry budget). Since PR 7043 a point of the mesher's on a side, or at a vertex, refuses `Triangulation` at once. No body is known to reach it: both reviewers' censuses and the mesh suite show no `Triangulation` refusal, needle funnels at δ `10⁻⁶` included. It needs a kept ear, or a triangle with a side a neighbour holds, whose developed height is under `3·10⁻⁸` of its radius: two boundary points within that angle of one ruling.

## Owed

Either a point that is inside by construction where the CDT runs (chosen in the developed plane and carried there, not read back through the chart), or the refusal named for what it is. Unmeasured: whether any body reaches it.
