---
id: a-fold-row-cites-a-minted-seam-by-its-head-whatever-later-steps-absorb
kind: issue
title: A union fold row citing a seam the fold minted names its head, so it reads the seam's surviving piece even where a later step absorbed the stretch holding the row's point
status: closed
opened: 2026-10-19
priority: P3
cost: M
refs: [a-fold-steps-coincidence-rows-are-named-without-the-end-passes]
closed: 2026-10-19
---


Filed by WIRE's `wire-foldsteps` lane (branch `wire/fold-steps-end-pass`)
while sweeping the fold-minted cells the end passes re-qualify.

## What

That branch reads a fold row's minted **vertex** cell under the name the
finished table holds for its entity. It follows the vertex's key through
every later step (`eval/wire.rs` `fold_vertex_forward`). A minted **edge**
cell (`Seam`, `Merged`) is still spelled by `names::fold_cell` as its head
with the pieces' qualifiers dropped. It is checked only by name against the
published table (`union_member_orders_decide_alike::cells_are_held`).

A probe over the editor-core tests matching
`union|seam|merged|junction|fold|coincid|pinch|intersect` compared each
minted edge cell's ends at the step that decided its row with the ends of
the published edge of that name. 34 `Merged` and 14 `Seam` cells had the
same extent. 4 `Seam` cells had a different one. All 4 are in
`union_pinch_member_order::crossing_one_wedge` (and the guard row that
reuses it), in the orders c, a, b, plate and a, c, b, plate. The row is
`CapVertex(End)` of b `OnCarrier` `Seam { a lateral, c lateral }`, decided
when b folds in. At that step the seam runs z 0.5 to 1.81. The plate then
absorbs the part below z 1.0, and the published seam runs z 1.0 to 1.81.
b's vertex is at z 1.7, so the published entity still holds the row's
point.

## The open question

Nothing pins the case where the absorbed stretch is the one that held
the point. The cell would then name a live seam that no longer holds what
the row states. To measure it, build a union whose middle step puts a
member vertex on a fold-minted seam, and whose last member absorbs that
stretch of the seam but not all of it. Then check the published row
against the finished body's seam.

## Where the row is published (PR 5328's review)

This row is not published in every member order. PR 5328's review counted it
in 8 of the 24 orders of `crossing_one_wedge`. Whether a fold row like this is
published in only some orders is the three-member row's question, and the
WIRE orchestrator has taken it there. This row does not decide it.

## Why edges are not forwarded the way vertices are

PR 5328 carries a minted vertex's key through every later step and reads the
finished table's name for it. A vertex either stays one vertex, is
consumed, or is split per cone, and the split is refused as a tie.

An edge has no such single successor. A later step can cut it into
pieces, each named by its ends (`Fragment(Ends)`), or absorb part of it.
`names::fold_cell` spells a minted seam or `Merged` edge by its head, the
parent, so the row names the whole seam, not one piece. Forwarding an
edge would mean choosing a piece, for instance the one that holds the row's
point. No naming rule says that yet, so that choice belongs to this row.


## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.
