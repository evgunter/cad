---
id: boxfinish
kind: program
title: BOXFINISH — the box scalars finish a sweep leaf at tier 3, so Ev's switch on PR 6371 retires
status: ready
opened: 2026-11-08
group: kernel/boolean/one-home
prefix: boxfinish/
tag: (BOXFINISH orchestrator)
paths: [crates/topo/src/validate.rs, crates/topo/src/props.rs, crates/topo/src/finish.rs, crates/topo/src/side_reader.rs, crates/editor-core/src/drive.rs]
keep_out: [opened 2026-11-08 by splitting ONEHOME per work/README.md Track size after PRs 6371 and 6380 landed - ONEHOME measured 78 budget points against 35 - every row moved by git mv with its id and body unchanged and a dated note on the row - the switch is AtRestPolicy Leaf and leaf in crates/topo/src/props.rs and its deletion row is parked here blocked on the side reader, cost and quadrature rows - Ev on PR 6371 (comment 6582793508) keep aiming for the final state, the common cases P0, NURBS P1 - the f64 hairline row is not behind the switch and is in the deletion row's refs only - territory is shared with ONEHOME, ONEROUND, ONECOPY and the HONE family by design, run scripts/work.py territory on your branch and announce the seam in the PR]
---

**Every door finishes its own result at every scalar.** PR 6371 made
the scalar's `finish` the one constructor of a finished body. Ev
ruled (PR 6371, comment 6582793508) that at the box scalars
(`Interval`, `Sym`) a sweep leaf stays tier 2 behind one switch,
`AtRestPolicy::Leaf`/`leaf`, until tier 3 can read these bodies over
a box. The common cases are P0 and NURBS is P1. This track holds the
rows that close those gaps, plus the parked row that deletes the
switch once they land.

Slate and posture: `work/boxfinish/plan.md`; narrative in
`work/boxfinish/log.md`.
