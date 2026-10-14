---
id: a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest
kind: issue
title: A split through a seam vertex nearly along the axis leaves a wall face of in-band width, which refuses only at rest, as VolumeUncomputable
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [4861, 4335]
---



## What

Found by PR 4861's at-rest pins (`crates/sweep/tests/section_ellipse_seam_poses.rs`,
`COIN_FLIP_1E6`). It reproduces on main `98353b4835` (CLEAVE's census
probe, the same 18 poses).

At ε = 1e-6, the unit cylinder is split by planes through a seam vertex
`(1, 0, 0)` or `(−1, 0, 1)` with normal `(0, 1, 3e-5)`, the vertex
`c·ε` off along the normal for `|c| ≤ 0.99`. The split returns `Ok`.
One side's wall face is a sliver between the seam ruling and the
plane, at most `θ·h = 3e-5` wide, and tier 3 refuses that side:
`VolumeUncomputable { source: Face { source: Escalated { predicate:
"props_quad_face_extent", margin ≈ 7.5e-6, check: Extent } } }`. The
face's mean width is in band (between ε and Kε = 1e-5).

The same family at ε = 1e-9 and `θ = 1e-4` refuses earlier, at reduce
(`SliverSector`). At `θ = 3e-5` and ε = 1e-6, nothing in the split
reads the sliver, and the reading surfaces only at rest, as the volume
quadrature's escalation.

## Owed

Under Ev's PR 4335 rule, an in-band reading of the input is `Escalated`
with a lever. Decide where the split reads a face of in-band width,
which is likely reduce's sliver reading, so that the refusal is typed
there with its lever, not left for the at-rest gate to end. Then
re-baseline `COIN_FLIP_1E6`.
