---
id: a-two-member-unions-row-margin-follows-its-list-order
kind: issue
title: A two-member union's row margin follows its list order, so the two orders publish one row with two margins
status: closed
opened: 2026-10-18
priority: P3
cost: M
refs: [a-fold-row-names-a-vertex-the-fold-minted-by-the-step-that-minted-it]
closed: 2026-10-19
---


Filed by WIRE's `wire-foldmint` lane (branch `wire/fold-minted-cells`).

## What

Past two members, `eval::wire`'s `judge_pairwise_contact` now runs each
pair with the earlier-minted read as operand A, so one pair publishes
one margin in every member order. A union of TWO members runs no
separate judgement: its one fold step is the pair's judgement
(`wire_combine`, `pairwise` false), run in list order because the glue
keeps the listed-first member's description (D10). So its rows' margins
follow the list.

Measured on `union_pinch_member_order::pinch`'s two blocks alone:
`Union([p1, p2])` and `Union([p2, p1])` publish one row
(`CapVertex(Start)` of `p1` `OnCarrier` `LateralEdge(Leg)` of `p2`,
`VertexFusion`) with margin `Value(2.22e-16)` in one order and
`Value(0.0)` in the other. Every other row, name, count and the volume
agree (`union_member_orders_decide_alike`'s `compare`, after mapping
that one margin, is `Ok`). The margin decides Zero either way.

## What it needs

One value for the pair whichever member is listed first: read the
step's margins as the canonical-order run would have (a second pair
run per two-member union, which doubles its cost), make the kernel's
vertex-on-carrier margin symmetric in its operands, or have the row
comparison read a margin as its decision. Then add the two blocks to
`union_member_orders_decide_alike` as a two-member fixture.


## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.
