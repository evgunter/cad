---
id: a-line-passing-a-cones-apex-off-the-cone-reads-on-to-the-sampled-incidence
kind: issue
title: A line passing a cone's apex off the cone reads on to the sampled incidence test, and ends at check 4 rather than as its incidence read
status: open
opened: 2026-11-04
priority: P1
cost: M
---



Found in the review of PR 6504 (`pcert/one-ruling-reading`, fix list
AA2). It is pre-existing on main.

## What

The mint's incidence test for a line (`incidence_over_span`, through
`farthest_over_span`) samples the line's distance from the chart at
`INCIDENCE_SAMPLES + 1` (65) points of the span. A line that passes a
cone's apex a little to the side is off the cone only in a dip about
as wide as its offset, and every sample can miss that dip.

**Measured** on `ruling_reading_rows::cone()` (α 0.5), at ε 1e-9:
- the line is the generator at azimuth −2.5 or 3.1415, anchored at
  height 0.3, shifted 1e-7 along the rim's tangent, over `[−0.5, 0.5]`;
- it passes the apex 1e-7 to the side, so it is `s·cos α ≈ 8.8e-8`
  off the cone there: 88ε, past the band;
- the 65 samples read at most 4.8e-12, so the test shows the line on
  the cone.

The line then takes the general lane (its ruling gate reads definite).
The projected image mints at f64, and `PcurveCache::certify` refuses
`ResidualExceeded { check: MapResidual }`. At Interval the mint refuses
`SectorRefused { Lever }`, which topo's walk (`minted_refusal`) would
relabel "not yet" (`NoGeneralLane { Pose }`).

Neither is this carrier's ending. It is off the chart, and D4 ¶1 (iv)
ends a definite miss as its incidence read (`CarrierOffChart`), at the
door that reads it.

The same lines shifted `1.02` and `1.1·ε`, which pass within the zero
band of the cone, end "not yet" at both scalars
(`ruling_reading_rows::a_line_across_the_apex_that_is_no_generator_ends_not_yet`).
Shifted `30ε` at ε 1e-6, 26ε off the cone at the apex, the line meets
this same gap: the projected image mints at f64.

The test's own comment admits a miss inside the rounding ("the image
it gets is measured by its certificate"). This miss is 88ε, not
rounding.

## Fix

Bound a line's distance from a cone over the span rather than
sampling it: a line's sup on a cylinder is already read by
`projected::line_off_cylinder_sup`, and the cone's form sup by
`projected::line_off_cone_form`. Or take the apex's own distance from
the line as a sample wherever the apex projects into the span.

Row: the probe above, ending `CarrierOffChart` at both scalars.
