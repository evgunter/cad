---
id: a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind
kind: issue
title: a run of merged ruling points whose ruling-side triangle is a needle adds triangles f64 cannot wind
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found building `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`.

## Finding

A run of ruling points the cone's development merges (`trimmed::Run`)
is anchored by its neighbour along the ruling: the triangles between the
run's points are cut from the CDT triangle on the polygon side to that
neighbour, and fan about that triangle's third corner. Where that
triangle is a needle along the ruling, the added triangles are the
run's gap wide and the needle's width over its length high, and f64 does
not resolve their winding: the face refuses `UnresolvedWinding`.

Measured at ε `10⁻¹²`, the only ε at which `split_edge` admits the gap:
the cone `(0, 0), (1, 0), (0, 1)` revolved about `y` (half-angle 45°,
through its apex), one ruling split `10⁻¹⁰` of its range from its rim
end, placed obliquely. The run is the split vertex and the ruling's rim
end; its neighbour along the ruling is the apex, a whole ruling
(`1.41`) away. At δ 0.1 and 0.01 the face meshes. At δ 0.001 the
lattice's rows stand close about the apex, the triangle on the side from
the run to the apex takes a node there for its third corner, and the
added triangle reads

    height 5.15e-15, allowance 9.14e-15

(`validate::Unresolved`; corners `1.4·10⁻¹⁰` apart and a node `1.41`
away, about `5·10⁻⁵` off the ruling). The same on a full turn and on a
wedge of 0.5, with the split `10⁻¹⁰` from the rim end or twice `10⁻¹⁰`
apart at 0.99 of the range: 11 of the 12 such cells probed at δ 0.001.
Before the unit these cells refused `DevelopedMerge` at every δ.

Pinned: `split_rulings::a_run_anchored_on_a_needle_triangle_refuses_unresolved`.

On the draft cones of the unit's spec (the wall and the solid, half-angles
0.1° to 26.6°, gaps down to `10⁻¹¹`) no added triangle comes within
163 times the allowance.

## The cure seen

Every triangle of the run's fan can carry the added triangles: cut
along any fan edge, the triangles before it take the run's first point
and those after it the last. The lane takes the fan's first or last
edge, by which neighbour is along the ruling. Taking the fan edge that
stands most squarely on the ruling in the development makes the added
triangles as high as the fan allows, and is a choice read from developed
coordinates where today's is read from the polygon's order.
