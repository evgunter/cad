---
id: deleting-a-placed-feature-unplaces-its-target
kind: issue
title: Add a fillet then delete it and the body leaves the world: the add gesture re-points the target's placement, the delete cascades it
status: open
opened: 2026-10-08
---

Found by the review of INTENT stage 2 C (PR #4359, MINOR-4); filed on
the orchestrator's ruling.

Adding a fillet (any feature gesture) re-points the target body's world
placement to the fillet in the same action (`feature_over` in
`crates/viewer/src/session.rs`). Deleting the fillet
(`Session::delete_node` → `cascade_delete_order`) deletes the fillet's
placement with it, so the pre-fillet body is left unplaced and the part
vanishes from the viewport. Before C, A10's `on_delete` put the
target back in the root list.

Cascading is right for the kernel: re-pointing on delete would infer a
re-point, which DM6 forbids, and keeping the placement would strand it.
The gesture pair add → delete is simply no longer an identity on the
world. The fix is the gesture's: the viewer's delete of a feature could
explicitly re-point the feature's placements back to its target, as the
add gesture re-pointed them forward, in the same action.

`story_authoring`'s rook row was restated in C to expect the rook to
leave the world when its merlon block is deleted.
