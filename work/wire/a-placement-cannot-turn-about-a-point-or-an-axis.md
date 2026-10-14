---
id: a-placement-cannot-turn-about-a-point-or-an-axis
kind: issue
title: a Placement step turns only about an axis through the origin: turning about a pivot is a three-step chain, and no step reads a Datum::Axis
status: open
opened: 2026-10-02
priority: P3
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
