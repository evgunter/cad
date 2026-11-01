---
id: a-vertex-minted-inside-a-face-reads-as-that-face-alone
kind: issue
title: A boolean's row on a vertex it minted inside a face names the face and not the pierce that minted it
status: dispatched
opened: 2026-10-26
priority: P1
cost: M
branch: touchrec/in-face-mint-keeps-its-pierce
---


Filed by WIRE's `wire-meetrow` lane (branch
`wire/meet-row-keeps-the-minting-cell`), from that unit's sweep.

## What

`topo/src/boolean/mod.rs` `Mints` now keeps, beside each vertex the
reduction mints on an edge, the cell of the other operand whose crossing
minted it (`EdgeSplit::by`, `Mints::minted_by`), and
`boolean/ops.rs` `spell` widens such a vertex to the edge and that cell
(`input_read`). A vertex minted *inside a face* where a held pierce
stands (`Mints::push_in_face`, called from the ring minting in
`boolean/mod.rs` beside `vtxfac::faces_holding_pierce`) keeps only the
face: `ops.rs` `input_end` reads it as `Cell::Face(face)` and nothing
else. The pierce that minted it was another operand's cell meeting that
face, so a row on such a vertex may name the face and omit the piercing
cell, the in-face sibling of
`work/wire/a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it.md`.

The unit did not establish that a row reaches this: the ring tie's
`through` names the pierced face for the rows it decides
(`cite_rows`), and that may already cover every row such a vertex
appears in.

## What it needs

A fixture whose decision names a vertex minted in a face beside a
cell other than its ring tie's, spelled in every member order of a
union, to show whether the piercing cell is dropped; and, if it is,
`push_in_face` keeping the piercing cell the way `EdgeSplit::by` keeps
the crossing one.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TOUCHREC holds a result's records of its own touches and the glue's kept copy; id and body unchanged. (CLEAVE orchestrator)

2026-11-01: banded P1, cost M. It is the in-face sibling of WIRE's closed P1 `a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it` (one point spelled two ways across a union's member orders); the fixture that settles whether a row reaches it is the first half of the work, and the row closes on that fixture if none does. (TOUCHREC orchestrator)

## No row reaches it (`touchrec-inface`, 2026-11-01)

Every row on a vertex minted inside a face already names the pierced
face, so the piercing cell is not dropped.

- `cite_rows` spells the decisions in `ledger.pending`. The sweep's are
  written before the held pierce mints (`acc.finish()` precedes
  `vtxfac::faces_holding_pierce` in `boolean/mod.rs`), so none names
  the held vertex.
- A ring tie names it beside the pierce it shares the point with. Its
  `through` is the ring's pierced face (`ops.rs` `ledger`, `pierced`),
  and that face is the held pierce's own (`ring_groups` ties pierces of
  one face only).
- `ops.rs` `carried`'s minted meeting is the one other writer. A pair
  that is a tie folds onto the tie's decision (`reduce::pend`), which
  keeps its `through`. A meeting needs both cells minted
  (`records::landings`, `minted`), so any other partner is a vertex
  minted on an edge whose crossing there was the pierced face, the
  only cell of the other operand at a point inside it, and its `by`
  names that face. A second held vertex cannot be the partner:
  `faces_holding_pierce` refuses two pierces of one face at one point
  (`HeldPierceUnbuilt`), and two faces of one valid operand cannot both
  hold the point in their interiors.

What the argument leaves open: an other operand that touches itself at
the point (a lump's edge lying in the pierced face there), where a
split's `by` may name that edge; and a node's ring vertex as the
partner, which `Mints` does not record at all
(`a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row`). The
first needs three of the piercing operand's lumps at the point as well,
since only a group of three pierces has a pair that is not a tie.

Measured: with the minting cell kept in `push_in_face` and `cite_rows`
logging every row on a vertex minted in a face, `sweep`'s
`a_pierce_held_in_a_curved_face` (4 tests) logged 160 such rows, every
one a ring tie with `through`, none spelled differently. The new
plane-face fixture logged 4. No carried minted meeting was logged.

Pinned: `union_member_orders_decide_alike::a_pierce_held_in_a_face_is_one_statement_in_every_member_order`
on `union_pinch_member_order::edge_in_a_wall`. The point is one
statement, `Meet {Edge@wedge, Face@plate, Face@wall}`, in all six
member orders, two of which hold the pierce in the wall's face. With
the ring tie's `through` dropped, the test stays green: the tie's
partner is the edge split, whose `by` names the plate's top.
