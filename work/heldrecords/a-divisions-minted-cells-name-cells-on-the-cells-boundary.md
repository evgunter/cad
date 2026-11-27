---
id: a-divisions-minted-cells-name-cells-on-the-cells-boundary
kind: issue
title: Since PR 7745 a boolean division's minted cells name cells on the cell's boundary, not only in its interior, and one out-of-lane pair on them refuses the whole record unread
status: open
opened: 2026-11-27
priority: P1
cost: M
---


Found by the review of PR 7825 (`heldrecords/editor-fold-red`, the
probes on `heldrecords/review-editor-fold-red-5f04e9`); filed by that
lane's follow-up.

## What

- **The invariant.** `lineage::Division::minted` is "the cells the op
  minted in the cell's interior between its pieces"
  (`crates/topo/src/lineage.rs`, the field's doc).
- **What breaks it.** Since PR 7745, the boolean's division of a face
  also names its section (`boolean::ops` `Incidence::chords`, the
  `section` rule: an edge or vertex beside a join's null face whose real
  faces are all fragments). A section edge can lie on the face's
  **boundary**: the patch a flush block rests on has its sides on the
  plate face's boundary. So `minted` names boundary cells.
- **What PR 7825 did.** It normalises only the case where such a cell's
  live image is a face (`fate_of` drops a minted cell whose live image is
  a face, after the output stage's merge took it in). Boundary minted
  *edges* remain.
- **Why the next one reds the same way.** `boolean::records::landings`
  reads every pair of a record's images. A pair outside the census's
  lanes refuses the whole record `Unread` (the `?` on the kept-pair
  confirm) before its other pairs count. So the next boundary edge whose
  pair with the other record cell is out of lane refuses a record that
  lands elsewhere. That is the shape of
  `main-red-an-editor-core-fold-refuses-its-plates-face-touch-unread-since-7745`.

## Way through

1. **Restore the interior invariant upstream.** `face_pieces` and
   `Incidence::chords`' section rule should name a section cell only
   where it lies in the divided face's interior (beside a null face on
   one side and a fragment of the same face on the other, never the
   face's original boundary).
2. **Then reconsider the `fate_of` filter (S6)**: with the section rule
   interior-only it should never fire, and can become a debug assertion
   or go.
3. **Then reconsider the two homes the PR 7825 review names for
   normalising a division (S1)**, so one place owns it.
4. **Fold in S2.** A minted face dropped by `fate_of` should lie beyond
   the pieces and be the image of a boundary cell. Assert it in debug
   builds while the filter stands, or drop the filter once step 1 holds.

## Pins

- `sweep face_face_touch::a_{tall,deep,short}_flush_block_leaves_the_rods_touch_on_the_plates_piece`
  (`ci`) go red with the `fate_of` filter reverted.
- A boundary-edge witness that refuses `Unread` today, if step 1 finds
  one, is this row's red.
