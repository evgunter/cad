---
id: ring-refinement-inserts-one-point-per-plan-over-the-whole-net
kind: issue
title: RingRefinement inserts one point per plan over the whole net, so an equal split costs O(insertions × n) to plan and to apply
status: open
opened: 2026-10-12
priority: P4
cost: M
---

## What

`geom_core::spline::RingRefinement` (`crates/geom-core/src/spline/algebra.rs`)
is a chain of `CurvePlan`s, one per inserted point
(`refine_plan_homogeneous`), each carrying a whole new knot vector and
applied to a whole channel. An equal split of `s` spans into `k` pieces
inserts `s·(k − 1)` points, so planning and applying both cost
`O(s·k·n)`: quadratic in the span count.

Measured on the NURBS rational speed meter before it dropped its
refined scan (`nurbs/refined-scan-cost`): a 40-control-point cubic
(37 spans), 16 pieces per span, release, `Interval`, per call:
`RingRefinement::equal_split` 14.3 ms (pure `f64` structure),
`refine_homogeneous` 4.8 ms per channel, against 3.5 ms for the bound
read off the refined net. Its remaining callers are
`spline/net.rs` (`refine_u` / `refine_v` and the equal-split grid,
`CertGrid`-shaped consumers) and `compose.rs`'s split points.

## Shape

Insert all of a span's points at once (Boehm's multi-insertion, or the
Oslo algorithm), or refine per span from its local window: `O(n·k·p)`.
Measure the remaining callers first; at the sizes they see the cost may
not matter.
