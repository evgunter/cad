---
id: a-new-body-lands-unplaced
kind: unit
title: no gesture writes a world Place: each gesture lands what it makes as the viewer README says
status: review
opened: 2026-11-01
priority: P0
cost: H
branch: spaces/lands-unplaced
refs: [6209]
pr: 6269
---


## Spec

The contract is `crates/viewer/README.md`, *A new body lands unplaced*
and the free-move paragraph of *The picture draws bodies* (Ev on PR
6209, fork row 230). No gesture writes a world `Place` on the person's
behalf (`place_in_world`, `create_placed` go):

- construction, feature, boolean: the node alone; mirror writes
  `body ∪ image`;
- "Add part…": `new_space()` plus the instance into it by the identity
  bundle, one action, and the gesture writes the new space's display
  frame clear of what is drawn (display metadata, the kind a free-move
  writes); DI5's prune keeps a frame across an undo/redo of its space
  (drop it only once the next evaluation has settled);
- duplicate: of a copy, one more copy in that copy's space, stepped; of
  an unplaced body, an identity stand-in plus a stepped copy in its own
  space;
- transform: of an unplaced body, a placement into its own space by
  values; of a copy, an edit of its placement (PLACING's row);
- pattern, fused or not: lands where the pick was seen and takes the
  seen body over along the seen path (a placement's source, a `⊔`
  operand, the product slot), member 0 standing in for it — the
  carried row `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`'s
  half (a); its half (b) is superseded (nothing joins the product);
- projecting a pattern member writes nothing.

**Done when.** Every carried row's finding is gone, each with a row in
`crates/viewer/tests/`; a fresh extrude raises only `unused`; two "Add
part…" of one file are drawn apart and stay apart across undo/redo; no
copy carries two placements. Carries `a-new-body-lands-placed-in-the-world`,
`an-instances-pattern-draws-nothing`,
`the-pattern-tool-stands-member-zero-where-its-body-stands` and the
DOORS row named above. Coupled with PLACING (its re-point, delete and
transform rows act only on placements a person asked for after this).

Review tier: single, FULL (claims plus style): many gestures change at
once and each can leave a stray placement.

**Stacked on `the-picture-draws-bodies` (PR 6265; its dual review,
2026-11-01).** That unit draws a split or pattern in its target's own
space at the identity while the target's creation-time world `Place`
still stands, so a split or pattern of a fresh box makes the box
unpickable (`Err(Ambiguous)`), masked by a `SetBodyHidden{split}` in
`frame_policy.rs`. This unit removes the world `Place`, so it closes
that: build on PR 6265's branch, merge the two back to back, and add
a row that a split and a pattern of a fresh box leave every drawn body
pickable, with the `frame_policy.rs` hide removed.
