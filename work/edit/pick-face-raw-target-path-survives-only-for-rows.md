---
id: pick-face-raw-target-path-survives-only-for-rows
kind: issue
title: pick_face's raw PickTarget path has no non-test consumer, and its document half is a claim
status: open
opened: 2026-09-16
refs: [2773, 1098]
---

## What

`pick_face` (`crates/editor-core/src/resolve/pick.rs`) takes a slice of
`PickTarget`s, and a `PickTarget` has two mints: `NodePick::target`,
where the document, the node, the body and the mesh all come from one
tessellation, and `PickTarget::new`, where the caller declares them
over a `MeshPick` of its own.

**The raw mint has no consumer outside tests.** Measured on PR 2773's
tree: `crates/viewer` offers targets only through `NodePick::target`
(`pickindex.rs`, `PickIndex::pick_for`), `crates/pncad-py` does the
same (`py/pick.rs`, `t.inner.target()`), and `crates/pncad` does not
carry `MeshPick` at all, so a façade consumer cannot build one
(`select.rs`'s curation comment, and `pncad/tests/all.rs`'s roster).
Every caller of `PickTarget::new` is a row.

**And on that path the document half is a claim, not a check.** PR
2773 made the fields private, so a target minted by `NodePick::target`
cannot be taken apart and re-stamped (a `compile_fail,E0451` row sits
on the type). What remains is a caller who builds a `MeshPick` over one
document's mesh and declares it to be of another: `pick_face` compares
the declaration against the handed evaluation, not against the mesh,
so the door answers a name out of the other document's tables. The row
that measures it is
`edit_pair_apply_names::a_raw_target_is_a_claim_in_every_half`. This is
the same shape as the NODE half, which cannot be checked even in
principle (arena keys collide across sibling nodes of one document) —
issue #1098's residual raw-assembly class — and `PickTarget`'s docs and
A2a now say so in one voice.

## The question

Whether `NodePick::target` should be the SOLE mint — which would make
"the document half is checked" true of the type rather than of one
path, and would close #1098's raw-assembly class at the API rather than
documenting it.

## What that would cost, measured (PR 2773's fix pass)

Four rows, none re-expressible through `NodePick::target`:

- `review_gui1_r1::dyadic_battery_pins_faces_edges_corners_and_tiebreak`
  pairs a mesh scaled ×2 BY HAND with its node so the ray oracle can run in
  exact integers. A `NodePick` only ever indexes the node's own
  tessellation.
- `gui1_pick::unusable_nodes_surface_typed_errors` builds targets
  naming a failed, a poisoned and an absent node, so that `pick_face`'s
  standing ladder has rows. `NodePick::build` refuses all three and
  mints nothing. (The ladder would stay REACHABLE — a target minted at
  one picture and handed a later evaluation in which the node has since
  failed is admitted by the pairing and refused by standing — so the
  rows could be rebuilt around the later-evaluation admission, at the
  cost of being rows about a different thing.)
- `gui1_pick::node_pick_door_is_prepaired_and_typed` compares the
  door's target against the raw path, which would be gone.
- `gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`,
  #1098's ignored witness, is about raw assembly by construction.

So it is a change to `pick_face`'s public shape and to four TCOST/TINT
rows, not a one-line narrowing — which is why PR 2773 stopped at the
private fields and the honest contract, and left this here.

## Where it stands

`crates/editor-core/src/resolve/pick.rs` is EDIT's
(`work/edit/program.md` paths); the four rows are TCOST's and TINT's.
Found by lane `nodepick-fix` while taking the style review's N1 on PR
2773.
