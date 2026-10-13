---
id: a-near-aligned-capped-tube-refuses-by-operand-order
kind: issue
title: A sphere-capped tube turned within a hair of aligned refuses at the ruling door with the tube as operand A and escalates a contact vertex's containment with the cap as operand A
status: closed
opened: 2026-10-11
priority: P3
cost: M
branch: contacthold/glue-door-escalates
pr: 4918
closed: 2026-10-13
---


Found by stage 4 F (`declared-pairs-retire`) when the declared `Seam` on
the walls went.

## What

`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`the_sphere_capped_tube_turned_within_a_hair_of_aligned`, at the rung
`apart = 2√(zero · escalate)`: with the tube as operand A the union
refuses `CurvedPierceUnsupported { operand: A }` on the tube's seam
ruling, which is what the neighbouring rungs refuse in both orders. With
the cap as operand A it refuses `Escalated`, predicate
`bool_contact_vertex` (containment, margin about 6.3e-9). Both orders
refuse, so nothing silently builds. But which refusal the author sees
depends on the operand order, and D10 defines a pair's verdict as the
same in either operand's frame or the band refuses. While the walls
were declared `Seam`, both orders escalated on `seam_cover_*`.

## Owed

Read why the contact-vertex containment escalates only with the cap
first. Then either have both orders meet the ruling door, or both meet
the escalation. The row pins each order's refusal as it stands and names
this file.

## Resolved by PR 4918

The glue door now escalates a refusal no reading of a pair settles
instead of reading it as no tangency. At this rung the tube's wall and
the cap's sphere face share the rim, and the rim routing reads the seam
cover's order in band, so both operand orders escalate at the door on
`seam_cover_order` (margin about 6.3e-9), as they did while the walls
were declared `Seam`. The row pins both orders on that refusal.
