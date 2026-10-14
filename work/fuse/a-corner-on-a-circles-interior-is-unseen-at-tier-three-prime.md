---
id: a-corner-on-a-circles-interior-is-unseen-at-tier-three-prime
kind: issue
title: Tier 3′ does not see a vertex resting on a curved edge's interior: unrecorded, a corner on a circle off its vertex reads clean
status: closed
branch: fuse/census-tier3-corners
pr: 5051
closed: 2026-10-14
opened: 2026-10-07
priority: P3
cost: M
---

## The finding

The census's vertex-granular sweeps read straight edges only
(`crates/topo/src/census.rs:1173`, `edge_is_line`): a vertex resting
on a curved edge's interior is confirmed when an op records it (the
curved confirm lane, `census/curved.rs`), but never found when no op
did. The corner witness pins the gap as expected
(`crates/sweep/tests/snowman.rs:1568`): a cube's corner on a cap
circle 40° from its conventional vertex, with its `(u, E)` record
stripped, reads clean at tier 3′, where the same corner at the
vertex reads `UndeclaredContact`. An op that lost such a record would
ship a body whose touch no tier sees.

## What it needs

A vertex-on-curved-edge sweep: each vertex against the curved edges
whose boxes it meets, read through `census::curved::on_curved_interior`
(which already decides on-carrier and in-span), finding an unrecorded
touch as `UndeclaredContact`. The snowman row's 40° arm then flips to
expect the finding.

## 2026-10-14, closed: PR 5051

Tier 3′ gains **pass 2′**, `census::sweep_vertex_curved_edge`. Each
vertex is read against each curved edge, and an unrecorded vertex on the
edge's interior is an `UnrecordedContact` `VertexOnEdge`. It is backed
as pass 2's is: by the `(vertex, edge)` record, the face rung, or a
conventional vertex's edge-edge record.

- **Skipped before any reading:**
  - Candidates are the pairs whose padded boxes meet: a new class of
    curved-edge boxes (`boolean::boxes::edge_box`, a spline's control
    hull) in the census's
    `Candidates`, beside the line edges'.
  - A vertex at one of the edge's ends is pass 1's.
- **Only reaching vertices are read.** A vertex that every piece of the
  span clears (`census::curved::may_rest_on`) pushes nothing, so an edge
  the reading would refuse is refused only for a vertex that reaches it.
- **Reproduced and pinned through the public API.** The snowman row's
  40° arm (`a_corner_at_a_caps_conventional_vertex_reads_the_circles_interior`)
  now expects the finding, and goes red with the pass removed.

The sibling, an unrecorded curved edge-edge meeting at pass 5, is
`work/fuse/an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime.md`.
It waits on `[ev]` PR 5027.
