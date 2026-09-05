---
id: MSOLVE-1
kind: unit
title: The mate reads at its operand: the transform-aware solve
status: spec
opened: 2026-09-05
branch: msolve/1-mate-operand
refs: [mate-solve-is-transform-blind]
---


## Spec

`docs/MSOLVE-1-SPEC.md`. Answers `mate-solve-is-transform-blind`
(parked on this unit; closes at merge) and lands the
pattern-of-transform half of the PR 1731 ruling. Deletes
`crates/editor-core/tests/fix_xblind_probe.rs`, as its header asks.

## Ruling of record

Ev, in chat, 2026-09-05: the mate reference carries the node it is
read at (the shape `MeasureRef` already has); that operand is an A12
reading edge; N1 is untouched; the solve composes every pose-bearing
node's map from the operand to the minting instance. The three shapes
weighed and the reasons are in `work/msolve/log.md`'s entry of that
date.
