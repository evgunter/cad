---
id: a-placement-cannot-turn-about-a-point-or-an-axis
kind: issue
title: An instance's values cannot turn a copy about a pivot point or an axis pose, so the tour spells the turntable as trig
status: open
opened: 2026-10-02
priority: P3
cost: M
design: true
---


Found by SHOW's `bench-on-a-gauge` (PR 3840), writing a turntable the
way a user would.

## What

A `Placement` step is `Step::Rigid` — rotate about an axis through the
ORIGIN, then translate — or `Step::Literal(Frame)`
(`crates/editor-core/src/placement.rs`, `pub enum Step`). So "turn this
gauge by `swing` about the vertical through the bench's centre" has no
one-step spelling. The tour writes it as a three-step chain,
`[Literal(to the pivot), Rigid(angle = swing), Literal(from the
pivot)]` (`demos/tour/src/assembly.rs`, `turntable`). A single `Rigid`
step cannot do it either: its translation would have to be
`c − R(swing)·c`, which is trigonometry over the parameter.

Two related gaps sit at the same step:

- No step reads a `Datum::Axis`, though `PatternKind::circular` places
  about one and the evaluator already builds that motion
  (`Affine3::rotation_about_axis`, `eval/wire.rs`). A gauge can't turn
  about the hinge axis the document already names.
- There is no rotation-only constructor. A turn spells three zero
  translations and an axis of three scalar expressions
  (`turn_about_z` in the tour, a scene-local helper).

`Node::Transform` holds the same `Placement`, so it shares all three.

## Shape of a fix (the owner's call)

A rigid step with an axis ORIGIN (or a step reading a `Datum::Axis`
node), plus a rotation-only constructor. Keep the bits of today's
one-step `Rigid` unchanged (P1's exactness rule).

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. `placement::Step` is unchanged, but stage 3 retires `Step::Literal`, `Node::Transform` and `Node::Datum`, and poses move by lengths and angles through mates; re-check the turntable after D. Parked on `transform-retires-into-a-placement`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Changed. The axis half is answered; the pivot half stands.**
- `Step` is gone (D). `PoseDef::Spin { about: Axis read }` (`pose.rs:276-284`) spins any pose about any axis.
- The tour's turntable still spells `p + R(swing)·(spot − p)` as trig formulas (`demos/tour/src/assembly.rs:731-756`), because instance values chart on the target's axes (`Coord::Spin(TargetAxis::Z)`, `:765`). The tour header lists this as a finding (`:70-74`).

What is left: an instance's values cannot turn a copy about a pivot or an axis pose. That is closer to a design fork than a fix: designer pair first.
