---
id: a-fold-row-names-a-vertex-the-fold-minted-by-the-step-that-minted-it
kind: issue
title: A union's fold row citing a vertex the fold minted spells it by the step that minted it, so the plate pinch's rows differ by member order
status: closed
opened: 2026-10-18
priority: P2
cost: M
refs: [a-fold-steps-coincidence-rows-are-named-without-the-end-passes, a-union-fold-publishes-no-pinch-row]
closed: 2026-10-18
---


Filed by WIRE's `wire-foldrows` lane (branch
`wire/fold-rows-in-member-space`), which spells each fold row by member
cells (`names::fold_cell`, FORK-FOLDPINCH) and keeps a cell the fold
minted in the node's space under its head.

## What

`crates/editor-core/tests/union_pinch_member_order.rs`'s `pinch` (the
plate and the two blocks), run through
`union_member_orders_decide_alike`'s `decide` in all six orders,
publishes rows that are not one multiset. Two causes, both measured:

- **A fold-minted vertex in a three-member row.** In the four orders
  that fold the plate before one block, the node publishes one more row
  than the two blocks-first orders (5 against 4): the second block's
  `LateralEdge(Leg)` `OnCarrier` `VertexFusion` against
  `Crossing { edge: <first block's leg>, face: <plate Cap(End)>, sense: Leaves }`,
  the vertex the first block's leg made on the plate's top. Its spelling
  names whichever block reached the plate first, while the published
  table names that vertex one way in every order
  (`union_pinch_member_order::a_pinch_union_publishes_one_table_in_every_member_order`).
  In one probe run of the editor tests whose names match `union`, 236
  published cells were a fold-minted `Crossing` vertex under a name the
  node's published table does not hold, 118 one it holds, and every
  fold-minted `Merged` edge cell (32) was held.
- **The judged pair's margin follows operand order.** The pair of
  blocks judged with `p1` as A and with `p2` as A publish one row
  (`CapVertex(Start)` of `p1` `OnCarrier` `LateralEdge(Leg)` of `p2`,
  `VertexFusion`), its margin `Value(0.0)` in one order and
  `Value(2.22e-16)` in the other. This row was not published before the
  judgement published its `VertexFusion` rows. Read by PR 5276's
  review: the margin is reproducible and decides Zero either way; the
  difference is the diagnostic's rounding under the operand assignment.
  That is not a determinism defect, but the published value breaks
  DM4's "one row in every order".

## What it needs

DM4 says a three-member row is "spelled by its member cells, so it is one
row in every order", and that no row names a cell the fold minted. So a
cell the fold minted is spelled by the member cells that make it (the
edge and the face of a `Crossing`), or a row already in the judgement
cites it. The margin needs one value for one pair whichever member is A,
or a row comparison that reads a margin as the pair decided it. Once
both hold, add `pinch` to
`union_member_orders_decide_alike::a_pinchs_row_is_the_unions_in_every_member_order`.

## Resolution (2026-10-18)

Closed by PR 5303:
- A fold-minted `Crossing` of a member edge is spelled as that member
  edge.
- A fold row that restates a judged overlap cites the judged rows.
- Each judged pair runs with its lesser read as A, so one pair publishes
  one margin.
- Statements carry their origin. One pair's rows never merge, two pairs'
  rows merge only when identical, and fold rows merge only when they come
  from the same minted names.

Pinned by `union_member_orders_decide_alike` with the plate `pinch` and
`union_row_tests`. The two-member margin residue is
`a-two-member-unions-row-margin-follows-its-list-order`.
