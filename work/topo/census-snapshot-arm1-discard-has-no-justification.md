---
id: census-snapshot-arm1-discard-has-no-justification
kind: issue
title: census.rs snapshot's arm-1 LoopBoundary discard is the one of three with no written justification
status: open
opened: 2026-09-06
refs: [S49]
---


## Finding

Filed by the GATES orchestrator from the `gates/loop-boundary-discards`
lane (PR 2044), which registered every `LoopBoundary` discard under
`crates/*/src` (80 sites; 2 audited). `crates/topo/src/census.rs:611`
(`snapshot`) is the third arm-1 discard #737 points at and the only one
of `census.rs`'s three with no justification at all — the other two, in
`sweep_cross_solid_backstop`, name the arm that asks the same question
about the same pair and are registered `audited`. This one is registered
`unaudited`; the audit (name the deferring arm, or make the discard a
typed refusal) is TOPO's, and closing it moves the register entry's
disposition in `scripts/gates/loop-boundary-discards.sh` in the same
PR. `census.rs` is CURVED's by glob and TOPO's by the #737 lineage;
whichever takes it announces the other.
