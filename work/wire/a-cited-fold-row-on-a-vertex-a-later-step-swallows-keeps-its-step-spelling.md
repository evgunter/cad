---
id: a-cited-fold-row-on-a-vertex-a-later-step-swallows-keeps-its-step-spelling
kind: issue
title: A union fold row a final record cites, on a vertex the fold minted that a later member swallows, keeps the spelling of the step that minted it
status: closed
opened: 2026-10-19
priority: P2
cost: M
refs: [a-fold-steps-coincidence-rows-are-named-without-the-end-passes, a-fold-row-cites-a-minted-seam-by-its-head-whatever-later-steps-absorb]
closed: 2026-10-19
---


Filed by WIRE's `wire-foldsteps` lane on PR 5328's fix pass (branch
`wire/fold-steps-end-pass`).

## What

PR 5328 reads a fold row's minted vertex cell under the name the finished
table gives that vertex. It follows the key with `eval/wire.rs`
`fold_vertex_forward`. If the finished body no longer holds the vertex
(`Forwarded::Consumed`), the cell keeps the name the step that minted it gave
it, which is what the base already did. No table holds that name.

Measured with `union_pinch_member_order::a_block_over_the_wedges_junction_builds_in_every_member_order`,
which adds a block covering the junction at (1.5, 1, 1) to
`crossing_one_wedge`. In orders `[1,3,0,2,4]` and `[3,1,0,2,4]` the row
`Crossing { Seam{a lat, c lat}, plate Cap(End), Enters }` `OnCarrier`
b's `LateralEdge(Leg)` is decided when b folds in. Then the block's step
swallows the vertex: `settle_cones` returns `Dead`, at a key no fusion or row
moved. The row is still cited after that. The union's final records include
`Contact { Vertex(14v9), Edge(4v5) }`, which cites `Decided(0)` alone, and
`Decided(0)` is that row. So the row cannot be dropped. With nothing left to
cite, the record would refuse `UNION_CARRY_UNCITED`.

## What it needs

There are two questions.
- Should a contact record carried onto the finished body keep citing a
  decision about a vertex that body does not hold? That is a records
  question, about the carry.
- If it should, what does the row name? In member space the vertex lies in
  three member faces and in b's edge, which is no one cell (DM4). Read
  together with the three-member row's presence question, which the WIRE
  orchestrator is taking.

The guard `union_member_orders_decide_alike::published_row_cells_denote_entities_of_their_reads`
would red on these orders. That is why the cover fixture is pinned only as a
"builds" test.


## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.
