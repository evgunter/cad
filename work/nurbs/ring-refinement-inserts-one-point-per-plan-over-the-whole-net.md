---
id: ring-refinement-inserts-one-point-per-plan-over-the-whole-net
kind: issue
title: RingRefinement inserts one point per plan over the whole net, so an equal split costs O(insertions × n) to plan and to apply
status: deferred
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
`cert_grid`) and `ssi/enclose.rs` (`refine_u` / `refine_v`), and in
`geom-core`'s `spline/net.rs` equal-split grid (`equal_split`).
`compose.rs`, `patch_bound.rs` and `offset_fit.rs` take only
`equal_split_points`, the schedule, not the chain.

## Shape

Insert all of a span's points at once (Boehm's multi-insertion, or the
Oslo algorithm), or refine per span from its local window: `O(n·k·p)`.
Measure the remaining callers first; at the sizes they see the cost may
not matter.

## Deferred (2026-10-12, the NURBS orchestrator's ruling)

Measured with the speed meter's refined scan removed (PR 4860), by
timing `RingRefinement::new` / `equal_split` (planning) and
`RingRefinement::refine` (applying) inside real workloads, release:

| workload | wall | planning | applying | calls |
|---|---|---|---|---|
| sweep thread-count digest (`the_walk_matches_the_serial_golden_at_four_threads`, ε 1e-9; times summed over its 4 threads) | 22 501 ms | 0.8 ms | 12.3 ms | 44 |
| NURBS SSI (`m8_4_intersection_iso::a_degree_two_widening_measures_against_the_oracle`) | 1 486 ms | 1.0 ms | 8.8 ms | 34 |
| rational mass properties (`m8_3_rational_volume::tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`) | 6 576 ms | 0.1 ms | 2.7 ms | 8 |

At most 0.7% of wall time: the remaining callers (`props/quad.rs`,
`ssi/section.rs`, `ssi/certify.rs`, `ssi/enclose.rs`, `spline/net.rs`)
refine short vectors. The quadratic chain stays, disclosed; the linear
per-span refinement above is the fix if a caller ever refines a long
carrier.
