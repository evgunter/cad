---
id: segment-ends-are-not-remapped-across-the-rest-lanes-strut-undo
kind: issue
title: The REST lane's segment ends are not remapped to the surviving vertex across undo_struts
status: dispatched
opened: 2026-10-05
priority: P3
cost: M
refs: [torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate]
---

## What

`boolean/rest.rs` `read_segments` reads each segment's end vertices
from the null-edge records (`BoolNullEdgeRecord::at_vertex`) and then
calls `undo_struts`, which kills each strut's copy (`kev_describing`,
reverse mint order). Nothing maps a segment end to the vertex that
survives the undo.

A nested strut is minted at its holder's tip (`insert.rs`, `let at =
site.map_or(vertex, ..)`; `BoolNullEdgeRecord::at_vertex` in
`boolean/mod.rs` says so), so the inner record's `at_vertex` is the
outer strut's copy. The undo kills the inner copy first, then the
outer's, so a segment end read from a nested pair no longer resolves
when `realize_seam` runs.

`realize_seam` reads every open segment's ends before each take
(PR 4056), so such a segment refuses
`JoinDesync("REST lane: a segment's end no longer resolves")` before any
segment is taken. Before PR 4056 the stale end read as unjoined, and the
answer depended on take order: a desync where the segment was taken, or
the `Ok(None)` fallback to the join's own refusal (or
`SegmentsBetweenIsolatedPierces`) where it was not. So PR 4056 turns a
former `Ok(None)` fallback into `JoinDesync` in that case; no sound case
regresses, since a taken stale end already desynced. The `join*`
batteries do not reach a nested pair on this lane (base and head
outputs are line-identical).

`rest::tests::a_stale_end_refuses_before_the_first_segment_is_taken`
pins the eager reading.

## Direction

Remap each segment end across `undo_struts` to the vertex its kill
fuses it into (the record's `at_vertex` chain down to the classified
vertex), so a segment end is live by `realize_seam`; then a stale end
there is a kernel bug, and the desync can become a panic on a proven
key. Build a nested-strut REST union that reaches it first, to show the
remap is load-bearing.
