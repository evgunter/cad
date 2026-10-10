---
id: tilted-read-skips-edge-adjacent-pairs-that-cross-away-from-their-edge
kind: issue
title: The tilted read skips edge-adjacent pairs, which could cross away from their common edge
status: closed
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
pr: 4467
closed: 2026-10-10
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


## Closed

2026-10-10, PR 4467. The read now cuts every transversal pair,
adjacent ones included. It takes out every moved common edge and every
shared vertex, and decides what is left (`walls_cross`). Pinned at unit
level by `the_tilted_read_takes_an_adjacent_pair_less_its_common_edge`:
two walls sharing `[0, 1]` that also cross over `[2.5, 3]` refuse at
`0.5`, and clear when moved off or when they meet only along the edge.

**Reachability through `shell`, measured rather than proved.** The
lipped block (`common::shell_operands::lipped_block`: a lip standing
on the edge a block's top shares with its front, over part of its
length) is the shape in which the two adjacent walls come within the
lip's width of each other away from their edge. Over 54 lips at
`t = 0.1` (foot `0.05` to `0.5`, brim `0.01` to `0.9`, rise `0.01` to
`0.3`, leaning forward and back), every lip whose moved top would reach
the moved front refused at the offset door before the read ran. The
lip's moved section inverts, and the re-attached edge's parameter runs
backwards. Lips thick enough to keep the walls apart built, to their
closed form. Both outcomes were the same on the merge base. A crossing
away from the common edge needs a moved neighbour of the two faces to
pass between them, and on every body built that neighbour inverted
first. No proof was found that every planar body does this, so the read
now covers the case itself.

Rows: `an_adjacent_pair_crossing_away_from_its_edge_refuses_at_the_offset_door`
(`verbs_shell`) pins the inverted-edge refusal and a thick lip's
closed-form volume through `shell`.
