---
id: a-contradicted-extent-refuses-with-no-typed-cause
kind: issue
title: A face whose lines or nappe contradict its extent refuses as plain Uncertified departure, after splitting to the depth budget where only one face is contradicted; the payload names no cause
status: open
opened: 2026-11-17
priority: P3
cost: M
---


## Finding

From EMBED lane half-sphere's second review (PR 7086).

`face_extent` (`crates/topo/src/injective.rs`) refuses
`Unpresented::Structural` for a face whose own lines or nappe
contradict its extent. That is a fact about the face's stored rows,
not about any cell, but it surfaces with no cause of its own.

The review planted the original wrong extent on the revolved ball
(each sphere face's `u` extent replaced by its complement):
- With both faces contradicted, the ball refuses plain
  `EngineRefusal::Uncertified { arm: Departure }` at the root (`stuck`).
- Where only one face of a pair is contradicted, one side's
  `Structural` only splits, so the pair runs to the depth budget
  before it refuses.

Neither payload says that the face's extent is off its rows.

## What is needed

A typed refusal naming the face and the contradiction (a line against
the extent, or the nappe against it), raised as soon as either face of
a pair carries one. It belongs to the engine's refusal set
(`EngineRefusal`) beside `CollapsedChart`.

P3: no body reaches it now. A body that does is first a soundness
question (see `a-face-with-no-line-on-a-periodic-axis-reads-its-sheet-off-the-stored-lift-unchecked`)
and only then a diagnostics one.

## A built body that showed it, and a new source of the refusal (EMBED, lane pole, PR 7115, 2026-11-21)

On PR 7115's second head (`39f25fc2d`) `face_extent` refused
`Structural` for a cone cavity a hundred units off the origin (the
triangle `r = 0.37`, `h = 2.3`, apex up at `y = 103.6`, its axial-door
cavity): the extent ended `1.4e-14` off the apex, past the slack that
head moved it across. It surfaced as
`Uncertified { 1v1, 2v1, Edge(1v1), Departure }` at 167 / 17 / 178,
with nothing saying the extent was the cause; the review found the
cause by loosening the slack. That body holds on the PR's final head.

The final head adds one source of the same untyped refusal: a face
holding a vertex at a pole or an apex whose stored extent ends off
that point by the band (`reaching_its_singular_points`). No body
measured reaches it.
