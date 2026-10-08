---
id: no-row-asserts-the-operand-gate-and-the-sweep-agree
kind: issue
title: No row asserts that the operand gate and the sweep's curved arm agree on the pairs the narrow phase parts
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by the review of PR 4122 (`analysis/reach-review/4122`,
`review.md` S3).

## What

The operand gate (`first_unsupported_pair`,
`crates/topo/src/boolean/reduce.rs:316`) and the sweep's curved arm
(`sweep_direction`, the `apart_from_face` closure at
`crates/topo/src/boolean/reduce.rs:1153`) read the same narrow phase
(`boolean::separating::apart`) with the same pad and one shared axis
set per operand pair. The gate's doc (`reduce.rs:270`) and the curved
arm's comment carry the consistency argument in prose: a face pair the
gate parts, the sweep parts for every edge of that face. No row
asserts it, so a change that gives the two readers different pads,
axes or items would pass every row that exercises only one of them.

## What would close it

A row over a few pairs the narrow phase parts and a few it does not,
at several poses: the gate's verdict on each face pair, set against the
sweep's trace (`SweepTrace::examined`, behind `sweep-testing`) for that
pair's edges. Every face pair the gate parts has no edge of it
examined against the other face.
