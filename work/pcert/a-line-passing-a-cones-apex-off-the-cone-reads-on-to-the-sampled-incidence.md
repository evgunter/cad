---
id: a-line-passing-a-cones-apex-off-the-cone-reads-on-to-the-sampled-incidence
kind: issue
title: A line passing a cone's apex off the cone reads on to the sampled incidence test, and ends at check 4 rather than as its incidence read
status: closed
opened: 2026-11-04
priority: P1
cost: M
pr: 6527
closed: 2026-11-04
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

The same gap also ends an off carrier "not yet" at the mint itself,
not only after topo's relabel. The PR 6504 delta verifier measured it
on the shifted generator over `[−0.5, 0.5]`:
- at ε 1e-12, apex shifts of 3ε to 300ε (2.6ε to 263ε off the cone at
  the apex) end `NoGeneralLane { Pose }` directly at the mint, at both
  scalars;
- at ε 1e-9 that holds for 3ε to 30ε on azimuth −2.5, while azimuth
  3.1415 escalates.

Each of these is off the chart by more than the band, so "not yet" (a
lane's capability gap, lever-only) is as wrong an ending for it as the
check-4 miss is. It has the same root cause: the sampled read does not
show the carrier off.

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

## Closed

A line's incidence read is no longer sampled: `incident`'s line arm
reads the chart's exact distance at the span's ends and at every
parameter between where it can peak
(`pcurve_cache::line_incidence::critical_fractions`), so the read is
the distance's supremum over the span on every analytic chart:
- the plane at the ends;
- the sphere and cylinder at the foot of the centre or axis;
- the cone at the axis's nearest approach, the apex plane's crossing
  (the apex's own kink) and each side's stationary point;
- the torus and spindle at the axis's nearest approach and the
  stationary points of `q²`, isolated on a quartic and refined on the
  unsquared form.

The probe ends `CarrierOffChart` at both scalars, at the mint and at
the incidence door alone (`read_incidence`), at every ε where it is
past the band, and the verifier's in-band shifts (3 and 10·zero) end in
the incidence lever:
`ruling_reading_rows::a_line_passing_the_apex_off_the_cone_ends_as_its_incidence_read`,
red under the `sampled-line-incidence` mutant. A counterexample search
against an independent hunt of the distance
(`line_incidence_rows::no_parameter_of_the_span_is_farther_than_the_read`)
holds the read to the supremum on eight charts, and
`line_incidence_rows::a_line_on_the_chart_or_inside_the_zero_band_is_not_read_off`
holds the no-false-Off side.

The other carriers the sampled read serves (closed conics, spline nets
on analytic charts) are filed:
`a-closed-conics-and-a-spline-nets-incidence-on-an-analytic-chart-is-sampled`
(P2).
