---
id: the-boolean-and-the-surgery-carry-place-a-divided-cells-record-by-two-routes
kind: issue
title: The boolean and the surgery carry place a divided cell's record among its pieces by two routes
status: closed
opened: 2026-10-11
priority: P1
cost: M
refs: [surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key]
closed: 2026-10-12
pr: 4820
branch: contacthold/one-record-carry-boolean
---

## What

D1's rule that a record on a divided cell lands on the pieces whose
interiors still meet has two homes.

- **The boolean** places a carried record per kind, in its operand
  clones, before the result exists: `ops::ledger` moves a `(vertex,
  edge)` record along the split log with `census::on_segment_interior`,
  `ops::ee_lineage` places an `(edge, edge)` record with
  `census::segment_interiors_meet`, and `Descendants::holder` places a
  `(vertex, face)` record among a divided face's pieces with
  `piece_holds` (`contain::face_placement`). Each asks one of the
  census's sub-questions, on straight segments or one face.
- **The surgery carry** (`ContactRecords::carried_through_pieces`,
  used by `SplitResult`, `Shelled` and `Blended`) places a record of
  any kind on the result: every pair of pieces is put to the census's
  own record confirm (`census::RecordConfirm`), and the pairs it
  confirms land.

Both answer "which pieces still hold this touch" by the census, so a
landing is what the census confirms either way. But they are two
implementations, and they already differ in reach: the boolean's
segment questions read only straight edges and do not place an
edge–face or face–face record at all (it does not carry those kinds,
`work/wire/a-boolean-drops-its-operands-curve-and-patch-records.md`),
while the surgery carry reads whatever the census confirms.

## The question

Whether the boolean's placement can be the surgery carry's, read after
the result exists. The boolean's routes also do what the carry does not:
`ledger` and `ee_lineage` record a vertex the reduction minted on a
carried touch, with a fresh decision where it meets a vertex. That is
the shape the split lacks for
`a-cut-through-a-recorded-touch-mints-a-pair-no-record-backs`, so the
two rows are best weighed together.

## Resolved (PR 4820)

Ev ruled on `[ev]` PR 4749: one carry. `ops::ledger`'s edge-split
placement, `ops::ee_lineage` and `Descendants::holder` retire; each
operand's carried records go through `ops::carried` (the boolean's
lineage, `ops::operand_fate`, into `records::landings`, the surgery
verbs' placement), the boolean deciding a minted vertex pair at its door.
