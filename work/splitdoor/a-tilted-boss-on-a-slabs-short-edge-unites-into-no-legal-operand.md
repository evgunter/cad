---
id: a-tilted-boss-on-a-slabs-short-edge-unites-into-no-legal-operand
kind: issue
title: A boss turned off a slab's plane, its top holding the slab's short edge, unites into a body no later Boolean takes as an operand
status: open
opened: 2026-10-30
priority: P2
cost: M
---

Found by PR 6053's review (branch `splitdoor/arm-vs-reach-review`,
`crates/sweep/tests/review_arm_vs_reach_probes.rs` `review_tilted`).
Predates PR 6053.

## The pose

`A` is the slab of the quad `(0, 0), (0.5, 0), (0.5, 0.5), (0, s)`,
`z ∈ [−0.5, 0]`, `s = 300ε`. `B` is the quad
`(0, 0), 0.4·(cos θ, −sin θ), (0.35, 0.25), s·(cos 45°, sin 45°)` from
`z = −0.3` up to `0`, turned by `φ = −0.05` about `y` through the
origin, so `B`'s top holds `A`'s short edge.

## What

`A ∪ B`, `B ∪ A` and `B ∖ A` return a body that is no legal operand:
uniting it with a far brick refuses `bool_plane_parallel` in band
(margin `1.7e-9` at the default ε), though `A` and `B` each unite with
the same brick. At `θ = −10⁻³`, `A ∖ B` also fails tier 3′. The
differential reads these as `operand=false` (and `t3p=false`).

Main ships it at `θ = ±10⁻³`; at larger `θ` main refuses earlier, on
the arm reading PR 6053 retired, so on that PR's head it ships at
`θ = ±10⁻²`, `±3·10⁻²` and `10⁻¹` too. Which of the result's plane
pairs reads parallel in band, and whether the union should have glued
or parted them, is unmeasured.

## To do

Measure the in-band pair (the two faces, the margin, its lever) on
the union's result, then decide whether the Boolean should refuse the
union typed at that reading or build a result whose faces read
definite against each other.
