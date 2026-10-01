---
id: require-kill-anchors-empty-arm-refuses-loop-cycle-broken-for-a-collision
kind: issue
title: require_kill_anchors' Empty arm refuses LoopCycleBroken for another loop Empty at the vertex, which is EmptyAnchorsCollide's question
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [euler-op-corruption-refusals-end-in-a-tag]
---

(TOPO fix pass on PR 3621, from its review's NOTE-3.)

## What

`Body::require_kill_anchors` (`crates/topo/src/euler.rs`) proves each
loop a kill re-anchors `Empty { vertex }` by three conjuncts: `writes`
anchors the vertex `Lone`, no half-edge stays in the loop, and
`empty_at_besides(vertex, target)` finds no other loop already `Empty`
at it. Any one failing refuses `EulerOpError::LoopCycleBroken` naming
the loop. The third is not a broken cycle: it is two empty loops
holding one lone vertex, which is the question
`EulerOpError::EmptyAnchorsCollide { vertex }` exists to answer ("the
operation would leave two empty loops holding the same lone vertex").

PR 3621 moved the sibling check in `Body::require_vertex_unnamed` (a
kept loop `Empty` at a vertex the kill removes) from `LoopCycleBroken`
to `KillLeavesDangling`, by the rule that a kept record naming a
removed one has the variant that decides that field. This arm is the
same shape of mismatch, left as it was.

## Repair shape

Split the third conjunct out and refuse `EmptyAnchorsCollide { vertex }`
for it, after the first two; update `LoopCycleBroken`'s doc list and
the `require_kill_anchors` doc ("is the only loop `Empty` at that
vertex"). A `review_d18` row on a torn body with a second `Empty` loop
at the vertex pins it. Both variants answer
`reports_tier1_corruption` `true` and end in
`geom_core::KERNEL_DEFECT_ENDING`, so no render changes class.
