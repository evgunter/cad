---
id: a-sign-walk-escalates-on-the-last-rounds-in-band-reporting-reading
kind: issue
title: a sign walk escalates on the last round's in-band reporting-target reading, though its enclosure is sound
status: closed
opened: 2026-10-21
priority: P2
cost: E
parent: the-measurement-door-refines-until-a-predicate-resolves
closed: 2026-10-23
pr: 5628
---


## What

Found by the sweep of `check-7-refuses-the-reporting-budget-on-a-definite-sign`
(the class: a reporting-target question answered inside a walk whose
caller does not ask it). Unmeasured on a witness; read from the code.

`geom_brep::props::quad::Schedule::converged` reads each round's
`props_quad_converged` margin, `target_len − width_len`, where
`target_len` is the REPORTING target `1024·ε`. At a round with a
successor an in-band reading refines; at the schedule's LAST round it
escalates (`PropsError::Escalated { check: Converged }`), because there
a budget refusal could misstate a width that is under the target.

That reasoning is the reporting caller's. `topo::props::sign_walk`
enters the same lanes one round at a time (`RoundWindow::at`), and
`face_flux` turns an `Err` into a walk-ending refusal: the escalation
leaves the face with no enclosure, so check 7 refuses the body. But the
last round's enclosure is sound whatever its margin against the
reporting target, and it is the narrowest the schedule has; a sign
walk whose sign is still open there wants that enclosure, and a sign
walk whose sign it decides is refused for a question it never asked.

## The fix's shape

The same as the parent row's: the in-band last-round reading becomes a
fact the lane hands back beside the enclosure (an `Open` outcome with
the escalation as its `refusal`, no rounds remaining), and the
reporting caller (`RoundOutcome::into_target`) turns it into the error
it is today. Needs a witness first: a face whose last-round width lands
within the band of `1024·ε` (choose ε against a measured last-round
width, as `tcost_k1_budget_exit`'s `early_then_exhausted` does).

## Closed

PR 5628 (measure door unit B). Nothing asks a face the reporting target inside a sign walk any more. The lane is asked a resolution only by a number walk, and a resolution is compared, not decided, so there is no in-band reading to escalate on. A sign walk refines every refinable face until its sign settles or every face is at its floor or work wall.
