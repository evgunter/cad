---
id: a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge
kind: issue
title: A curved section germ tangent to a straight edge reads OnEdge(that edge) and a flank-wall face, so the join cannot pair a fillet arc lying inside a partner face
status: closed
opened: 2026-10-02
priority: P1
cost: H
closed: 2026-10-03
pr: 3880
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

## Built (JOIN-2, PR 3880)

`boolean/sectors.rs` `germ_loci` (vertex-pair sites) and `germ_locus`
(vertex-on-face sites) name `OnEdge(E)` for a bound read On only where
the reduction's records say the segment runs along E. The sweep splits
an edge at every crossing of the partner, so an edge lying on the
partner ends at a recorded touch: an edge whose far end has no `vv` or
`vf` contact leaves the partner, and the germ is only tangent to it.
Where both sectors hold such a bound, the two edges are one segment
when their far ends are a recorded vertex pair. Otherwise they part,
and the segment runs along the edge whose far end lies deeper in the
partner (`Touch`: apart < boundary < face). The other germ takes the
face holding both ends of that edge (`tangent_face`). `insert.rs` reads
a germ along an edge of either solid in that edge's bound direction,
since its two record faces may be tangent.

On the plate (convex fillets), the straight edge past the tangent point
ends at a corner the reduction never touched, so the arc carries the
segment. On the L's concave fillet, the straight edge ends at the
corner recorded on the rounded L's top face (`vf`), and the arc ends on
the sharp L's boundary (`vv`), so the line carries it. The join now
pairs every germ of
`reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`.
Its surgery refuses the germ's tangent record faces typed, and the
declared-REST zip builds each union on the join's segments (pinned
through `test_support::boolean_join_refusal`).

Residue: two edges that part with their far ends recorded alike stay as
they read
(`two-tangent-edges-parting-with-like-far-end-touches-stay-on-edge-both`).
