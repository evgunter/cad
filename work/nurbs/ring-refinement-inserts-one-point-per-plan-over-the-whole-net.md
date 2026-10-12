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
read off the refined net. Its remaining callers construct and apply
it in `geom-brep`: `props/quad.rs` (`RingRefinement::new`, `refine`,
`refine_u` / `refine_v`), `ssi/section.rs`, `ssi/certify.rs` (over
`cert_grid`) and `ssi/enclose.rs` (`refine_u` / `refine_v`), and
`patch_bound.rs`'s `patch_cells_refined` through `geom-core`'s
`TensorChannels::refine_equal_split` (`spline/net.rs`, `equal_split`),
which `mesh`'s `nurbs_cert` (rational patch certificates) and
`offset_meters.rs` reach.
`compose.rs`, `patch_bound.rs` and `offset_fit.rs` take only
`equal_split_points`, the schedule, not the chain.

## Shape

Insert all of a span's points at once (Boehm's multi-insertion, or the
Oslo algorithm), or refine per span from its local window: `O(n·k·p)`.
Measure the remaining callers first; at the sizes they see the cost may
not matter.
