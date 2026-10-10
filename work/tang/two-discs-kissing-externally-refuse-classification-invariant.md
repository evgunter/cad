---
id: two-discs-kissing-externally-refuse-classification-invariant
kind: issue
title: Two coplanar discs kissing externally refuse ClassificationInvariant, declared or not
status: open
opened: 2026-10-10
priority: P2
cost: M
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
