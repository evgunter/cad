---
id: an-edge-or-face-record-follows-its-faces-key-through-a-division
kind: issue
title: An edge-face or face-face record follows its face's key through the section's division, not the piece the contact lies on
status: closed
opened: 2026-10-11
priority: P1
cost: M
closed: 2026-10-12
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

## 2026-10-12, closed as no path (FUSE, `fuse/records-read-the-piece`)

No edge-face or face-face record reaches a divided face by key. The
argument is structural:

- **No carried kind.** `ContactRecords::carried`
  (`crates/topo/src/boolean/mod.rs`) gives an edge-face or face-face
  record no carried kind; `OperandRecords` holds `vv`, the
  vertex-on-face rows, `ve` and `ee`. So `carry_rows`' `cell_rows`
  (`crates/topo/src/boolean/ops.rs`) holds edge-edge rows only: the
  reduction discovers no edge-face or face-face pair (its kinds are
  vertex-vertex, vertex-on-face and edge-edge), and the carried ones
  are `ee`. The loop never chases a face cell.
- **Vertex-group pairs.** An edge-face pair a vertex group implies has
  its face end from a vertex end the merge pruned. The merge's
  substitution row names the group's kept face, which is live and
  undivided after the merge, by construction.
- **Spans.** `record_spans` mints edge-face records from the result's
  geometry, after the carry, so it reads no key.

Measured on ea1088c5, with instrumentation that was then reverted. It
counted a face cell entering `cell_rows`, and the vertex-on-face loop's
non-vertex arm reading a face the section divided. Across the
2,721-body lattice, the two-stage carried corpus (4,536 runs), a
hand-built edge-in-divided-face prism pose, the full
topo/sweep/editor-core suites (8,342 tests) and 2,940 random
star-prism scenes under four ops, it recorded 0 hits.

The obligation moves to where the trigger is: WIRE's
`a-boolean-drops-its-operands-curve-and-patch-records`, the row that
would give an edge-face or face-face record a carried kind, now carries
the line that its face read must go through the piece door.
