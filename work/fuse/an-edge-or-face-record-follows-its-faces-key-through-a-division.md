---
id: an-edge-or-face-record-follows-its-faces-key-through-a-division
kind: issue
title: An edge-face or face-face record follows its face's key through the section's division, not the piece the contact lies on
status: dispatched
opened: 2026-10-11
priority: P1
cost: M
branch: fuse/records-read-the-piece
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
