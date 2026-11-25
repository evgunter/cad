---
id: solid-door-reach-drops-the-faces-uncovered-edges
kind: issue
title: The solid door's reach maps the cover's FaceUncovered to CorruptFace, dropping the edges and the reason it could not cover
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the sweep of `contain-error-drops-the-loop-its-carrier-named`
(lane `inside-contain-small`; the `map_err(|_| …)` pass at the blind
spot of its `{ .. }` pass).

## The finding

`crates/topo/src/solid_contain.rs`, `partial_reach` and
`selection_reach`: each reads `crate::cover::face_reach_from` and maps
its error with `.map_err(|_| PointInSolidError::CorruptFace { face })`.
The error is `cover::FaceUncovered`, which carries the carrier's
`Uncovered` reason and a `Vec<BoundaryUnknown>` naming each boundary
edge it could not cover (`BoundaryUnknown::Edge(EdgeKey, EdgeUncovered)`)
and why. The door's refusal keeps the face and drops both: a reader
told the face is corrupt is not told which edge, or that the cause may
be a cover the reading does not reach rather than corruption.

## The repair

Carry the `FaceUncovered` (or its first edge and reason) on the arm the
two sites raise, or a variant of its own, and render it; first decide
whether an uncovered face is a `CorruptFace` at all at these sites
(its standing is an abort, `PointInSolidError::refusal`), since a cover
the reading does not reach is a limit, not a defect.
