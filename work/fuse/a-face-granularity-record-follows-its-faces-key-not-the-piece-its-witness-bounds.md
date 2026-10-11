---
id: a-face-granularity-record-follows-its-faces-key-not-the-piece-its-witness-bounds
kind: issue
title: A curve or patch record follows its faces' keys through a division, not the pieces its witness or overlap lies on
status: open
opened: 2026-10-11
priority: P1
cost: M
---


Unmeasured, read. Found by FLUSH's sweep for
`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`.

## What

`boolean::ops::carry_rows` carries a `CurveContact` (`face_a`, `face_b`,
`witness`) and a `PatchContact` (`face_a`, `face_b`) by chasing each
face through `Descendants::live`, by key. When the op divides one of
those faces (the join's chord-mef, a pinch weld), the key stays on one
piece, whichever it is: the record can name a piece the witness edge
does not bound, or one the patch does not overlap, and keep a record
the census then finds stale; or lose the record with a discarded key
piece while the piece that holds the contact survives.

The vertex-on-face rows of the same loop had exactly this defect and
now read their face through the division rows
(`Descendants::holder`, `Descendants::fragments`). The face-granularity
rows want the same lineage with their own question: for a curve
record, the live piece of each face its live witness edge bounds; for
a patch record, the pieces that still overlap.

## Owed

A pose that divides a face carrying a curve or patch record and keeps
the record's other piece (a cylinder's wall resting on a cradle, cut by
a third body's section), measured on main; the fix at the same site.
