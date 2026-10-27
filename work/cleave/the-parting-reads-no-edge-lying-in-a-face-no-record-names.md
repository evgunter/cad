---
id: the-parting-reads-no-edge-lying-in-a-face-no-record-names
kind: issue
title: The Boolean's parting reads an edge lying in a face only where a record or its ends name the pair, so an unrecorded rider keeps one edge
status: open
opened: 2026-10-26
priority: P1
cost: M
---



Filed by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`
(PR 5793), a residue its rows disclose. P1: D1-wrong topology shipped
silently. Unmeasured: no fixture is known where the join leaves such an
edge without a record.

## What

`topo::boolean::slit::riders` asks a face of an edge only where
something names the pair: both ends lying on the face (by structure, a
v-f, v-v or v-e record), an e-f record, or a record of a crossing with
the face's loops. An edge whose interior lies in a face with neither
end on it and no record of the touch (D1 owes one: "every pair whose
interiors meet is recorded") is never asked, and keeps one edge even
where both its faces go into the face's material. The record producers
mint the e-f record for a line between two recorded bounds in a plane
face (`boolean::ops::record_spans`) and none for a curved face
(`work/contacthold/a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only.md`),
so the gap is the producers' as much as the reading's.

## Fix

Either let the reading find such an edge without a record (every edge
against every face its box meets, at the cost of the reads), or close
it at the producers so that every edge lying in a face carries its
e-f record, and assert that in the result gate.
