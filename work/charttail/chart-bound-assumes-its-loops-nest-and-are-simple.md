---
id: chart-bound-assumes-its-loops-nest-and-are-simple
kind: issue
title: ChartBound's certified region statement assumes the face's loops nest and are simple, and nothing decides that where it is assembled
status: closed
opened: 2026-10-12
priority: P2
cost: M
refs: [4822, validate-tier3-curved-boundary-containment, a-cone-face-whose-rings-leave-or-overlap-its-region-measures-silently]
closed: 2026-10-24
---


## What

`topo::chart_bound` assembles a face's `ChartBound`: one closed chord
polygon per loop in the face's chart, each chord either a chart segment by
structure or carried with a certified box. Its statement is "the face lies
in the outer polygon plus its boxes, minus the rings' polygons".

That statement is only true if the loops nest (every ring inside the outer
loop), the rings are pairwise disjoint, and no loop crosses itself between
vertices. The assembly decides one premise of the same family,
`OuterSpansPeriod`, and none of these. `editor-core::clearance` consumes
the statement as certified.

Found by a designer on the FLUX fork `[ev]` PR 4822 (which layer
guarantees a face's loops bound one region). The recommendation there puts
the region statement in tier 3's check 9, off this same chart polygon.
Deciding it where `ChartBound` is assembled would give the clearance engine
the guarantee as well. Check 9 would then report, by name, what the
assembly refused.

## The question

Whether the assembly decides nesting, disjointness and simplicity itself
(and check 9 reads its refusal), or documents the premise and leaves the
clearance consumer to require an at-rest body. This waits on Ev's answer
on PR 4822.

## Closed

Closed 2026-10-24 by `clearance-cells-are-decided-in-model-space`
(CHART): `topo/src/chart_bound.rs` and `pcurves::chart_boundary` left the
tree, and clearance decides its cells off each face's edges in model
space (`topo::cell_region`). The code this row is about no longer exists.
