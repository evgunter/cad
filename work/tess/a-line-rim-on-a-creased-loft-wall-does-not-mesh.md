---
id: a-line-rim-on-a-creased-loft-wall-does-not-mesh
kind: issue
title: A line rim on a loft wall creased by another section's sub-arcs does not mesh: its chord points straddle the crease
status: open
opened: 2026-11-04
---


Filed by the PCERT main-reds lane (`pcert/main-reds-e2-e5`).

## Measured

A degree-1 loft of a square whose +x side has bulges `[0, 1, 0]` at
z = `[0, 1, 2]` builds and is at rest, but `mesh::tessellate` (δ 1e-3)
refuses it:

    UnsupportedCurve { edge: EdgeKey(2v1), note: "trimmed NURBS face
    boundary crosses a C⁰ crease of the face between two chord points —
    only an arc rim's chord schedule lands on the crease it crosses" }

Edge `2v1` is section 0's bulge-0 side, a line rim whose image is an
`IsoLine` (`p0 = (0, 0)`, `pl = (0.5, 0)`) on the slab wall's row
`v = 0`. The wall's `u` carries the semicircle's sub-arc joint as a C⁰
crease, and the line's chord points (uniform in its parameter) straddle
it. The refusal is in `mesh::trimmed`, at the crease check in the
boundary walk. `crate::chords` places a point on a crease only for an
`IsoArc` image.

The loft `[0, 1, 0.3, 0]`, which builds at degree 1 since the same lane
let a refined arc rim be read, meets the same refusal. It is the third
case of `sweep degree1_loft_slabs::an_interior_arc_refined_by_its_neighbours_lofts`,
which therefore meshes only its first case.

## Fix

An `IsoLine` image across a crease needs a chord point at the crease:
the line's parameter where its affine `u` channel meets the crease's
`u`. That is exact for an iso line, as the arc rim's joint is for its
map.

## It depends on δ (PR 6528's review, 2026-11-04)

Whether a line rim's chord points straddle a crease depends on the
chord count, so the refusal comes and goes with δ. Measured on the
three degree-1 lofts of
`sweep degree1_loft_slabs::an_interior_arc_refined_by_its_neighbours_lofts`
(✓ meshes, ✗ refuses as above):

| bulges | δ 1e-3 | δ 2e-3 | δ 5e-3 |
|---|---|---|---|
| `[1, 0.2, 1]` (no line rim) | ✓ | ✓ | not measured |
| `[1, 0.2, 0, 0.7, 1]` | ✓ | ✗ | ✗ |
| `[0, 1, 0.3, 0]` | ✗ | ✓ | ✗ |

A body that meshes at one δ and refuses at a neighbouring one is the
crease gap, not the body. The row meshes its first case only.
