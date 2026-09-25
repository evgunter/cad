---
id: cite-member-edges-group-rerank-can-reverse-the-folds-rank-direction
kind: issue
title: A seam-vertex group re-ranked by cite_member_edges runs along the member edge, while an unmoved group keeps the fold's seam-line direction, so ranks can swap between member orders
status: open
opened: 2026-09-24
priority: P2
cost: D
---


## The finding

Found by the review of PR 3168 (O4b). It is unreached: over the corpus
and the r1–r4 fixtures, every group `cite_member_edges` forms has one
member, and that member moved (438 groups).

`cite_member_edges` (`crates/editor-core/src/eval/emit_union.rs`)
re-ranks a seam-vertex base group along the member edge's own
direction, and only in an order where some vertex of the group moved.
In an order where none moved, the group keeps the fold's ranks, which
run along the fold's carrier: the seam line's `n_a × n_b`, or the
A-side edge. Suppose the two directions are opposite, and two or more
vertices share one seam base on one member edge. Then `base#0 of 2`
swaps vertices between member orders, and the name silently rebinds.

The whole-group branch has no row or unit test reaching it, and
neither does either of its refusals (`CITED_GROUP_NOT_ONE_SEAM`,
`CITED_GROUP_NO_MEMBER_EDGE`).

## Fix direction

Rank every such group in one direction, whether or not a vertex moved:
the member edge's own direction in every order. Otherwise refuse when
the two directions disagree. Build the case first: two vertices
sharing a seam base on one member edge, with the fold's direction
opposite to the edge's. It should be red on main.
