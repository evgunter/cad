---
id: an-edge-or-face-record-follows-its-faces-key-through-a-division
kind: issue
title: An edge-face or face-face record follows its face's key through the section's division, not the piece the contact lies on
status: closed
opened: 2026-10-11
priority: P1
cost: M
closed: 2026-10-13
pr: 4893
---


Unmeasured, read. Found by FLUSH's sweep for
`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`
(PR 4650), corrected by its review r2.

## What

`boolean::ops::carry_rows` reads every record's cells into result keys.
Its vertex-on-face rows now read a divided face through its division
rows (`Descendants::holder`): the live piece whose interior holds the
vertex. Its last loop does not: `cell_rows`, the records of two edges
or faces (the joins' and each operand's carried ones), chase each cell
through `Descendants::live`, so a face cell is read by its key alone.
So are the edge-face pairs a vertex group implies, where one end of the
group lands on a face.

When the op divides a face such a record names (the join's chord-mef, a
pinch weld), the key stays on one piece, whichever it is. An edge-face
record can then name a piece the edge's interior does not meet, which
the census reads stale, or lose the record with a discarded key piece
while the piece the edge lies on survives, which is silent where the
census is blind (a same-solid curved pair). Face-face records the same.

## Owed

A pose that divides a face carrying an edge-face record, keeping the
piece the edge lies in while the key goes elsewhere (an operand's
carried edge-in-face record on a face a second body's section cuts),
measured on main; then the same lineage read with this record kind's
question: the live piece whose interior the edge's interior meets
(for a face-face record, the pieces that still overlap).

## Evidence (CONTACTHOLD, PR 4820)

The reduction's own records now read a face the join divided through
its pieces and chords (`ops::place_found`, the one carry's placement):
a vertex-face or two-cell row lands on each pair the census confirms.
An operand's carried edge-face and face-face records still never enter
the boolean (`work/wire/a-boolean-drops-its-operands-curve-and-patch-records.md`),
so that half stays open; no pose here was measured.

## 2026-10-13, closed as no path for the found half (FUSE, PR 4893)

The evidence note above (CONTACTHOLD, PR 4820) covers the reduction's
own records: a two-cell row on a face the join divided goes through
`boolean::ops::place_found`, which lands it on every pair of pieces
and chords the census confirms. No other edge-face or face-face record
reaches the carry's last loop:

- **No carried kind.** `ContactRecords::carried`
  (`crates/topo/src/boolean/records.rs`) gives an edge-face or
  face-face record no carried kind, so the carried two-cell rows are
  edge-edge only.
- **Vertex-group pairs.** An edge-face pair a vertex group implies has
  its face end from a vertex the merge pruned, or from a vertex-edge
  row's edge the merge swallowed. The merge's substitution row names
  the group's kept face, which is live and undivided after the merge,
  by construction.
- **Spans.** `record_spans` mints edge-face records from the result's
  geometry after the carry, and reads no key.

Measured on ea1088c5, before 4820, with instrumentation that was then
reverted: no face cell entered the two-cell loop. The run covered the
2,721-body lattice, the two-stage carried corpus (4,536 runs), a
hand-built edge-in-divided-face prism pose, the topo/sweep/editor-core
suites (8,342 tests) and 2,940 random star-prism scenes under four ops.

The carried edge-face and face-face half belongs to WIRE's
`a-boolean-drops-its-operands-curve-and-patch-records`. Its row now
says such a record, once it has a carried kind, lands through
`operand_fate` and `records::landings`.
