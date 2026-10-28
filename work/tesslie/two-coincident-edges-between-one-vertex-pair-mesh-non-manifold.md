---
id: two-coincident-edges-between-one-vertex-pair-mesh-non-manifold
kind: issue
title: Two coincident edges between one vertex pair mesh as one segment of four triangles, which check_mesh refuses
status: open
opened: 2026-10-02
priority: P2
cost: M
---



## What

Two solids touching along a line leave a body with two coincident edges
between one vertex pair. That shape is a legal pseudomanifold edge, and
3′ passes it. Both edges are straight, so each one's chord is the single
segment between those two vertex ids. The four faces meeting there each
use that segment, and `mesh::validate::check_mesh` refuses the mesh:
`NonManifoldEdge { count: 4 }`.

The tessellator's own census, `unpaired_chord_segment`
(`crates/mesh/src/tessellate.rs`), counts two uses per chord carrying a
segment. It therefore accepts this mesh. So "the mesh of the body" is
not "a closed 2-manifold mesh" for this body, and the two checks answer
differently on purpose. The census reports what it checks; `check_mesh`
stays the definition of watertight.

## Witnesses

From `crates/editor-core/tests/union_pinch_member_order.rs`
(`DOUBLED_EDGE`), on branch `tang/pinch-union-order`:

- X ∩ plate and plate ∩ X, where X = `block((-1,4),(-1,3),0.5,2.5)` − p1
  − p2, with p1 = `block((1,1.5),(-2,1),0.3,2)` and
  p2 = `block((1.5,2),(1,4),0.27,1.73)`. The result is two L-prisms
  touching along x = 1.5, y = 1, z ∈ [0.5, 1]. 3′ passes, the volume is
  the closed form, and `check_mesh` refuses.
- The TANG reviewer reports the same refusal for X, X − P, X ∩ P and
  P ∩ X in the side-face poses of that fixture.

## Owed

Decide what the mesh of a pseudomanifold edge is:

- coincident ids, which a consumer gets as a non-manifold mesh, as now;
  or
- one id per sheet, which makes it a 2-manifold mesh with coincident
  positions, a combinatorial distinction `check_mesh` already admits.

Then make the census and `check_mesh` agree on that answer.

## Measured (JOIN's pinch unit, base `f9bf3bca`)

With `tessellate` at δ = 0.05 added to `outcome`, `SOUND` bodies of two
JOIN batteries panic at the cross-face census (`tessellate.rs`,
`unpaired_chord_segment`: "chord segment … is an edge of 4 face
triangles"):
- 18 lines of `join_pierce_runs_sweep::pierce_runs_battery`, e.g.
  `edge i=3 j=1 psi=0 pc S`;
- 86 lines of `corner_pairs_battery`.

Each holds `v` as one vertex, and no face passes it twice. The lines
are identical on main and on the pinch unit's head, so they are this
row's class, not a pinch's.

## Measured: the split's zero-area ring (`cleave/keel-slit-ring`, PR 5350)

The box `[0, 4] × [0, 4] × [−1, 1]` less a V-trench along x over
`x ∈ [1.5, 2.5]`, split at `z = 0`
(`crates/topo/tests/a_keel_wrapped_at_both_ends_splits_as_a_ring.rs`,
row "closed at both ends"). The side above carries the keel as a ring of
two coincident straight edges between one vertex pair, and passes tiers
1–3 and 3′. `tessellate` then `check_mesh` refuses
`NonManifoldEdge { count: 4 }` at that pair, at chordal 5e-3 and 0.1,
either normal; `signed_volume` is exact (15.7). The one-end slit (the
trench over `x ∈ [1.5, 4.5]`) meshes clean: its two edges end at two
copies of the open end.

No public door runs `check_mesh` (the STL writer does not), so the
ring's mesh, and its STL, are non-manifold without a word.

Curved, measured on the same branch before it stopped building the
curved ring (the split now refuses it, `CurvedSlitTrace`): the curved
ring's walls side gave `MismatchedWinding` at the V's tip on the plane
and at `+0.6 ε`, and passed at `−0.6 ε`. The curved one-end slit, which
main builds, refuses loudly: its walls side refuses `tessellate` at
every δ from 1e-3 to 0.1, `PinchWedge` with the V turned −20° and
`Triangulation` turned 160°
(`crates/sweep/tests/a_curved_keel_in_the_plane.rs`'s fixture).

## Moved to TESSLIE (the mesh answers Ok, or panics, where it should refuse or mesh right) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
