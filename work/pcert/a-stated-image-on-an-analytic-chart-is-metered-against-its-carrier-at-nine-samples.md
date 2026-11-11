---
id: a-stated-image-on-an-analytic-chart-is-metered-against-its-carrier-at-nine-samples
kind: issue
title: A stated chart image on an analytic chart is metered against its carrier at the nine samples, so a net on its plane that leaves its image between them gets an edge certificate
status: closed
opened: 2026-11-11
priority: P1
cost: M
pr: 6836
closed: 2026-11-11
---


Filed by `pcert/analytic-net-bounded`, which closed
`a-net-carrier-stated-on-an-analytic-chart-is-certified-by-a-sampled-meter`
by bounding a stated net's incidence on an analytic chart over its span.
Incidence is not the whole of the stated image's claim: the image must
also follow the carrier.

## What

The witness net of `pcurve_cache/analytic_net_rows.rs` (`witness`, a
degree-1 polyline along the plane's ruling `y = 0.3` with vertices at
`0, 1/256, 1/128, 3/256, 1`) with its vertex at `t = 1/128` pushed 1 mm
**within the plane** (along `dir × n`, here `y`), stated by the straight
ruling as a `Pcurve::IsoLine` or a degree-1 `Pcurve::General`, gets an
edge certificate from `EdgeCurve::certify` at `f64` (executed on the
branch at ε 1e-9: `Ok(())` for both images). The net lies on the plane,
so the bounded incidence read (`pcurve_cache::read_net_bound`) shows it
on, correctly; but the stated image maps `t = 1/128` to a point 1 mm
from the carrier there.

## Trace

- `certify.rs` `run_checks`: on an analytic chart a stated image is read
  for incidence (`stated_analytic_bound`) and then metered against the
  carrier only by the schedule's `pcurve_map_residual`, at the nine
  `CERT_SAMPLES`. The bump lies between them.
- On a spline chart the image is certified over the span through its
  pcurve door (`stated_bound`, PR 6815). An analytic chart's stated image
  is not routed there (about seventy product rows refused when PR 6815
  tried, since not every stated restatement is an image class a pcurve
  door certifies).
- `pncad` re-exports `geom_brep` whole, so `EdgeCurve::certify` is a
  pncad door: the P1 criterion of the sibling rows.

## Class

A stated image's fidelity on an analytic chart is sampled for every
carrier whose image is not affine in its parameter along the carrier.
On a curved chart a tangential wiggle leaves the chart at second order
(`δ²/2R`), which the incidence read now catches for a net once it is
past the band; on a plane it does not leave the chart at all. A closed
form's stated image (a harmonic restatement) against a closed-form
carrier is a trigonometric family, which nine samples may or may not
determine; not measured.

## Fix

Bound `|C(t) − S(P(t))|` over the span for a stated image on an
analytic chart: route the image through its pcurve door where the door
certifies its class (`General`/`Fitted`/`IsoLine` against a net, whose
envelope is a sup), keeping the sampled meter only where the image and
carrier are closed forms whose residual nine samples are shown to bound.
Rows: the witness above on the plane, red on main; the PR 6815 refusal
census rerun on the routing.
