---
id: loft-walls-keyed-per-segment-on-a-declared-carrier
kind: issue
title: loft: a declared continuation's two walls get two NURBS surface keys, so no merge rung can join them
status: parked
opened: 2026-09-25
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
design: true
---


Found by BAND's `subdivided-profile-side-coplanar-walls-gate` rows
(`crates/sweep/tests/band_subdivided_side_walls.rs`,
`lofted_continuation_walls_carry_one_key_per_segment`).

Loft two copies of a square whose bottom side is a declared straight
continuation. The two walls on that side get two distinct NURBS
surface keys (one per segment, from `assemble` in
`crates/sweep/src/loft.rs`), and neither carries a shared
`GeomSource`, so `merge_coplanar_faces` finds no run. Extrude and
revolve share one key across the same continuation (the cosurface run
structure in `crates/sweep/src/swept.rs`). The loft does not, so the
author's declaration is lost here.

This is latent today: the boolean refuses the lofted body's spline
edges before its maximal-faces gate is reached. It becomes a
`NonMaximalFaces`-shaped wall, or a numeric-only coplanarity no rung
can merge, on the day NURBS edges are admitted. The fix has one of two
shapes. The skin can build one surface over a same-carrier run in
every section, or the walls of such a run can carry a shared
`GeomSource` so the declared rung joins them.

## Parked on the D10 hold (CARVE, 2026-10-06)

The row is about a DECLARED straight continuation and the declared merge rung that would join its walls. Declared continuations are on the hold's ground, and D10 retires the declaration in favour of structural identity (one construction, canonical carrier forms), so the fix's two shapes are both questions D10's stage 4 answers. It waits on that build.
