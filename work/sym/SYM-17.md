---
id: SYM-17
kind: unit
title: "a widened rotation angle refuses on the plain interval lane: read the column norm, then carry the rotation or say it is the tier's"
status: closed
opened: 2026-10-06
priority: P1
cost: H
branch: sym/17-rotation-readout
refs: [a-widened-rotation-angle-refuses-on-the-plain-interval-lane, SYM-14]
closed: 2026-10-13
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

Spec: `docs/SYM-17-SPEC.md`, deleted at close (`docs/doc-ledger/sym-17-spec.md`). Opus implementer.
- **Phase 1:** single FULL review (`work/sym/log.md`, 2026-10-06).
- **Phase 2:** dual review, concurrent arm, difficulty H
  (`work/sym/log.md`, 2026-10-12).

## Closed (2026-10-13, PR #4197)

Phase 2 merged after a concurrent dual review on `a89f7a54eb` (both
APPROVE-WITH-FIXES; R2 raised the guide doctests red as a MAJOR, which R1
raised too, so the MAJOR was bilateral). Then came a fix pass and a delta
review on `1355348da2` (MERGEABLE-WITH-NITS), whose nits a final pass
took.

`RigidMotion<T>` replaces `OrthoFrame`. Its mints are rigid by
construction, and it has one decided door. Both the mints and the door
ask finiteness through `Real::is_bounded`, which decides an enclosure's
two ends. The plain `Interval` lane now certifies the chain at one link
up to 6.49e-9 and at four up to 6.73e-10.

Successors:
- `products-of-door-admitted-frames-are-not-re-decided` (SYM, P3);
- `the-sphere-re-cut-rotation-is-decided-at-the-door-not-minted` (CLEAVE);
- the `x − x` / `0·x` finiteness siblings filed on INTENT, BOXES and
  MSOLVE.
