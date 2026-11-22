---
id: a-body-level-crossing-at-a-conic-corner-a-pair-shares-alone-is-unbuilt
kind: issue
title: No body pins a crossing at a conic corner a face pair shares alone: the conic vertex reading's crossings are pinned at unit level only
status: open
opened: 2026-11-18
priority: P3
cost: M
---

Filed by lane `embed/conic-vertex` (PR 7105 fix pass, 2026-11-18), at the
review's request (its item 12).

`Reading::Conic` at a vertex the pair shares alone (`corner_of`'s `alone`
corner, `graph_of` listing the vertex) is pinned against crossings only
at unit level, through `certify_pair` and `graph_of` on hand-built
boundaries:
- `the_graph_through_a_vertex_alone_parts_a_corner_off_the_disc_and_not_one_across_it`;
- `a_crossing_quadrant_tangent_to_the_conic_at_a_vertex_alone_is_not_parted`.

The PR's first body said no kernel operation builds such a crossing.
That is too strong: the suite's sweeps cross (`a_370_degree_flat_ribbon_reads_violated`,
`an_arc_spine_past_a_full_turn_reads_violated`).

**What the lane tried.** A reasoning pass over the sweeps the suite
crosses with, not a build; the lane spent well under the hour the review
allowed.
- The arc spine's walls are lofted splines, not conics.
- In a single sweep of a circle profile (a disc cap whose rim is split
  into two half-circles at the seam), each face sharing a rim vertex
  also shares a rim arc or a seam line with the cap. So the faces that
  pass through each other on an overshoot share no topology, and the
  crossing is decided by distance rather than at a vertex alone.

**A candidate not tried:** a graft of two bodies that share no topology
cannot reach it either. What could is a body whose two faces meet
topologically at a split rim's vertex alone and cross past it, for
example two sweeps unioned without the boolean's embedding check, if a
door builds such a thing. A pin there would put the reading's crossing
guard on a body row.
