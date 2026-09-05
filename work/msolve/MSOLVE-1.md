---
id: MSOLVE-1
kind: unit
title: The mate reads at its operand: the transform-aware solve
status: closed
opened: 2026-09-05
branch: msolve/1-mate-operand
refs: [mate-solve-is-transform-blind]
pr: 1929
closed: 2026-09-05
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

## Closed (2026-09-05, PR 1929)

Landed: `SitedRef` (a name and the node it is read at, one type for
the mate and the measure), the walk from operand to minting instance
through transforms and one pattern level, the offset folding every
pose-bearing node's map through `transform_map` (one home, shared with
`wire_transform`), `Member { instance, copy, at }` with an explicit
`Ord`, the edit door's `ReadSiteMissingNode`, the split door's
`OperandSeveredFromMate` (both directions, after `TornCluster`), the
content key feeding `at`, the viewer's tool authoring the picked node,
the Python door's `(at, name)` pairs; `fix_xblind_probe.rs` deleted;
A11 (5) and A12 restated in `ASSEMBLY.md`. Measured first: the
blindness was never class-dependent and covered rotation. Reviews: one
MAJOR on the suite's own fixture (an interpenetrating seat), fixed with
a whole-frame product oracle and a physical seat. Residue with its own
files: MSOLVE-2 (nested patterns, the walk's structure), MSOLVE-3 (the
typed cause where `DanglingHead` now names the node the walk stopped
at), `assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern`,
and in `work/issues/` the two-roots `Naming` refusal and
`Transform` over `Instances`.
