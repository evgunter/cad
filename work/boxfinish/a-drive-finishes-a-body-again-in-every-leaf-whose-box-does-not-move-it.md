---
id: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
kind: issue
title: A drive builds and finishes a body again in every leaf, though the leaf's box moves none of its inputs: a third of the tour's leaf time
status: open
opened: 2026-11-22
priority: P1
cost: H
design: true
---


Split from `tier-3-at-every-box-replay-leaf-multiplies-the-decisions`
(its second part) once M7 showed it is the largest cost left when the
check after a definite sign is gone. A design question; designers are
weighing it (the fork's record is on
`analysis/design-fork/a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it`).

**What is measured** (the box-scalar switch's box arm off, so every
sweep door finishes its leaf at `Sym<Interval>`; `boxfinish/probe-switch-off`,
`probe/M4-REPORT.md`, the M5 section of the cost row, `probe/M7-REPORT.md`).

- **A drive finishes one body many times.** In the tour's certified
  study, 3,069 box-lane finishes cover 255 distinct bodies (by vertex
  fingerprint): 92% of the finishes repeat a body already finished in
  the same drive.
- **The repeated body is one value.** The tour's plate is finished
  1,023 times, once per replay, with the same boxes and the same
  symbolic nodes every time: the leaf's box varies only in variables
  the plate does not read. Each hole's value is finished about eight
  times.
- **Refusals repeat too.** The holes' 2,046 finishes all refuse.
- **What it costs once the check is gone** (M7): the tour's drive is
  215 s of leaf time over 1,023 replays; box finishes are 72 s of it
  (33.6%, the same share as before the check went): the plate's 1,023
  finishes 22 s (21.6 ms each), the holes' 2,046 refusing finishes 50 s
  (24.5 ms each). The build outside the finishes is the other two
  thirds, and it too is repeated per replay.
- **Per leaf:** a plate with holes runs four finishes, one per sweep
  door; a leaf with a move runs three (the door, the moved copy's
  re-finish, the union's result). `m10_8`'s
  `the_a0_set_is_a0_alone` is 1.6 s against a 1.0 s budget on that.
- **Not measured:** how much of a replay's build (outside the finish)
  is of bodies the leaf's box does not move; whether the symbolic
  nodes of an unmoved body are the same node ids across replays or
  equal expressions rebuilt; what share of the 1,023 replays refuse at
  the same place for the same reason.

**Beside it:** ONEHOME has been cutting the number of finishes a
document's build runs by other means (`corpus_finish_count` 505 → 390:
readers move no body, an aggregate finishes once, the n-ary fold and
the at-rest intersection take placed operands; its fork 288). That is
finishes per build; this row is builds and finishes per drive.
(BOXFINISH orchestrator)
