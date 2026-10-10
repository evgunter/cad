---
id: tilted-read-skips-edge-adjacent-pairs-that-cross-away-from-their-edge
kind: issue
title: The tilted read skips edge-adjacent pairs, which could cross away from their common edge
status: dispatched
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
branch: shell/tilted-read-gaps
---

Raised in the review of PR 4311, which added the tilted read
(`moved_walls_cross`, `crates/topo/src/shell.rs`).

The read skips every edge-adjacent pair of planar cavity faces
(`face_neighbours` in `moved_walls_cross`). The reason is that the
pair's moved planes share the line of its moved common edge, so the two
cuts of that line always overlap along the edge and the overlap decide
cannot tell a crossing from the edge itself. A crossing that inverts
the common edge refuses at the offset door's interval-forward check on
the re-attached edge (module docs, "Where the loud cases actually
refuse").

Not ruled out: an adjacent pair whose moved faces also cover a second,
disjoint stretch of that line, and overlap there, while the common edge
stays forward. That needs a non-convex face with a second stretch of
boundary reaching the common edge's line. Every extruded row in the
tree has its adjacent pairs meet only along the edge, and the review
could not rule out a non-extruded body that does this.

Done when: the read cuts an adjacent pair's line, removes the interval
that is the moved common edge, and decides the overlap of what is
left, with a row that either crosses away from its edge or shows by
construction that no planar body can.

