---
id: partframes
kind: program
title: PARTFRAMES — stage 2 of fork 288: a body records its parts in their frames, and a Boolean reads each operand in its own
status: ready
opened: 2026-11-21
group: kernel/boolean/one-home
prefix: partframes/
tag: (PARTFRAMES orchestrator)
paths: [crates/topo/src/finish.rs, crates/topo/src/boolean/*, crates/editor-core/src/eval/framed.rs, crates/editor-core/src/eval/wire.rs, crates/geom-brep/src/certify.rs]
keep_out: [split off from ONEHOME 2026-11-21 when Ev chose to run fork 288 stage 2 as its own program (PR 7282) - every row moved by git mv with its id and body unchanged and a dated note on the row - shares crates/topo/src/boolean/* with ONEHOME CLEAVE and the HONE family by design, so run scripts/work.py territory on your branch and announce the seam where changes couple - stage 1 is done (PRs 7088 7099 7206) and its numbers are in a-body-records-its-parts-in-their-frames - the design is DESIGN.md placement.frame as ratified by fork 288 (PR 7079) and fork 274 (PR 7006), not to be re-litigated - plan stage 2's staging with designers before a lane builds it]
budget: 70
---

**Stage 2 of fork 288.** Fork 288 (PR 7079, approved 2026-11-18)
ratified `[placement.frame]`: a body records each of its parts in the
frame it was built in, and that record is the part's data. Relative
motion is composed through the nearest shared frame, and every reading
between two entities goes through one aligning door. Stage 1 (PRs 7088,
7099 and 7206) made every kernel door over two bodies take placed
operands. Those doors still move the operand up front, and the
whole-Boolean numbers after stage 1 (7 of 60 build at boxed motions,
the first refusals in data that move wrote) are why stage 2 is needed.

Stage 2 gives the body a frame table and moves its geometry reads
behind the aligning door, so a Boolean's result keeps each operand's
survivors bit for bit under its own frame.

The budget is 70, not 30, because the program's core is one `X` row,
`a-body-records-its-parts-in-their-frames` (48 points). That row is a
single rebuild of how bodies hold geometry, and cutting it across two
tracks would leave neither able to land the door. Ev chose to run it
as one program (PR 7282).

Slate and posture: `work/partframes/plan.md`; narrative in
`work/partframes/log.md`.
