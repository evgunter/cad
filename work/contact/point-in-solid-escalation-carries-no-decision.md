---
id: point-in-solid-escalation-carries-no-decision
kind: issue
title: topo: PointInSolidError::Escalated carries no decision, and its refusals end in the coincidence menu with no declaration door
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, from the §5 sweep of `contact/10-contain-endings`.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`: the recourse follows from the
decision and its verdict, and the decision is a closed type at its site.

## What

- `PointInSolidError::Escalated { face, diag }`
  (`crates/topo/src/boolean/solid_contain.rs`) is raised from some thirty
  rows of the point-in-solid door: its chart trims, wall outlines, ray ×
  surface roots and the point-in-window tests. It names none of them. Two
  chains stop there:
  - `boolean::contain::solid_err` renders it as
    `ContainDecision::SolidDoor`, which ends in the lever alone
    ("move the geometry clear of the boundary");
  - `census::Undecided::of_point_in_solid` reads it as
    `WitnessTooClose(None)`, which ends in the pair's lever
    ("move the parts until their bounding boxes no longer overlap").
  The loop walk's escalation (`PointInSolidError::Loop(PointInLoopError::Escalated)`)
  does carry its `ContainDecision`, and `WitnessTooClose(Some(_))` ends in
  that decision's lever.
- `impl Display for PointInSolidError` ends `Escalated`, `RayExhausted`
  and `Loop(RayExhausted)` in `COINCIDENCE_RECOURSE` ("declare the
  coincidence, move the geometry, or lower the tolerance"). The
  point-in-solid door takes no declaration, and D4 ¶1 (i) offers no
  unvalued lowering. A literal grep for "lower the tolerance" misses it,
  because the words live in the constant.
- `Undecided::of_point_in_solid` also folds
  `Loop(PointInLoopError::CorruptLoop)` into `WitnessTooClose(None)`, so
  an unwalkable loop is reported as a corner too close to a boundary. Its
  sibling `CorruptFace` is `CorruptInstance`.

## Repair shape

Name the door's decisions as a closed type on `Escalated`, the way
`ContainDecision` names contfp's. A point's side of a face passes on
either nonzero sign, so it wants `SizedPass::NonZero`. A chart trim's
window is a question about the face, and its lever is the face's. Then
end the Display arms through the decision, and route `CorruptLoop` to
`CorruptInstance`.
