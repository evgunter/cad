---
id: two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner
kind: issue
title: Two lumps touching along an edge against a partner flush with a face of each refuse PinchRaysInAFace in some orders
status: dispatched
opened: 2026-10-26
priority: P1
cost: H
refs: [a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece]
branch: cleave/two-lumps-pinch-rays
---

Found by the flush-partners-at-a-slit lane (`cleave/flush-partners-at-a-slit-build`),
the same on main `be2b7a139c`, at ε default, 1e-6 and 1e-12, at rest,
turned, and flipped and turned.

## What

`x` is the box `[0, 1]³` ∪ a second lump touching it along the edge
`x = 1, z = 1`: the box `[1, 2] × [0, 1] × [1, 2]` (an S), the wedge
over the `(x, z)` triangle `(1, 1), (2, 1), (2, 2)` (above, its bottom
coplanar with the box's top), or the wedge over `(1, 1), (2, 0), (2, 1)`
(beside, the tops coplanar). `y` is the box
`[−0.5, 2.5] × [−0.5, 1.5] × [1, 1.5]` or `× [0.5, 1]`, flush with a
face of each lump on the line's plane. These refuse
`PinchRaysInAFace` (`vtxfac::germ_order`, `join::pinch_ray_refusal`):
the pierces of `y`'s face at the lumps' shared point read germs along
one direction that the ring cannot order:

| lumps | `y` | refuses |
|---|---|---|
| S, wedge above | either | `x ∪ y` |
| wedge beside | above | `x ∪ y`, `y ∪ x` |
| wedge beside | below | `x ∪ y`, `y − x`, `y ∩ x` |

The other ops build, tier 3 and 3′, as the closed form says
(`a_keel_on_the_floor_subtracts_as_a_slit::a_flush_partner_over_two_lumps_touching_along_an_edge_builds`
and `::a_flush_partner_crossing_the_slit_intersects_it_in_either_order`
pin the S and wedge-above cells and the beside wedge's `y ∩ x`).
`PinchRaysInAFace` is typed, but an operand order that refuses where
the other builds is a kernel defect (DESIGN), not a gap: the ∪ cells
are P1 (ruled 2026-10-27 on PR 5812).

## Owed

Build these as the material decides, so both orders of each op agree.

