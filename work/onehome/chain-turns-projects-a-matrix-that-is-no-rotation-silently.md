---
id: chain-turns-projects-a-matrix-that-is-no-rotation-silently
kind: issue
title: mate::chain_turns reads a turn chain off any 3x3 matrix, so a matrix that is no rotation is projected onto three angles with no refusal
status: open
opened: 2026-11-10
priority: P3
---


Filed by PR 6717 (ONEHOME rigid-motion, fork 258: "`chain_turns` takes `RigidMotion<f64>`, if small"). It was not small, so it is filed here.

`editor_core::mate::chain_turns` (`crates/editor-core/src/mate/value.rs:959`) reads `r = Rz(γ)·Ry(β)·Rx(α)` off any `Mat3<f64>`, using `atan2` over a handful of its entries. It never asks whether `r` is a rotation. A scaled, sheared or mirrored matrix is projected silently onto three angles, and the angles reproduce some other turn.

Its callers hand it matrices, not motions:
- `read_back`'s chain read, `chain_read(&at.linear, …)` (`value.rs:871`), over the `Affine3<f64>` a gesture stands a copy at;
- `test_support::euler_turns` (`crates/editor-core/src/test_support.rs:358`), over a test's stated columns;
- `crates/viewer/tests/bundle_editor.rs:270`, over a product of turns.

What is owed (the mate lane):
- Take `RigidMotion<f64>`, so each caller admits its matrix at the `f64` door first (`RigidMotion::check_rigid`) and a non-rigid one refuses there by name.
- `read_back` would then take the gesture's motion, not its map.
