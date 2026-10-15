---
id: a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread
kind: issue
title: A vertex a split mints on an edge lying on a curved face is not read against that face
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice]
---

Raised in review of PR 5147 (m2). Unreached: no witness is known.

`reduce::sweep_direction` reads an edge lying in a face at the edge's
ends only. When a later face splits that edge, the vertex the split
mints has to be read against that face too, or the two operand slots
disagree about it. That PR's witness is a line edge in a planar face,
split by a later face, which refused `VertexReadTwice` in one slot and
`Join(UnpairedLooseEnds)` in the other.

The sweep's worklist entry (`Pending`, `reduce.rs`) carries the
**planar** faces an edge lies in: the line lane's `(Zero, Zero)` arm and
the conic lane's `ConicPlaneMeet::Parallel` arm record them, and every
split reads its minted vertex against them in `requeue`.

An edge lying on a **curved** face is not carried. `curved_face_arm`
takes such an edge with its ends only, in these arms:

- the covered `(Sign::Zero, Sign::Zero)` arm, both ends on the carrier
  under a one-sided cover;
- the undeclared `(Sign::Zero, Sign::Zero)` arm's `SpanVerdict::LiesOn`
  (a conic arc on the carrier) and `SpanVerdict::Constant` (a ruling on
  a wall).

A later face that splits such an edge mints a vertex that is never read
against the curved face. Reading it there means asking
`vertex_on_curved_face` and combining its `Placement` under that arm's
rule (`Placement::declared` or `Placement::undeclared_no_interior`), so
the read is a placement question with a frontier, not the planar
side-then-`vertex_on_face` read `requeue` does.

Ev tried to reach it and could not. In a sweep probe, a wall block was
joined to a lens prism (two arc faces) whose corner rests on the wall's
face, against tools crossing the wall's face along the contact line.
Every op refused `VertexReadTwice` identically in both slots and both
groupings, and the rest built sound.

## Done when

A pose with an edge lying on a curved face, split by a later face of
the same operand, is measured in both slots. Either the minted vertex
is read against the curved face, or the pose is shown to be
unreachable and `Pending`'s doc says why.

