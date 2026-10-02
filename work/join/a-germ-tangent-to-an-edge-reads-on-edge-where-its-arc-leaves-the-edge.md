---
id: a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge
kind: issue
title: A curved section germ tangent to a straight edge reads OnEdge(that edge) and a flank-wall face, so the join cannot pair a fillet arc lying inside a partner face
status: open
opened: 2026-10-02
priority: P1
cost: H
---


Found by JOIN-2 (branch `join/2-zip-reads-segments`, on JOIN-1's head),
measuring whether the declared-REST zip can read the join's segments.

## What

`crates/sweep/tests/reach_continuation.rs`,
`a_tangency_in_the_middle_of_an_edge_keeps_its_typed_refusal`, second
half: a plate with four rounded corners (A, z ∈ [0, 1]) under a sharp
6×4 plate (B, z ∈ [1, 2]), the mating plane declared `Rest`. A's top
outline is four straight sides on B's bottom edges and four quarter-arc
fillets inside B's bottom face, tangent to B's bottom edges at their
ends.

The join refuses `Join(UnpairedLooseEnds { count: 8 })`: the eight
fillet germs. At each tangent point the arc germ's direction is
exactly along B's straight bottom edge, so
`boolean/sectors.rs` `germ_locus` reads the bound On and names the
germ's B cell `OnEdge(that bottom edge piece)`. The two ends of one
fillet name two different B edges (e.g. the south and the west bottom
edges), so `join::partners` never pairs them. The record's B face is
the side-wall sector (faces 3 to 6, one per side), whose section with
A's fillet cylinder is a vertical line, not the arc: the arc lies in
B's bottom face. Both the locus and the face are first-order readings
that a second-order departure defeats.

The row builds today only because the declared-REST zip's own
`enumerate_segments` ignores loci and pairs the fillet ends by
straight-chord facing (`rest.rs`), then finds the arc between them by
vertex pair. That is the zip's second spelling of matching, which
JOIN-2 is meant to retire.

## Why it matters

It is the one zip-built row (of 10 on JOIN-1's head) where the join's
matching does not yield the segments, so it decides whether the zip
can read the join's segments without a regression.
