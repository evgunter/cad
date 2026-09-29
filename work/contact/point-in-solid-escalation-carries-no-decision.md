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
  surface roots and the point-in-window tests. It names none of them, and
  it does not say how its reading stands (an in-band margin, a straddle
  of two bounds, or a row decided and still refused). Two chains stop
  there, and both spell "no decision named" the same way:
  - `boolean::contain::solid_err` renders it as
    `ContainError::Escalated { decision: None, .. }`;
  - `census::Undecided::of_point_in_solid` reads it (and the door's own
    `RayExhausted`) as `WitnessTooClose(None)`.
  Both end in the one unnamed lever, `boolean::placement_lever(None)`
  ("move the geometry clear of the boundary"). Its `Invalid` margins take
  the unreadable-margin note, because the door cannot say which of them
  are straddles. The loop walk's escalation
  (`PointInSolidError::Loop(PointInLoopError::Escalated)`) does carry its
  `LoopDecision`, and `WitnessTooClose(Some(_))` ends in that decision's
  lever.
- `impl Display for PointInSolidError` ends `Escalated`, `RayExhausted`
  and `Loop(RayExhausted)` in `COINCIDENCE_RECOURSE` ("declare the
  coincidence, move the geometry, or lower the tolerance"). The
  point-in-solid door takes no declaration, and D4 ¶1 (i) offers no
  unvalued lowering. A literal grep for "lower the tolerance" misses it,
  because the words live in the constant.

## Repair shape

Name the door's decisions as a closed type on `Escalated`, the way
`LoopDecision` and `ContainDecision` name the loop walk's and contfp's.
Tag each site's reading with `splitting::Escalation` (margin, straddle
or decided). A point's side of a face answers on every definite reading
(on it, or clearly to either side), so it wants `SizedPass::Definite`. A
chart trim's window is a question about the face, and its lever is the
face's. Then end the Display arms through the decision.
