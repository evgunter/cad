---
id: main-red-an-editor-core-fold-refuses-its-plates-face-touch-unread-since-7745
kind: issue
title: Main red since PR 7745: editor-core union_fold_face_face_touch::a_piece_of_the_plates_face_still_holds_the_line refuses CarryRefused Found Unread, and PR 7799 did not mend it
status: closed
closed: 2026-11-27
opened: 2026-11-27
priority: P0
cost: E
refs: [7745, 7799, main-red-removed-members-refuses-three-slow-carry-rows]
---


Found in PR 7808's per-test evidence: red on its base and on its head. HELDRECORDS's orchestrator bisected it with `cargo nextest`, default dev profile:

| commit | result |
|---|---|
| `cc90e26a7a1` (PR 7745's first parent) | PASS |
| `0ac627ec92` (PR 7745) | **FAIL** |
| `9a25655c18` (PR 7799) | FAIL |

The witness, `editor-core::all union_fold_face_face_touch::a_piece_of_the_plates_face_still_holds_the_line`, refuses `CarryRefused Found Unread` on a face–face pair. It is the editor-core counterpart of the sweep row `face_face_touch::a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line`, which PR 7799's presence read mended. Neither PR 7745's evidence nor PR 7799's ran editor-core's slow set.

## Done when

The row builds as it did on `cc90e26a7a1`, with no record dropped and no rule loosened. The per-test editor-core default-profile diff against `cc90e26a7a1` also shows nothing else PR 7745 moved.

## 2026-11-27 — fixed (`heldrecords/editor-fold-red`)

- **Cause.** The refusal is on a kept pair, not a removed member. Since
  PR 7745 the plate face's division names the section the cut ran
  along. The output stage's coplanar merge then killed two of those
  section edges, and `Descendants::live` follows each one's
  substitution row to the face now holding its interior. So the
  division listed two result faces (one beyond the plate's pieces)
  among its minted cells.
- **Why it refused.** The record lands on the plate's piece that holds
  the line. Reading the face–face pair (rod wall, that merged face) is
  outside the census's lanes (plane × cylinder along a ruling), so the
  record refused `Unread` before its other pairs counted.
- **Fix.** `boolean::ops` `fate_of` drops a minted cell whose live image
  is a face: a division's minted cells are edges and vertices in the
  cell's interior. A face holding one is a piece where it holds the
  cell's interior, and beyond the cell otherwise. A touch that lay only
  on such a section would refuse `Stale`, loud, rather than drop.
- **Result.** The record lands on the plate's piece, as on
  `cc90e26a7a1`.
