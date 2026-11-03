---
id: a-substitution-row-names-a-kept-fragment-of-a-face-across-the-corner
kind: issue
title: A subtract's substitution row names a kept fragment of a face perpendicular to its 'by' face, so holdings cite it
status: closed
opened: 2026-11-03
priority: P1
closed: 2026-11-03
---


Found by PR 6381 (the per-face naming), which now reads `Holdings` and so
surfaces what a `SubstitutedFace` row says.

## Witness

The corpus document `crossing_slots` (`crates/editor-core/tests/corpus/slots.rs`),
rebuilt in topo with bricks: the plate `[0,3]² × [0,1]` less slot 1
`[1,2] × [−1,4] × [0.5,1.5]`, then less slot 2 `[−1,4] × [1,2] × [0.5,1.5]`.
The second subtract's records (keys as printed by a scratch probe beside
`boolean/holdings_rows.rs`):

- `substituted` holds `SubstitutedFace { operand: B, face: 9v1, by: 3v3 }`.
  `face_fragments_b` roots `9v1` at B's `3v1`, slot 2's `y = 1` wall
  (normal −y). `3v3` is the cavity floor (normal +z).
- `graft_faces` also lists `(9v1, 6v3)`: the fragment the row calls
  dropped was grafted into the result.
- `Holdings::of_records` reads the row as the floor holding the wall, so
  `holdings.of(3v3)` is {A's floor, B's floor, B's `y = 1` wall}, and the
  per-face naming publishes the cavity floor as `Merged` of three faces,
  one of them a wall that lies nowhere under it.

The other row, `(B, 7v3, by 3v3)`, roots at slot 2's floor and is right.

## Where to look

`fate.rs`'s tail, where `substituted` is assembled from `halves` and from
`red.fates.corners` (`sector_piece` on the dropped sector). A dropped
sector the kept copy holds lies on a face coplanar with the kept one; a
row naming a perpendicular face, and one the graft also keeps, breaks
that. Which arm wrote the row is not yet read.

## Done when

The row is gone or names the floor's own fragment, `crossing_slots`'
cavity floor holds exactly the two floors, and the corpus digest
`lib_g16_corpus_name_digests` is re-pinned with that name.

## Resolution (PR 6381)

Not the corner arm: both rows came from `fate::cut`'s fate edges. Each
fate edge of the witness (A's floor edges at x = 1 and x = 2) runs along
an edge the dropped floor already held, so `place` returned `Along` at
both ends, and the cut took the mate of the end-0 half because the
copies' normals are opposed. That mate lies in a null face the join set
between two pieces of the floor; null faces inherit the lineage of the
face they were split from, here slot 2's `y = 1` wall (`9v1`), and at
x = 2 the floor itself by chance (`7v3`). The half is now read from the
end that runs the wanted way (each `Along` names a half of a non-null
piece), and refuses as a classification invariant where it lies in no
piece. The second subtract now records one row, `(B, 14v3, by 3v3)`, the
floor's middle piece. A sweep of 720 brick booleans (plate and
plate-less-slot, subtract/union/intersect over a brick grid) found 15
such rows before the fix and none after
(`holdings_rows::every_brick_booleans_face_holds_only_member_faces_on_its_plane`).
`Holdings::of_records` now also reads each substitution and uncut row
against the finished face (carrier, then overlap) and refuses
`HeldRefusal::NotUnder` where a row contradicts the geometry.
