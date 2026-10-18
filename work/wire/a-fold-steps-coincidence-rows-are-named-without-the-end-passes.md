---
id: a-fold-steps-coincidence-rows-are-named-without-the-end-passes
kind: issue
title: A union fold step's coincidence rows are named against collapse_table(acc_table) and published without the end passes, so a row citing an intermediate entity can keep fold qualifiers
status: closed
opened: 2026-10-17
priority: P2
cost: M
refs: [a-union-fold-publishes-no-pinch-row]
closed: 2026-10-18
---


## Finding

Found by WIRE's refusal-endings fix pass (PR 5262, 2026-10-17) while
checking the invariant `names/emit_union.rs`'s module doc used to record.
The old guard (`is_fold_qualified_member_edge`) and the refusals it
protected were removed in `c6e9e3d95b`; a mid-fold refusal now carries no
fold-table name at all (`UnionFoldStep` wraps the kernel's refusal). But
the coincidence rows a fold step decides are named against
`collapse_table(acc_table)` in `eval/wire.rs` `wire_combine` and published
(the #4323 fold) without the end passes the finished body's names get. A
row citing an intermediate entity can therefore keep the fold's
qualifiers, so its name is not one the published table holds, and nothing
guards that. Unmeasured: build a three-member union whose middle step
decides a coincidence on an entity the last step re-qualifies, and compare
the row's cited name against the published table.

Read together with `a-union-fold-publishes-no-pinch-row` (same function,
same fold).

Signed (WIRE orchestrator).

## Measured on the member-space spelling (2026-10-18, `wire-foldrows`)

With fold rows spelled by member cells (`names::fold_cell`), a cell
whose collapsed name has a member `From` head no longer carries fold
qualifiers: in a probe run of the editor tests matching `union`, every
fold-row cell was `From` (vertex or edge), `From` + `Fragment` (edge),
`Crossing` (vertex) or `Merged` (edge). No vertex cell had a `Fragment`
tail. The `Crossing` cells the fold minted are still read off
`collapse_table(acc_table)`, and 236 of 354 published ones are not
names the node's table holds:
`a-fold-row-names-a-vertex-the-fold-minted-by-the-step-that-minted-it`.

## Resolution (2026-10-18)

The guard is now in place. `union_member_orders_decide_alike`'s `decide`
checks every published row of every order that builds
(`cells_are_held`). Each `Entity` cell must be a name held by the table
of the read it names: a member cell's member table, or the union's own
published table for a cell the fold minted. Every fixture that `decide`
runs is covered: the corpus, the emit corpus and the vertex-row fixtures,
including `union_pinch_member_order::pinch`. The mutant that publishes a
member cell under its collapsed fold-table name in `in_member_space`
turns `a_vertex_row_is_the_unions_once_in_every_member_order` red at this
check.
