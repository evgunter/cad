---
id: spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor
kind: issue
title: The spline face placement decides a point off its carrier on an uncharged f64 box floor
status: dispatched
opened: 2026-11-04
priority: P1
cost: M
branch: splice/spline-region-charge
---


Filed by `pcert/spline-chart-incidence` (PR 6498, review fix Y9, R1's MINOR-5).

## What

`topo/src/boolean/spline_region.rs` `read_capped` decides a point **off the
carrier** when every Bézier cell's floor decides past the band (`decide(CARRIER,
Margin::of(cell.hull_gap(q)), band)`). `Patch::hull_gap` is the distance to the
control points' box. At `f64` that box is computed from points cut by
`refine_knots` and de Casteljau (`Patch::split`), and nothing charges their
rounding.

PR 6498's spline incidence test measured the same uncharged `f64` box floor
exceeding a sample's distance from its chart. At band `1e-12`, on patches
`1e2`–`1e5` across or placed up to `1e5` away, it found false `Off`s past the
distance of up to 5.8e-12, the review's probe `probe_corner_tight` (913 of 4000
corner samples), and
`spline_incidence::rows::the_f64_charge_holds_where_its_rounding_is_a_bands_size`
under the `spline-charge-dropped` mutant. Here a false "off the carrier" places
a point outside a face it lies on. The kernel runs ε = 1e-12 (the nightly's
1e-12 row) and admits bodies of that scale, so the case is reachable.

## Fix

Charge the cut's rounding, as `geom_brep::pcurve_cache::spline_incidence` does:
a per-point first-order bound tracked through every blend (`blend`,
`point_rounding`), and `rounding_charge` of the reach for the read. Better,
read both walks through one home (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`).
