---
id: two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner
kind: issue
title: Two lumps touching along an edge against a partner flush with a face of each refuse PinchRaysInAFace in some orders
status: closed
closed: 2026-10-31
pr: 6170
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

## Cause

The ring orders the two lumps' germs along the line correctly
(`vtxfac::germ_order`'s tie, `tie_breaks_after`). The refusal was the
join's: whichever order first closes a cycle through the tie's corner
of no angle walls off the sliver between the two lumps' copies of the
line, a run that encloses no area either way round, and the ring lane
read that zero as a degenerate winding (`join::ring_winding_order`),
which `pinch_ray_refusal` typed. Which operand order met the sliver
first on the ring lane, rather than later on the outer lane, decided
which orders refused. With that fixed, `x ∪ y` over the S and the
wedge above kept the sliver as a face of no area beside the kept lump's
flush face, a plug of two faces in the partner's face's hole, and the
coplanar merge took the sliver as its survivor because only faces the
ring borders read as nested (`merge_faces::outermost_survivor`):
`Merge` `SameFace`.

## Closed

2026-10-31, PR 6170. The ring records each tie as the strut halves
facing its two germs, the first clockwise first
(`PierceRingRecord::ties`). A ring run whose winding reads zero and
whose end faces a tied germ winds as the tie's order places the chord
and the copy it closes against apart (`join::tie_winding`): counter-
clockwise where the chord leaves the run's start along the first germ.
The coplanar merge's survivor reads a member as nested when it shares
an edge with a member in a ring's hole, so a plug of several faces lies
in its hole. Every cell builds in both orders, tier 3 and 3′, as the
closed form says, with one edge along the line per creased sector of
the result's material about it, at ε default, 1e-6 and 1e-12, at rest,
turned and flipped and turned. Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_flush_partner_over_two_lumps_touching_along_an_edge_builds`
(the three lumps, both partners, six ops) and
`::a_flush_partner_crossing_the_slit_intersects_it_in_either_order`;
`union_flush_onto_edge_contact::a_face_holding_the_pinch_line_ties_the_pierces_germs`'s
slopes 0.5 and 2, which refused `PinchRaysInAFace`, build too.
