---
id: the-klein-loop-sweep-has-no-document-spelling
kind: issue
title: the klein scene's one-body loop sweep has no document spelling: the sweep is banked and no node interpolates a curve through points
status: open
opened: 2026-10-02
priority: P3
cost: H
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep`
unit (2026-10-02). The north-star audit's row 15 (`klein`) moved from
YES to NO on it (`docs/guide/north-star-audit.md`).

## What

The `klein` scene's top loop is now ONE `sweep::sweep_body` of an
annulus (`circle_split` × 4 per wall) along a spine built by
`geom::NurbsCurve3::interpolate` through 49 sampled points of two
tangent arcs, placed by `geom_core::linalg::frame::path_start_frame`
(`demos/tour/src/klein.rs`, `loop_spine` / `sweep_loop`). A Python
author cannot say it, for two reasons:

- the sweep: `Node::Sweep` exists in editor-core, but `wire_sweep`
  refuses unconditionally (`SWEEP_FRONTIER`, `editor-core/src/eval/wire.rs`),
  and `pncad.pyi` binds no constructor for it — the audit's G2, which
  already holds `lily`, `s_duct` and the twisted duct;
- the path: the recipe's path operand is a profile LOOP, and no node
  authors an open 3-D curve interpolated through points (nor joins two
  exact arcs into one). Even with the sweep un-banked, this loop's
  spine has no node.

`TestKlein` (`crates/pncad-py/tests/test_north_star.py`) now executes
the bulb only; the elbows it used to build left the scene.

## Done when

`klein`'s whole bottle is authorable from Python — the sweep node bound
and un-banked, and a path node that carries this spine (an
interpolated curve, or a join of two arcs) — and the audit's row 15
flips back to YES with `TestKlein` building the loop.

## A second scene on the same two doors

SHOW's `long-turn-helix-has-no-demo` (2026-10-03) put a six-turn
square-wire spring in `projectbox` (`demos/tour/src/projectbox.rs`,
`coil` / `spring`): one `sweep_body` along a helix interpolated through
32 points a turn. Same sweep bank, same missing interpolated-curve
node — or a helix node, which would carry the exact curve rather than
an interpolant of it. The audit's row 40 moved from YES to NO on it;
it flips back with this row, `TestProjectbox` building the spring.
