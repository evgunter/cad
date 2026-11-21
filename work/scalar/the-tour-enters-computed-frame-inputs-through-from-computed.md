---
id: the-tour-enters-computed-frame-inputs-through-from-computed
kind: issue
title: The tour enters a swept frame's computed inputs through Real::from_computed, 18 calls in three scenes
status: open
opened: 2026-11-21
priority: P2
cost: E
refs: [f64-built-data-is-built-at-t, a-solved-value-the-body-stores-is-admitted-by-its-check]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-21, which took `ConstructedLoop::lift_computed` out of the tour (the swept outlines are drawn at `S`) and left these.

`demos/tour` calls `S::from_computed` directly on values it computes at `f64`: a blade's or a leaf's frame inputs in `demos/tour/src/lily.rs` (`leaf`, `blade_station`, and three earlier frames, 14 calls), a spine's start point and tangent in `demos/tour/src/klein.rs` (`swept_loop`, 2) and `demos/tour/src/projectbox.rs` (`spring`, 2). Each is a point, an axis or a tangent the scene derives from authored `f64` numbers or reads off an `f64` spine (`NurbsCurve3::ders1`), then hands to `sketch_frame` or `frame_across` at `S`.

`a-solved-value-the-body-stores-is-admitted-by-its-check` gates `from_computed` to one caller. The tour is outside `--workspace` and outside that gate's crates, so the gate will not see these; if `from_computed` stops being callable from outside the kernel, the tour stops compiling.

What each becomes, by DL5.2's table: the frame inputs derived from authored numbers are built at `S` (the derivation is a few products and a `sin_cos`); a start point and tangent read off the spine are functions of the `f64` path alone, a constant of the model as the path is (`sweep_body` takes the path as `f64`), and enter as constants. A user writing the scene would not reach for `from_computed` at all, so the demo's spelling here is evidence about the demo.
