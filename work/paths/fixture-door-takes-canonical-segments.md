---
id: fixture-door-takes-canonical-segments
kind: unit
title: The dev-only RawLoop fixture door takes canonical segments; ProfileVertex retires; fixtures migrate through a helper forwarding to arc_to(Bulge)
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [canonical-segment-type-in-profile]
---


Split out of unit 1 at spec time, so that unit 1's byte-identity claim
stays reviewable apart from the mechanical churn: about 1355
`ProfileVertex::new` sites in about 290 test files. What this unit does
(Ev, #3218 q4):
- `RawLoop` (behind `raw_door!`, compiled only under test/test-support)
  takes canonical segments, so a fixture can build a one-segment
  circle;
- `ProfileVertex` retires;
- fixtures written with a bulge go through a test-support helper that
  forwards to the algebra's `arc_to(Bulge)` lowering and has no
  arithmetic of its own.

Afterwards the only bulge surface left in the kernel is `arc_to(Bulge)`.
Move one crate per commit, and keep the rewrite scriptable.
