---
id: contain-refusals-on-a-sound-face-reach-the-boolean-as-a-classification-invariant
kind: issue
title: The boolean answers ContainError's reachable refusals as ClassificationInvariant
status: open
priority: P2
cost: E
refs: [torn-body-refusal-families-beyond-the-six-doors]
opened: 2026-10-05
---

## What

`contfp` and the curved containment doors now answer their reachable
refusals typed under their own names (`ContainError::EmptyLoop`,
`LoopUnreadable`, and `Curved`, which carries the solid door's
`PointInSolidError` whole — `PartialConeFace`, `PartialTorusFace` and
`WallOutlineUnsupported` among them, each reachable on a sound face).
The boolean's two consumers still fold all three into
`BooleanError::ClassificationInvariant`, which claims a kernel bug:

- `boolean/reduce.rs` `esc` (the containment reads of the sweep and the
  curved placement, `curved_face_placement` through `esc`);
- `boolean/ops.rs`, the sphere extent scan's `contfp` (one arm for all
  three, "extent scan: contfp could not read the face's boundary").

That is the shape the old `ContainError::Corrupt` arm had, kept as it
was so the battery's line set did not move in the PR that split the
variant.

P2: `ClassificationInvariant` is a kernel-bug claim, and these are
states a sound operand reaches (a partial cone or torus face, a wall
outline the door has no arm for, a whole-turn scaffold circle).

## Direction

Route `Curved(e)` to the boolean's typed home for the solid door's
refusals (`BooleanError::Containment(e)`, whose `Display` opens "the
solids do not cross", so it needs a reading that fits a reduction-time
containment read first), and `EmptyLoop` / `LoopUnreadable` to an
honest typed refusal naming the face. Re-run the boolean batteries
(`rc_wide_battery` and the `join_*` ignored batteries); lines that
move are the point of the change and are listed in the PR.

