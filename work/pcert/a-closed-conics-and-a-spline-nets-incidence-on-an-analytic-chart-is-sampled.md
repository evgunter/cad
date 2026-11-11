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
| spline net (`net_sampled_incidence`) | every analytic chart | any: a net's knot spans can be narrower than the spacing, and it can pass any singular point (cone apex, spindle apex) | bounded at the edge certificate's stated-image read (`read_net_bound`, limb 2's sup over the span), the P1 below; still sampled at the mint and the lane choices, a lower bound whose images the pcurve certificates' envelopes bound |
| spiric (`incident_as`'s `_` arms) | plane, torus (its two charts) | the torus's axis and core circle, as for a conic on a ring torus | sampled; not measured (a sweep hit of the P1 below, not in this table before) |

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
apex the same. (The net's row is in `pcurve_cache/analytic_net_rows.rs`,
`a_net_beside_a_cones_apex_ends_off_*`, through `read_net_bound`: with a
knot beside the apex it ends off, and as one span it ends in the lever,
never certified, since the parts' floors beside the apex read no lever.)

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
  nine `CERT_SAMPLES`, after the read (`stated_analytic_read`). A stated image
  there is not routed through `PcurveCache::certify_stated_image` as on a spline
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
  `geom_brep::EdgeCurve::certify`. For a spline net stated on a plane, a
  cylinder or a cone this is witnessed, a 1 mm miss certified, and filed
  at P1: `a-net-carrier-stated-on-an-analytic-chart-is-certified-by-a-sampled-meter`.
- `run_checks`, the `Resolved::OnPair` arm on two analytic surfaces:
  `carrier_on_surface_1`/`_2` implicit residuals at the nine samples and
  nothing over the span (a spline carrier is refused there). A `Line`
  carrier on a plane is linear and the ends bound it; the conic rows of
  the table apply to the rest. (A spline net on a plane is not linear in
  this sense: it is the P1 row above, on the `Chart` arm.)

The fix this row names (a read that is a supremum) closes both, since
the derived image is the foot image of a carrier the read shows on.

## Evidence: the stated net's read is bounded at the edge certificate

Added by `pcert/analytic-net-bounded`, which closed
`a-net-carrier-stated-on-an-analytic-chart-is-certified-by-a-sampled-meter`.
`certify.rs` `stated_analytic_bound` reads a net stated on an analytic
chart through `pcurve_cache::read_net_bound`: the sampled read's definite
miss, then limb 2's bound over the span (`limb2_offset_sup`, decided by
`edge_nurbs::limb2`). That closes the net row above at the edge
certificate, on all five charts and the spindle. It does not touch:

- the conic rows and the spiric row: their stated-image read is still
  `incident_as`'s sampled `chart_incidence` (`cone_conic_incidence` on a
  cone). Limb 2's reader takes a closed form through its enclosing net,
  so the same read would bound them, at a cost on every stated arc of
  the product that was not measured here;
- the mint's and the lane choices' net read (`net_sampled_incidence`),
  a lower bound, whose images the pcurve doors bound;
- the `OnPair` arm, which refuses a spline carrier.
