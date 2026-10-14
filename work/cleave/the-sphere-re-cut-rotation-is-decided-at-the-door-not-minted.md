---
id: the-sphere-re-cut-rotation-is-decided-at-the-door-not-minted
kind: issue
title: the boolean's sphere re-cut builds its alignment rotation as twelve numbers and decides it at the rigid-motion door; its by-construction route is turned_between
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [SYM-17, the-sketch-plane-is-its-frame]
---

## What

`crates/topo/src/boolean/ops.rs`'s `apply_recuts` rotates a sphere
shell about its centre so the stored polar axis lands on the escape
normal. It builds the rotation algebraically, `R = I + K + K²/(1 + c)`
for `K = [â × n̂]×`, from bare unit-by-convention directions, composes
the seam spin into it, and since SYM-17 (PR #4197) admits the map at
`geom_core::RigidMotion::check_rigid` before `topo::transform_rigid`
moves the shell. Any refusal folds into `corrupt`, as it did through
`transform_rigid` before.

The rotation is rigid by construction over the reals, so the decision
re-derives what the construction guarantees, and at an enclosure scalar
it can refuse a rotation that is fine.

## What answers it

The by-construction route is `RigidMotion::turned_between` (the
Rodrigues-between form, one decision `1 + a·b > 0`), decided on the
`[ev]` commit `e7ad119a8e` and scheduled, unbuilt, on PATHS'
`the-sketch-plane-is-its-frame`. With it, and `â`, `n̂` and the seam
directions as `UnitVec3` witnesses, the re-cut mints its motion and
the door is not asked. Until then the door is the honest spelling.

