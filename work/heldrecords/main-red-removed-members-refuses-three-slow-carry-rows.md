---
id: main-red-removed-members-refuses-three-slow-carry-rows
kind: issue
title: Main red since PR 7745: three slow-set carry rows refuse (two union_flush rows Stale{record:3}, face_face_touch Unread{record:18}) that pass on its parent
status: closed
closed: 2026-11-25
opened: 2026-11-25
priority: P0
cost: E
refs: [7745, 7782, a-read-record-is-consumed-by-any-removed-remainder-so-a-lineage-gap-drops-it-silently]
---


Found by the `heldrecords/one-list` lane (PR 7782), whose `nextest.toml` edit ran the whole tier. Confirmed by the orchestrator. On the PR 7745 merge (`0ac627ec92`) all three fail, and on its first parent all three pass:

- `topo::all union_flush_onto_edge_contact::two_dangling_null_edges_with_one_segment_build_in_every_op`: `CarryRefused { Carried(A), Stale { record: 3 } }`
- `topo::all union_flush_onto_edge_contact::every_tied_strut_witness_holds_with_its_vertex_pairs_reversed`: the same refusal
- `sweep::all face_face_touch::a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line`: `CarryRefused { Found, Unread { record: 18 } }`

All three are in the slow set, which PR 7745's evidence did not cover: that was `ci` plus the touched test files. The merge was a valid build moved to refusing, which PR 7745's own claims ruled out.

## Done when

All three build as they did on the parent, with no record dropped, or a move is argued against the ratified carry clause. Also, the full default profile of topo and sweep (the slow set included) on main is compared per test against the parent of `0ac627ec92`, and any further moved row is fixed or listed here.

## 2026-11-25 — fixed, one row left red (`heldrecords/removed-members-red`, PR 7799)

- **`union_flush` ×2.** The record is edge–edge. A piece of one edge's
  division is the other edge's own image, because the glue made one
  edge of two that overlapped. Glued structure is now an image of one
  record cell that is itself an image of the other (`OneCell`), piece or
  minted; incidence still counts for nothing.
- **`face_face_touch`.** The removed fragment is read in the clones'
  joint body, which holds two whole overlapping solids, and there the
  census contradicted the record's tangent class after finding its
  line in both faces. A removed member is now read for the touch's
  presence (`census::RecordConfirm::touches`):
  - a patch is found by its region overlap on one carrier, whichever
    way the senses point;
  - a tangent pair is found by its line inside both faces;
  - a ruling made before the touch is read (two carriers the tangent
    table calls not tangent) is unread, never found.

  A kept pair is still confirmed as before.
- **Left red:**
  `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`
  refuses `JoinDesync`. It is loud, not a wrong body, and it has been red
  on main since PR 7745. On `cc90e26a` it passed.
  - The per-test comparison found it: a minted meeting at a point, of a
    split vertex and a section edge, is not two vertices, and no door
    decides it.
  - Setting it aside beside another landing ((C)) changes the ratified
    clause ("decides every such meeting at its door"), so it is not
    built. The door decides the meeting instead. That is decided and
    lands after PR 7799 as
    `the-boolean-door-decides-no-minted-vertex-on-edge-meeting` (P1),
    the row that turns this one green.
- **The comparison** was the full default profile of topo and sweep,
  release profile with debug assertions, between `cc90e26a` and the fix.
  Outside the pierce row it moves no row from PASS to FAIL. The default
  profile skips the `#[ignore]` rows, so those are not compared.
