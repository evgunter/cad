---
id: the-rational-meters-refined-scan-costs-190-ms-per-loft
kind: issue
title: The rational speed meter's refined-span scan costs about 190 ms per Interval loft, against a piece assembly that reaches as far
status: closed
opened: 2026-10-12
priority: P3
cost: M
branch: nurbs/refined-scan-cost
closed: 2026-10-12
---

## What

`NurbsCurve3::rational_speed_lower_bound` (`crates/geom/src/curves/nurbs.rs`)
joins two assemblies by `max`: `rational_refined_scan` (knot-insert every
span into `RATIONAL_METER_SPLITS` pieces in the homogeneous channels,
then the chord / `sup‖C − c‖·sup|w′|` bound per refined span) and
`rational_piece_assembly`. At `Interval` the refined scan dominates the
meter's cost. Measured on `nurbs/one-speed-meter-arm` (from main
`06bd618056`): an `Interval` loft of square sections swaying ±0.25,
release, median of 20, with every corner forced through each arm:

| meter on every corner | 40 sections, deg 3 | 24 sections, deg 5 |
|---|---|---|
| integral arm (16 pieces) | 11.1 ms | 10.8 ms |
| rational piece assembly only | 76.0 ms | 94.6 ms |
| full rational arm (refined + pieces) | 264 ms | 150 ms |

So the refined scan is about 190 ms of a 40-section rational loft
(4 corner carriers, one meter each since PR 4803's memo), and every
rational carrier on main pays it.

## Wanted

Measure what the refined scan still decides that the piece assembly
does not (the frontier and adversarial rows in
`tests/curves/m5_pr7_speed_meter.rs`, and a random sweep), then either
drop it, make it cheaper (its refinement goes through
`RingRefinement::equal_split` and `refine_homogeneous` at `Interval`),
or keep it and say what it buys.

## Closed (2026-10-12)

Dropped (the orchestrator's ruling (A) on the measurement). The
rational arm is now `rational_piece_assembly` alone behind its weight
licence; `rational_refined_scan`, `rational_span_scan`,
`rational_span_bound` and `RATIONAL_METER_SPLITS` are gone, and with
them `SpanLocate::refine_homogeneous` / `refine_weights`, which nothing
else called. What the scan still decided: on every
`m5_pr7_speed_meter.rs` fixture the pieces read at least as high; on
12 000 random rational nets it read higher on 125, never turning a
refusal into a positive answer, and every win above 1% was degree 2
(about 1% of degree-2 nets, worst 2.07×, pinned as
`a_degree_two_net_the_refined_scan_read_higher_still_meters`). Above
degree 24 the meter now refuses
(`a_rational_carrier_past_degree_24_refuses`). The tables and the
before/after loft cost are in the PR body. `RingRefinement`'s quadratic
chain, which the scan paid most of its cost in, stays for its other
callers: `ring-refinement-inserts-one-point-per-plan-over-the-whole-net`.
