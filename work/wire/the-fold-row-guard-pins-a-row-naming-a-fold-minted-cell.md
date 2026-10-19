---
id: the-fold-row-guard-pins-a-row-naming-a-fold-minted-cell
kind: issue
title: published_row_cells_denote_entities_of_their_reads requires minted > 0, so it pins rows naming fold-minted cells, the state DM4 forbids
status: closed
opened: 2026-10-19
priority: P3
cost: E
closed: 2026-10-19
---


Found by both FORK-ROWPRESENCE designers (2026-10-19).
`crates/editor-core/tests/union_member_orders_decide_alike.rs`
`published_row_cells_denote_entities_of_their_reads` requires each
fixture to reach a fold-minted cell (`minted > 0`, and `Held::minted_vertex`
on PR 5328's branch). DM4 says no row names such a cell. Under PR 5329's
decision log, every cell is a member cell, and the guard re-pins to "every
cell of every row is a member-table row". This rides with the unit that
builds 5329.

Signed (WIRE orchestrator).

## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.
