---
id: a-closed-conics-and-a-spline-nets-incidence-on-an-analytic-chart-is-sampled
kind: issue
title: A closed conic's and a spline net's incidence on an analytic chart, and every carrier's on a spline chart, is sampled, so an excursion narrower than the samples' spacing reads on
status: open
opened: 2026-11-04
priority: P2
cost: M
---



Filed by the PR that closed
`a-line-passing-a-cones-apex-off-the-cone-reads-on-to-the-sampled-incidence`.
That PR read a line's incidence at its critical points
(`pcurve_cache::line_incidence::critical_fractions`), so a line's read is
its distance's supremum over the span on every analytic chart. The other
carriers the sampled read (`pcurve_cache::incidence_over_span`, through
`sample_schedule`) serves are still sampled at `INCIDENCE_SAMPLES` + 1
points.

## What

The sampled read is a lower bound on the carrier's distance from the
chart, so a definite Off it reads is sound. A Zero it reads shows only
that no sample is off. Where the distance has a feature narrower than
the samples' spacing, a carrier off the chart by more than the band
reads on. It then takes the class or general lane and ends at the
image's certificate (check 4) or "not yet", where D4 ¶1 (iv) ends a
definite miss as its incidence read (`CarrierOffChart`).

The members, by where the distance can be narrow:

| carrier | chart | narrow feature | status |
|---|---|---|---|
| circle, ellipse | spindle (apple, lemon, horn) | the sheet's apexes on the axis, where `ρ`'s kink makes the distance from the revolved circle a cone-like spike: a conic passing an apex a little to the side | sampled; not measured for a conic, but the line's analogue at a cone's apex missed 88ε |
| circle, ellipse | ring torus | the axis (`ρ` kink) and the core circle, both at least `R − r` or `r` off the torus, so a sample beside them is off too | sampled; no in-band miss found |
| circle, ellipse | sphere, cylinder, plane | none: the squared distances are trigonometric polynomials of degree at most 2, whose excursion between 64 samples a turn is about `1e-3` of their amplitude; the kinks (centre, axis) are a radius off the chart | sampled; a Zero read can understate an in-band sup by that fraction, never a definite miss |
| circle, ellipse | cone | the axis: `cone_conic_incidence` first shows the conic's lowest point away from the apex's height, and a conic near the cone cannot cross the axis within one sample's spacing | sampled; no in-band miss found |
| every carrier, a line included | NURBS or Approx chart (`spline_incidence`) | the cell read bounds each sample's distance from the chart, not the carrier between samples, so a carrier that leaves a spline chart over a stretch narrower than the spacing (past a fold or a narrow ridge) | sampled along the carrier; not measured |
| spline net (`net_incidence`) | every analytic chart | any: a net's knot spans can be narrower than the spacing, and it can pass any singular point (cone apex, spindle apex) | sampled; not measured |

## Fix

As the line's read did: name the parameters where each carrier's
distance can peak and read it there, or bound it between samples. For a
conic the distance is a function of `(cos t, sin t)`; its stationary
points on a spindle are the roots of a trigonometric polynomial, and the
apexes' nearest approaches are the kinks. For a net, the spline chart's
certified cell read (`spline_incidence`) has the shape: a bound per
cell, subdivided where it does not decide.

Rows: a circle on an apple spindle passing an apex `30·zero` to the
side ends `CarrierOffChart` at both scalars; a net through a cone's
apex the same.

## Evidence: the edge certificate's derived chart image and two-surface `OnPair` inherit the sampled read

Added by `pcert/stated-image-bounded` (the row
`a-stated-chart-image-on-a-spline-face-is-certified-by-a-sampled-meter`).
That PR bounds a STATED chart image over its span through its pcurve
certificate (`certify.rs` `stated_bound`), and gives the spline chart's
read a spline carrier's own breakpoints and Greville abscissae
(`spline_incidence::own_fractions`). Two edge-certificate arms still
bound nothing between samples where this row's read does not:

- `certify.rs` `run_checks`, the `Resolved::Chart` arm on an analytic
  chart, a derived image or a stated one: `pcurve_map_residual` at the
  nine `CERT_SAMPLES`, after the read (`stated_door`). A stated image
  there is not routed through `PcurveCache::certify_stated` as on a spline
  chart: the stated images producers hand an analytic chart (an iso line
  on a plane, a harmonic image restated on a reverted or mirrored chart, a
  restriction placed in another window) are not all ones a pcurve door
  certifies, and about seventy rows across topo and sweep refuse
  `ChartImageUnavailable` when they are. Where
  the read is a supremum (a line's critical points) the residual's foot
  images inherit it; where the read is sampled (the table above: a conic
  on a spindle, a net on any analytic chart) neither the read nor the
  residual bounds the carrier between samples. The pcurve certificate the
  mint stores (`PcurveCache::certify`) carries an envelope, but the edge
  certificate is issued before it, and pncad re-exports
  `geom_brep::EdgeCurve::certify`.
- `run_checks`, the `Resolved::OnPair` arm on two analytic surfaces:
  `carrier_on_surface_1`/`_2` implicit residuals at the nine samples and
  nothing over the span (a spline carrier is refused there). A line on a
  plane is linear and the ends bound it; the conic rows of the table
  apply to the rest.

The fix this row names (a read that is a supremum) closes both, since
the derived image is the foot image of a carrier the read shows on.
