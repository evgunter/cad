---
id: a-vertex-minted-inside-a-face-reads-as-that-face-alone
kind: issue
title: A boolean's row on a vertex it minted inside a face names the face and not the pierce that minted it
status: open
opened: 2026-10-26
priority: P1
cost: M
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
