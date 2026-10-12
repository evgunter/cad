---
id: SYM-17
kind: unit
title: "a widened rotation angle refuses on the plain interval lane: read the column norm, then carry the rotation or say it is the tier's"
status: review
opened: 2026-10-06
priority: P1
cost: H
branch: sym/17-rotation-readout
refs: [a-widened-rotation-angle-refuses-on-the-plain-interval-lane, SYM-14]
---

## What

`a-widened-rotation-angle-refuses-on-the-plain-interval-lane`: on the
chain Ev asked for, the plain `Interval` lane refuses at the first
transform (`transform_rigid_col0_unit`), while the symbolic tier
certifies. The item's reading (`cos` and `sin` as independent brackets,
so `c·c` encloses an interval around 1) is a hypothesis nobody has read.

Phase 1 reads the column norm's enclosure at the refusal and prices
three answers: a rotation that keeps the identity by construction, a
tolerance the honest enclosure fits, or the statement that a widened
rotation angle is a symbolic-tier construction. Stop rules send a
change to another program's checks, or a scope statement for the plain
lane, to the owner or Ev.

Phase 1 stopped on both stop rules. Ev answered the `[ev]` question
on 2026-10-12, so Phase 2 builds the rigid-motion witness:
`RigidMotion<T>`, merged with `OrthoFrame` and closed under the rigid
group's action (the item's "Question, and Ev's answer").

Spec: `docs/SYM-17-SPEC.md`. Opus implementer.
- **Phase 1:** single FULL review (`work/sym/log.md`, 2026-10-06).
- **Phase 2:** dual review, concurrent arm, difficulty H
  (`work/sym/log.md`, 2026-10-12).
