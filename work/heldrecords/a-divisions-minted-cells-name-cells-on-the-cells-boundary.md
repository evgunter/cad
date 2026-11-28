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
3. **Then reconsider the two homes that normalise one list (S1).**
   - `boolean::ops` `fate_of` drops minted faces by dimension.
   - `boolean::records::landings` separately drops a minted cell whose
     joined image a piece also names (the `minted` computation at the
     head of its per-row loop).
   - `place_found`'s call to `fate_of` shares the first, and nothing
     ties it to the second.

   One place should own the normalisation.
4. **Fold in S2.** A minted face dropped by `fate_of` should lie beyond
   the pieces and be the image of a boundary cell. Assert it in debug
   builds while the filter stands, or drop the filter once step 1 holds.

## Pins

- `sweep face_face_touch::a_{tall,deep,short}_flush_block_leaves_the_rods_touch_on_the_plates_piece`
  (`ci`) go red with the `fate_of` filter reverted.
- A boundary-edge witness that refuses `Unread` today, if step 1 finds
  one, is this row's red.

## Outcome (`heldrecords/interior-minted`)

- **Step 1.** `Incidence::chords` (`crates/topo/src/boolean/ops.rs`)
  now names a section cell only when every twin of it across the null
  faces also bounds only the divided face's fragments. A twin is a
  clone cell on the same point keys. A section along the face's
  boundary has a twin on the neighbouring face, so it is none of the
  face's cells.
- **Steps 2 and 4.**
  - The `fate_of` filter is gone; a debug assertion stands in its place.
  - What the filter dropped on ci was always one of the division's own
    pieces (a probe panicking on a dropped face outside the pieces
    never fired). Each such face is a chord the merge took into the
    fragment it divides.
  - That case now falls under the shared rule below, and a face left
    among `minted` after it is a debug-assertion failure.
- **Step 3.**
  - The normalisation has one home, `boolean::records::minted_read`
    (`crates/topo/src/boolean/records.rs`): a minted cell that reads as a piece
    is that piece.
  - `fate_of` states its divisions through it, read as they stand, and
    `place_found` shares this through `fate_of`.
  - `landings` reads through it, through the joins.
- **S4.** With minted cells interior-only, four ci tests reach the
  kept-pair `?` in `landings`. Each one pins that `Unread` as
  expected:
  - three are in-lane reads: two band escalations and one contradicted
    verdict;
  - the one out-of-lane read is the fillet's own kept face pair (a
    conic trim with no region door, tracked by
    `work/charttail/a-planar-face-with-a-conic-trim-has-no-region-door.md`).

  No boundary image reaches it. The `?` rule is unchanged.
- **Pin.**
  `sweep face_face_touch::a_tall_flush_blocks_division_names_no_edge_on_the_plate_faces_boundary`
  (`ci`) reads the carry's divisions through the `sweep-testing`
  recorder `topo::arm_carried_minted`.
  - It is red in main's state (the old section rule plus the face
    filter), which names the edge (1, 2, 1)–(1.8, 2, 1).
  - The three flush rows stay green in that state.
- **Sweep slow set (the reach method).** Panic probes went at the two
  sites where the change alters a fate (a section cell the twin rule
  excludes, a minted cell the shared rule drops) and ran over the 1039
  slow `sweep::all` tests outside ci's default filter, at a 300 s cap.
  - 160 tests reached a changed site or the cap.
  - Those 160 ran unprobed on main and head at a 900 s cap. No test
    passes on main and fails on head.
  - The same three tests fail on both sides, and eight time out on both.
