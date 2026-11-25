---
id: main-red-removed-members-refuses-three-slow-carry-rows
kind: issue
title: Main red since PR 7745: three slow-set carry rows refuse (two union_flush rows Stale{record:3}, face_face_touch Unread{record:18}) that pass on its parent
status: open
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
