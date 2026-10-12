---
id: a-divided-record-no-piece-pair-meets-falls-back-to-its-key
kind: issue
title: A carried record on a divided cell that no pair of pieces meets falls back to its cells' keys, which the census reads stale
status: open
opened: 2026-10-12
priority: P3
cost: E
---


Found by lane A's review of PR 4893, in the sweep for by-key reads of
a divided cell. Read, not measured.

## What

`ContactRecords::carried_onto` (`crates/topo/src/boolean/records.rs:488`)
carries an input record onto the bodies an op made. For a record on a
cell that `divisions` divides, it asks `landed_on_pieces` for every
pair of the two cells' pieces (each cell itself among them) whose
interiors meet, through `RecordConfirm::meets`. Where no pair meets on
any side, it falls through to `carried_row`
(`crates/topo/src/boolean/records.rs:605`), which maps each cell by
its key through the side's `image` and records that pair. The pair
reads no geometry.

`landed_on_pieces` already asked the key pair itself and found it does
not meet. The fallback therefore records a pair the confirm just read
as apart, and the census reads it stale (`StaleContactDeclaration`).
That is loud, not silent: the op ships a body the census refuses
instead of one that is wrong. But it is the by-key read this class of
rows retires.

## Owed

A record on a divided cell whose pieces meet nowhere left the result
with the part it was on: drop it, as `carry_rows` drops a vertex-on-face
record no live piece holds, or refuse typed where the confirm could not
read a pair. Keep the by-key carry for undivided cells only. Then a
witness: a split door's carried record on a cell the split divides,
landing on no meeting pair.
