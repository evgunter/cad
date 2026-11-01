---
id: two-discs-kissing-externally-refuse-classification-invariant
kind: issue
title: Two coplanar discs kissing externally refuse ClassificationInvariant, declared or not
status: closed
opened: 2026-10-10
priority: P2
cost: M
closed: 2026-10-12
---


Two unit discs extruded `z ∈ [0, 1]`, centred at `(0, 0)` and
`(2, 0)`, touch along one ruling of their walls. Their union refuses
`ClassificationInvariant` ("odd number of surviving crossing records
at a vertex pair") in each of these cases:

- undeclared, with INTENT stage 4 PR E (the caps glue on Zero and the
  walls' tangency is witnessed);
- with the caps declared flush, and the walls declared `Tangent`, on
  main before PR E as well.

`ClassificationInvariant` names a kernel defect, not a door. On main
before PR E, the undeclared union and the caps-only union stopped
earlier, at `CurvedPierceUnsupported`, and that stop hid the defect.

The pin is
`crates/sweep/tests/wedge_end_doors.rs`
`a_boolean_that_would_kiss_a_curved_face_refuses_typed_at_the_op`,
in its external kiss row. The declared route for the wedge end it
would leave is `work/tang/declared-cusps-second-order-wedge-arm.md`
item 3. What this issue needs is a typed door, in place of the
invariant, wherever that arm does not reach.

**Cause (PR 4704's dual review, measured).** This is
`recl::resolve_edge_edge`'s declared-`Tangent` short-circuit, which
marks a germ whatever the op. Each wall pair leaves the kiss line on
the same side, so PR 4704's narrowing of the short-circuit to
same-side pairs leaves this row as it was. With the short-circuit
removed, the union builds a two-solid `Assembly` recording only the
ruling's two end vertices. What a kiss's union should produce is the
open question on
`work/contacthold/a-same-side-tangent-touch-refuses-at-every-op.md`,
which carries this row's fix.

## Closed (2026-10-12)

`contacthold/same-side-tangent-touch` deletes the short-circuit, per
Ev's ruling on `[ev]` PR 4723. Every op on the pair now answers: each
∖ returns its disc, ∩ is `Empty`, and ∪ is a two-solid `Assembly` at
2π carrying the edge–edge record of the two seam rulings. The row is
`wedge_end_doors::two_discs_kissing_externally_touch_at_every_op`.
