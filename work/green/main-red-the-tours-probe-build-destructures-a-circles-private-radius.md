---
id: main-red-the-tours-probe-build-destructures-a-circles-private-radius
kind: issue
title: Main red (nightly k-probe sweep): demos/tour does not build under --features probe, bodies.rs destructures Circle { radius, .. } and the field is private
status: closed
opened: 2026-11-21
priority: P1
cost: E
closed: 2026-11-22
pr: 7371
---


Reported by ONEROUND's `rounded-pairs` lane (2026-11-21) while taking the k-telemetry legs on main and on its head: the demo-scene leg does not build on either.

`demos/tour/src/bodies.rs` (the edge finder near line 561) matches `pncad::geom::Curve3::Circle(pncad::geom::Circle { radius, .. })` under `--features probe`. A conic's sizes are private on main (`geom: a conic's sizes are private, checked at construction; readers go through accessors`, `e771e8c78a`, landed with PR 7183, `germ/ellipse-speed-magnitudes`), so the pattern no longer compiles. The PR gate builds `demos/tour` only when a diff touches it, and not with `probe`, so the gate did not see it; the nightly's k-probe sweep does.

Not run by the orchestrator: the build failure is the lane's report, and the pattern and the private field were read on main `fdeff1ffd`.

## Owed

The reader goes through the conic's accessor (`Conic::semi_axes` or the circle's radius reading), as every other reader did in that PR. GERM's, as the PR's program.

## 2026-11-21: greened on main by PR 7226 (GERM)

GERM's commit `d393434fd5` ("demos/tour: a circle's radius is read through its accessor") reads `circle.radius()` in `demos/tour/src/bodies.rs` and landed with PR 7226 (`germ/crossing-pose-cross`). GREEN's PR (branch `green/small-reds`) swept for the rest of the class:

- **Pattern 1, struct patterns of the private conics**, `\b(Circle|Ellipse)\s*\{` across `demos tools benches interval-transcendentals`. 7 kernel-type hits (`bud.rs`, `verbs_teapot_r1_probes.rs`, `verbs_demo2_r1_probes.rs`, `common/rim_select.rs`, `verbs_teapot_r2_probes.rs`, `lily.rs` tests), and each binds only the public `frame` with `..`. The remaining hits are `LoopProgram::Circle` and lily's own `Circle` struct, which are other types. Nothing to fix.
- **Its blind spot is field access with no pattern.** Pattern 2 is `\.(radius|semi_u|semi_v)\b` not followed by `(` or at the end of a line. No hits.
