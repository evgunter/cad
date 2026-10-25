---
id: rigid-motion-radian-rotation-doors-twin-their-angle-doors
kind: issue
title: RigidMotion's radian rotation doors (rotation, rotation_then_translation, rotation_about_axis) are their Angle twins at Angle::radians
status: open
opened: 2026-10-21
priority: P3
cost: M
---


`crates/geom-core/src/linalg/rigid_motion.rs`: `RigidMotion::rotation`, `rotation_then_translation` and `rotation_about_axis` take a bare `T` radian angle, and each is its `_by` twin (`rotation_by`, `rotation_then_translation_by`, `rotation_about_axis_by`) at `Angle::radians(t)`. Two doors per construction, one body. The same shape stands on `Mat3::rotation_about_unit` / `_by` and `Affine3`'s rotation doors. Found by the exact-frames designer pair (2026-10-21, `exact-frame-mints-cover-three-of-the-world-frames`). Owed: one door per construction taking `Angle`, every caller moved (a bare radian is `Angle::radians`). Callers in `editor-core`'s mate and placement code are INTENT's ground, so the sweep announces a seam there or waits for D′.
