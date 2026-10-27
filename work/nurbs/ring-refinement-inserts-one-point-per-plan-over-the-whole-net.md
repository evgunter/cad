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
it in `geom-brep`: `props/quad/` (`RingRefinement::new`, `refine`,
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

## Deferred (2026-10-12, the NURBS orchestrator's ruling)

Measured with the speed meter's refined scan removed (PR 4860), by
timing `RingRefinement::new` / `equal_split` (planning) and
`RingRefinement::refine` (applying) inside real workloads, release:

| workload | wall | planning | applying | calls |
|---|---|---|---|---|
| sweep thread-count digest (`the_walk_matches_the_serial_golden_at_four_threads`, ε 1e-9; times summed over its 4 threads) | 22 501 ms | 0.8 ms | 12.3 ms | 44 |
| NURBS SSI (`m8_4_intersection_iso::a_degree_two_widening_measures_against_the_oracle`) | 1 486 ms | 1.0 ms | 8.8 ms | 34 |
| rational mass properties (`m8_3_rational_volume::tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`) | 6 576 ms | 0.1 ms | 2.7 ms | 8 |
| rational patch tessellation (`m8_4_intersection_iso::a_degree_two_widening_tessellates_against_the_oracle`, `patch_bound` via `refine_equal_split`) | 686 ms | 0.1 ms | 2.2 ms | |
| shell / offset meters (`encl_curved_loft_shell::a_moved_curved_fitted_cap_builds_where_its_fit_certifies`) | 4 436 ms | 5.6 ms | 12.5 ms | |
| shell / offset meters (`encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit`) | 31 744 ms | 12.0 ms | 23.9 ms | |

At most 0.7% of wall time on any of them: the remaining callers
(above) refine short vectors. The quadratic chain stays, disclosed; the linear
per-span refinement above is the fix if a caller ever refines a long
carrier.
