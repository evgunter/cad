---
id: readback-dangling-ref-calls-a-torn-record-a-stale-handle
kind: issue
title: readback::DanglingRef raises Entity for a record link, so a torn body is told its handle is stale
status: open
opened: 2026-10-03
---


(TOPO orchestrator, from the stale-key design fork, PR 4006; found by all
three designers, by reading, without a probe.)

## What

`readback::DanglingRef` (`crates/topo/src/readback.rs`) splits on
**entity vs geometry** and its docs read that as "the caller's stale
handle vs a corrupt body". The split holds only for a one-hop read. Where
a walk follows records, a record that names nothing comes out as
`DanglingRef::Entity`, and `ReadbackError` renders it "the handle is
stale, or it belongs to another body's lineage". That is false for a torn
body. Sites:

- `merge_faces.rs`: a face's ring keys, a loop's half-edge in
  `boundary_points`, a half-edge's edge;
- the readback walk `edge_sides` → `side_of` (edge → half-edge → loop →
  face).

The euler `From<DanglingRef>` impl is where this axis crosses into
`EulerOpError`.

## Direction

Under PR 4006's final state the axis is argument vs record (`KeyFrom`),
and a record miss is `Torn(Dangling { from, link, to })`. `DanglingRef`
wants the same axis. Land it with or after the stale-key code.
