---
id: a-cut-rod-inside-an-equal-rod-with-vertices-on-its-seam-refuses-single-site
kind: issue
title: A cut rod inside a longer equal coaxial rod, its rims' vertices on the long rod's wrap edge, refuses SingleSiteSectionLoop count 2 in one operand order
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Found by `tangrim/one-segment-ellipse-rims`'s outcome probe (scratch,
not committed), on `main` `7979553590` and that branch alike.

## The pose

`A` is a one-segment cylinder `r = 0.3`, `z ∈ [−3, 3]` along its axis,
intersected with the slab `z ∈ [0, 1]`; `B` is the same cylinder uncut.
`A` lies inside `B`, its wall on `B`'s wall, and its two rims, closed
edges with their vertices at azimuth `0`, lie in `B`'s wall on `B`'s wrap
edge (the ruling at azimuth `0`). Built straight (circle rims) and with
both cylinders tilted `0.3` or `0.7` rad about `x` through the origin
(ellipse rims).

## What it reads

`A ∪ B`, `B ∩ A` and `B ∖ A` refuse
`Join(BooleanJoin(SingleSiteSectionLoop { count: 2 }))` at rest and
turned, at every tilt, at ε = 1e-6, 1e-9 and 1e-12. The other order of
each builds `SOUND` (within the tilted rod's own volume pad), and
`A ∖ B` is empty. With `A`'s vertices at `π`, or in the two-arc form,
every op builds.

The raising site is `boolean/join.rs`'s single-site check; which cell
pair `closes_at_its_site` reads there, and why only one order reaches
it, is not yet measured. `count: 2` is two one-site loops refused,
one at each rim.
