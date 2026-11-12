---
id: a-lanes-schedule-read-at-a-computed-station-widens-by-the-stations-arithmetic
kind: issue
title: Every lane but the stated route reads check 3 at a station computed at the scalar, so its enclosure carries the station's width
status: open
opened: 2026-11-12
priority: P3
cost: E
---

Filed by `pcert/cone-image-fine-band`, from its class sweep.

## What

`pcurve_cache.rs` `schedule_residuals` (check 3) reads `|S(P(t)) − C(t)|`
at each station of the shared schedule. The stated route
(`stated.rs` `certify_stated_bound`, through `EdgeSpan::stations`) now
reads at held stations: its ends as held, every interior station the
`f64` schedule over the span's outer ends, entered as a point
(`certify.rs` `held_station`). Every other caller still passes
`Stations::Computed(t0, t1)`, so at `Interval` an interior station
`t0 + (t1 − t0)·f` is a few ulps wide. The image and the carrier are
then evaluated at that box apart, and each de Boor level carries the
station's width divided by its knot span and multiplied by its control
coordinates. The difference of the two boxes is as wide as both.

On the PCERT seed that this branch pins
(`analytic_image_rows.rs` `a_cone_image_at_a_steep_sample_certifies`)
a 3-ulp station gave each point about 5e-13 of width and the read
[0, 1.03e-12]. Held at a point, the same read is 2.0e-13. On the stated
route, a 3000-seed sweep of `a_stated_image_on_its_carrier_certifies` at
ε 1e-12 found 333 seeds that refused a correct image: 233 on a cone, 57 on
a cylinder and 43 on a plane. Their reads went up to [0, 1.89e-11]. Held
stations removed all 333.

## Callers still computed

- `pcurve_cache.rs`: the closed-form engine's verdict and cross-check
  (two sites), the fitted lane (`run_fitted_checks`), and the general,
  chart-foot and projected-hull lanes.
- `pcurve_cache/projected.rs`: the projected envelope's cross-check.
- `pcurve_cache/spline_incidence.rs` (the even schedule and the
  carrier's own fractions): `carrier.eval(t0 + (t1 − t0)·f)` at the
  scalar, which is the incidence read's station computed by hand, outside
  `schedule_param`.

These doors are `T: Decide` doors without `Bounds`, so they have no
`f64` view of their ends to hold a station at. Either the callers pass
the span's `f64` ends in (as `EdgeSpan::outer` does for the stated
route), or `Decide` gains `Bounds` (every scalar that implements
`Decide` already implements `Bounds`).

A held station is `T::from_f64` of a point. At `Dual` or `Sym` that point
carries no derivative or symbolic channel, so a caller whose read
differentiates through `t` needs its station's channel kept.

## Why P3

No row has been seen red on these lanes. Their images are fitted or
projected at the schedule's own stations, or closed forms whose
evaluation does not run a de Boor recursion. The width is real, though,
and it grows with the knot density and the distance from the origin.
