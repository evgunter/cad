---
id: the-add-to-the-product-door
kind: unit
title: one Add-to-the-product door, from a viewport pick, a tree row and every product recourse
status: dispatched
opened: 2026-11-01
priority: P1
cost: M
branch: spaces/product-door
refs: [6209]
---


## Spec

The contract is `crates/viewer/README.md`, *A body reaches the product
through one door*. One door, **Add to the product**, offered on the
body under a viewport pick, on a drawn body's tree row, and as the
action of the `EmptyProduct`, `StrandedProduct` and `unused` recourses:
a world copy → `SetProduct`; a placement's copy nothing in its space
reads (operand, mate pose, pick) → re-target the placement into the
world, delete a space the gesture minted and left empty, `SetProduct`;
anything else → a world `Place` by the identity bundle and that copy
added. Each is one action, one undo. Nothing joins the product unasked.

**Done when.** Each branch has a row; the door is reachable from a
viewport pick (today `product_ui` reads the minting feature, not the
drawn copy); each recourse offers it as an action; the `unused`
finding's recourse names it. Carries
`the-products-recourse-names-a-door-only-a-tree-click-reaches`.
Coupled with `a-new-body-lands-unplaced` (both edit `session.rs`'s
placement gestures): the later lane merges the earlier's branch and
says so.

Review tier: single, FULL.

**Fixtures it retires** (review of PR 6269): until this door exists,
rows place bodies into the world by hand — `story_assembly`'s
`into_world` / `reopened_with` (the windmill story reopens its file
mid-story to place the tower), `instance_authoring::into_world`,
`tests/common`'s `world_copy` / `world_copy_in`, `asm::store_world_copy`.
Rewrite the rows that stand for a person's path through the door.
