---
id: the-spline-chart-arms-read-an-uncharged-f64-cut
kind: issue
title: A spline chart's arms are read off an f64 cut with no rounding charge
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by `splice/spline-region-charge`, from its sweep for the class "an f64 cut of a control net (knot insertion, de Casteljau, a window restriction, out of homogeneous form) whose box, hull or offsets decide, with no charge for the cut's rounding". That PR charges the spline face placement's two instances (`topo/src/boolean/spline_region.rs`: the carrier floor and the trim's pieces), where a point on a sheet `1e5` out read off its carrier and a query on a trim curve was given a side at the 1e-12 row. **This row was read, not reproduced.** Confidence: high that it is uncharged, low that it matters (the slack is relative).

## What

`topo/src/boolean/spline_region.rs` `window_arms` reads each Bézier cell
restricted to the window (`Patch::restricted`, then `Patch::surface`) into
`topo/src/chart_region.rs` `spline_chart_arms` (line ~2850) and `gate_arm`
(~2828), whose arms meter the trim's clearance decisions (`Trim::exits_clear`,
`Trim::copy`). The arms come from `geom_brep::chart_stretch_inf`
(`geom-brep/src/pcurve_cache.rs` ~7688), whose `floor` closure is the bracket's
lower end: at `f64` that is the value, so nothing is charged for the rounding of
the restricted net it is read off. The `chart_stretch_inf` docs call the `f64`
slack "a few ulps" and "not an argument". A speed is a difference of control
points, so the slack is relative: an arm over-read by a few ulps over-reads a
metered gap by the same few ulps of itself.

`splice/spline-region-charge` now carries a rounding bound on every point of
the restricted net (`Patch::net`, `point_rounding`). This row did not fold
it in: `spline_chart_arms` takes a `Surface` and has no door for a per-point
bound.

## Fix

Charge the arm for its net's rounding, the bound the cut now carries, or show
the relative slack below every band the arm meters, and say so where the arm
is gated.
