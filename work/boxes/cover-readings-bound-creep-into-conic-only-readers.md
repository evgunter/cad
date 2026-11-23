---
id: cover-readings-bound-creep-into-conic-only-readers
kind: issue
title: Curve3::cover_over needs T: SpanLocate for its spline arm, so conic-only readers in geom-brep's intersect.rs and spiric_arc_aabb's reference carry a bound they never use
status: open
opened: 2026-11-03
priority: P4
cost: E
refs: [cover-edge-doors]
---


Found by PR 6399's delta review (Q7, 2026-11-03).

`cover()` and `cover_over` moved to `T: SpanLocate` because the spline arm restricts through `SpanLocate::restrict_window`. Readers that only ever cover conics inherit the bound: `geom_brep::intersect`'s `parallel_swing`, `axes_feet`, `parallel_axes_at` and `pair_bound`. And `Bounds` does not imply `SpanLocate`, which is why `spiric_arc_aabb` keeps its own reference construction.

Options:
- split the reading's bound per arm, e.g. a conic-and-carrier reading at `T: Real` with the spline arm behind `SpanLocate`;
- accept the bound as the reading's.

Weigh this when `cover-topo-faces` settles the reading's final signature. (BOXES orchestrator)
