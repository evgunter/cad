---
id: a-spline-face-has-no-subtract-or-intersect-lane
kind: issue
title: Subtract and intersect refuse any spline (loft or sweep) face whose box may meet the other operand, at the revert roster: NURBS is on the union's roster only
status: dispatched
opened: 2026-11-03
priority: P0
cost: H
refs: [torus-onto-the-subtract-and-intersect-roster, a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form, the-spline-rider-arm-is-pinned-by-no-fixture]
branch: splice/nurbs-revert-roster
---


Filed 2026-11-03 from the curved-boolean bookkeeping pass
(`work/issues/curved-demo-joins-build.md`). Klein wall 4 has pinned this
refusal for weeks, and no row owned it.

## What

`crates/topo/src/boolean/ops.rs:904`: for any op but ∪, the front door
runs `reduce::first_unsupported_pair` with `reduce::revert_arm_exists`
(`crates/topo/src/boolean/reduce.rs:419`), which lists plane, cylinder,
sphere, torus and cone and leaves out `Nurbs`. The union's roster,
`boolean_arm_exists` beside it, has `Nurbs`. So once a loft or sweep
body's spline face may meet the other operand (box overlap), every ∖
and ∩ refuses `CurvedPairUnsupported { op: Some(_), site: RevertRoster,
kind: Nurbs, .. }`, before SPLICE's union lanes are reached. A declared
pair cannot cover it (the comment at `ops.rs:897`).

Witnesses:
- klein wall 4, cutting the flare where the neck passes through
  (`demos/tour/src/klein.rs` `wall4_pinned`: `{ op: Subtract,
  operand: B, kind: Nurbs, other_kind: Plane }`);
- SPLICE's saddle, both orders of ∩ and ∖
  (`a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form`);
- PARTING's spline-face arm, which no fixture reaches because this
  roster refuses first (`the-spline-rider-arm-is-pinned-by-no-fixture`).

Cutting a swept groove or a lofted pocket into a part is everyday
authoring, so the row is P0.

## What the taker owes

The torus took this step in `torus-onto-the-subtract-and-intersect-roster`
(PR 3416, M) once its union lanes existed: measure ∖ and ∩ over the
union's fixtures with the kind admitted, find the classes that return
wrong bodies, gate or fix them, then admit the kind. Do that for `Nurbs`
over SPLICE's union rows. The docs on `revert_arm_exists` and on
`BooleanError::CurvedPairUnsupported` (`crates/topo/src/boolean/mod.rs`,
"NURBS: no edge×NURBS-face crossing layer at all") predate SPLICE's
crossing lanes, so re-read them first. Priced H rather than the torus's
M because the spline union lanes behind it are still being built. Expect
admission to be staged by partner kind, as the union's was.
