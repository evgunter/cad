---
id: the-glue-doors-tangency-swallows-its-escalations
kind: issue
title: The glue door's tangency read swallows every refusal, escalations included, so an in-band tangency surfaces later as CurvedBooleanUnsupported
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

glue::tangency tests the witness lane with .is_ok(), discarding an escalation; the stage that meets the pair then refuses CurvedBooleanUnsupported, naming a declaration recourse D10 retires. An in-band tangency should escalate typed at the door.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

## Measured (PR 4831, 2026-10-12)

Still swallowed: the claim-free door treats any refusal of
`verify_tangency` or `route_rim` as no tangency. Propagating
`TangencyFailure::Refused` moved five sweep rows to door refusals
(`mate7a_torus_rest` ×2, `a_ruling_lying_on_a_wall`,
`review_contact_edge_must_carry_r1_probes`, `offer_rows::every_sphere_offer_passes_just_below_it`),
and not all were in-band tangencies: the G1 tube chain refused
`contact_tangent_on_1` at margin 1.21, the witness on the surface but its
far side resting on a loose span sag. The fix has to tell a genuine
in-band reading from an uncertifiable one before it escalates.
