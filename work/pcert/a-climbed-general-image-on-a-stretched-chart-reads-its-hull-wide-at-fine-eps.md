---
id: a-climbed-general-image-on-a-stretched-chart-reads-its-hull-wide-at-fine-eps
kind: issue
title: A climbed General image on a stretched spline chart reads its hull 10^5 wider than its miss, and refuses at ε 1e-12
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Filed by `pcert/general-image-climbs-on-its-miss`, from its own row.

## What

A line on a flat wall whose `v` is quadratic in height (control rows at
`z = 0, 0.1, 1`, so `dz/dv` runs from 0.2 to 1.8) bends in the chart,
and the mint's `General` image climbs to
`edge_nurbs::general_chart_image`'s finest rung (degree 5, 257 control
points). That image follows the carrier: its miss, read at nine
stations, is at most 8e-16 m, the foot projection's own roundoff. At
ε 1e-12 its certificate still refuses: `FittedLimb { HullValueChart }`
reads 7.8e-11 m at `f64`, and the sampled residual escalates in band at
`Interval`. Both end the lane's gap, "not yet" (`LaneGap::Image`, or
`GeneralLane` escalated), as E1d asks, so this is a capability gap, not
a wrong ending. At ε 1e-9 and 1e-6 it certifies at both scalars.

The ring's curved spline arc on the same rungs certifies at 1e-12, so
the over-read follows the chart's stretch, not the climb.

Row: `pcurve_cache::general_climb_rows::the_bending_carriers_climb_*`
(`a line on the uneven wall`, asserted to certify only at
`band().zero() >= 1e-10`; the bound is this file's).

## Direction

Read where the hull limb's slack comes from on a stretched chart: the
composite `S(P(t))`'s per-span hull at degree 5 against a quadratic
wall, or the chart-to-metre conversion's speed bound. A rung the hull
reads tighter (fewer, lower-degree spans) may certify where the finest
does not; `general_chart_image` keeps its finest rung on a miss, never a
coarser one the certificate would take.
