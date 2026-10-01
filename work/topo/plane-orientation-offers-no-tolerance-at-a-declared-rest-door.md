---
id: plane-orientation-offers-no-tolerance-at-a-declared-rest-door
kind: issue
title: topo: the plane orientation rung offers no tolerance at a declared Rest door, where a smaller one would pass, because the decision carries no read
status: open
opened: 2026-09-30
---


(TOPO, PR 3513's fourth fix pass: a withdrawal's residue.)

## What

`BooleanDecision::PlaneOrientation` (the plane ladder's orientation
rung: whether two coincident planes face the same way, the normals'
cosine at the door's arm) ends on the corner's lever alone
(`LeverPass::ByArm`, `BooleanDecision::ending` in
`crates/topo/src/boolean/refusal_routes.rs`). Its offer was withdrawn
because the offset rung asks next, and an undeclared coincident pair
refuses there at every tolerance: executed in
`boolean::refusal_routes::offer_rows`' `planes_facing_at_a_short_arm`
(re-run just below the value its margin gives, the same raise refuses
`UndeclaredCoincidence`).

At a declared `Rest` door (`PlaneDoor::OnPair(Spent(Rest))`, the
declared rung that bridges an in-band residue) a smaller tolerance
that decides the orientation would go on to the bridge, so the offer
there is likely true. The decision carries no read, so it cannot tell
the two doors apart and offers the tolerance at neither. D4 ¶1 (i)
permits that; it does not require it. Not executed at the declared
door: this row's first step is the case that would show it.

## Repair shape

Carry the door's read on the decision (`PlaneOrientation(DeclarationRead)`,
as `Coincidence` does, from the `PlaneDoor` `plane_identity` already
receives), end the declared-`Rest` read sized from `CORNER_SENSE`, and
add a case to `offer_rows` that raises it through
`verify_declared_contacts` and executes the offer.
