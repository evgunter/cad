---
id: the-rational-meters-refined-scan-costs-190-ms-per-loft
kind: issue
title: The rational speed meter's refined-span scan costs about 190 ms per Interval loft, against a piece assembly that reaches as far
status: open
opened: 2026-10-12
priority: P3
cost: M
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
