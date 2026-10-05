---
id: tier-three-renders-every-carried-curved-containment-refusal-as-not-yet
kind: issue
title: Tier 3's classify_contain renders every carried Curved refusal as NOT_YET, a CorruptFace included
status: dispatched
priority: P3
cost: E
refs: [torn-body-refusal-families-beyond-the-six-doors, contain-refusals-on-a-sound-face-reach-the-boolean-as-a-classification-invariant]
opened: 2026-10-05
---

## What

`ContainError::Curved` carries the solid door's `PointInSolidError`
whole, so each arm keeps its own name. `validate.rs` `classify_contain`
folds it back at the renderer: one arm, `ContainError::Curved(_)`,
renders "a curved face's trim is one the check cannot yet read" with
the `NOT_YET` recourse for every carried arm. That is honest for
`PartialConeFace`, `PartialTorusFace` and `WallOutlineUnsupported`, but
`CorruptFace` is a body defect, and `Escalated` is a point too close
to a boundary (`OFF_BOUNDARY`, as the top-level `Escalated` arm renders
it).

## Direction

Match the carried arms by name in `classify_contain`, rendering each
with the cause and recourse its own kind has elsewhere in tier 3 (no
wildcard, so a new `PointInSolidError` arm has to be placed), and add a
`refusal_concision_at_rest` sample per arm that renders differently.
