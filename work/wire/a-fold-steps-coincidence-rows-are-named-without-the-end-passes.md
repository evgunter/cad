---
id: a-fold-steps-coincidence-rows-are-named-without-the-end-passes
kind: issue
title: A union fold step's coincidence rows are named against collapse_table(acc_table) and published without the end passes, so a row citing an intermediate entity can keep fold qualifiers
status: closed
opened: 2026-10-17
priority: P2
cost: M
refs: [a-union-fold-publishes-no-pinch-row]
closed: 2026-10-19
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

## Guarded, and measured still open (2026-10-18, `wire-small`)

The guard is `union_member_orders_decide_alike::cells_are_held`. Every
published row's `Entity` cell must be a strict row of the table of the
read it names, of the cell's kind, and its key must be a live entity of
that read's body. For a member cell that is the member's table; for a
cell the fold minted, it is the union's own published table. It runs on
every order that builds through `decide` (the corpus, the emit corpus
and the vertex-row fixtures).
`published_row_cells_denote_entities_of_their_reads` runs it over the
plate `pinch` (member cells) and a flush pair on an edge
(`flush_onto_an_edge`, fold-minted `Merged` edge cells), and requires
each fixture to reach cells of its kind. Two mutants in
`in_member_space` turn it red: one publishes a member cell under its
collapsed fold-table name, and one publishes a minted cell with a
fragment tail the published table does not carry.

**The finding is live.** A probe over the editor tests matching `union`,
`seam`, `merged` and `junction` saw 44 fold-minted published cells:
- 32 `Merged` edges and 10 `Seam` edges, all held;
- 2 `Crossing` vertices that are **not** held.

Both are in
`union_pinch_member_order::a_prism_crossing_one_wedge_about_the_line_publishes_one_table_in_every_member_order`
("with the plate"). Each is a `Crossing { edge: Seam { a: <prism a
lateral>, b: <prism c lateral> }, face: <plate Cap(End)>, sense: Enters }`
vertex: a crossing of an edge the fold minted, named by the step that
minted it. The union's table holds the plate-top crossings only under
member edges (`Crossing { edge: From { .. LateralEdge .. } }`). This is
the row's case: the cell keeps the fold's spelling, and the end passes
never re-spell it. That fixture is therefore not in the guard's test,
and adding it is this row's acceptance.

## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.
