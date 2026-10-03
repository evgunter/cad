---
id: two-pinches-crossing-on-one-line-refuse-their-union
kind: issue
title: Two pinches crossing on one line refuse their union: two crossing pairs share both their vertices
status: dispatched
opened: 2026-10-03
priority: P0
cost: H
branch: fuse/two-pinches-one-line
---


## What

Two pinches on one axis in complementary quadrants, overlapping in z:
`pinch_a = brick((0,1),(0,1),(0.5,1.5)) ∪ brick((-1,0),(-1,0),(0.5,1.5))`
and `pinch_b = brick((-1,0),(0,1),(1,2)) ∪ brick((0,1),(-1,0),(1,2))`,
each a Boolean output with its own records carried. Their union fills
all four quadrants over z ∈ (1, 1.5). At each end of the overlap each
pinch holds two vertices, and all four vertex-vertex pairs between them
cross, so two crossing pairs share both their vertices. The union
refuses `SharedVertexCrossings` in both operand orders
(`insert::reconcile_shared`, the guard on crossing pairs that share
both vertices, `crates/topo/src/boolean/insert.rs`). The subtract
builds. Pinned by `two_pinches_crossing_on_one_line_refuse_their_union_typed`
in `crates/topo/tests/union_flush_onto_edge_contact.rs`, the second
witness of `a-vertex-crossing-both-sides-of-a-pinch-refuses-shared-vertex-crossings`,
which `fuse/shared-vertex-crossings` built for the one-sided case only.

**P0** on that row's ruling (FUSE orchestrator, 2026-10-02): both
operands are Boolean outputs, and the next Boolean refuses them.

## What was measured

With the guard off, the null edges insert and the join completes,
but the finish cannot place the result. At (0,0,1) the union holds one
vertex whose link is a single cycle passing the −z direction twice:
the two pinch edges below it end there, coincident. The seam
correspondence then gives one A vertex both B partners. Before
`fuse/shared-vertex-crossings` admitted that, it refused `JoinDesync`
("conflicting seam vertex correspondence"). Admitted, the zip fuses a
vertex onto an edge's other end and refuses `Euler(SelfLoopEdge)`.

## Owed

Decide what the result is at rest: one vertex with two coincident
edges ending at it, or two vertices there with a v-v record between
them. Then build the zip arm that produces it, and flip the pinned
test to a build at volume 4 that passes 3′ with both records carried.
