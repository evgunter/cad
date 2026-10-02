---
id: chart-boundary-reads-an-on-locus-envelope-as-an-image-bound
kind: issue
title: chart_edge and the props quadrature lane read a fitted row's envelope as an image bound without reading its statement
status: open
opened: 2026-10-01
priority: P3
cost: E
---


Found by `pcert/general-circle-fitted-route` (PR 3733).

Two readers take a fitted row's `certificate().envelope` as a bound on
`sup |S(P(t)) − C(t)|`, the image's distance from its carrier, without
reading `certificate().statement`:

- `topo::pcurves::chart_edge` (the `Pcurve::Fitted(_) | Pcurve::General(_)`
  arm) widens the image's control hull by `slack: envelope`, and
  `chart_bound::MetredBound`'s `hull` docs (`crates/topo/src/chart_bound.rs`)
  read that slack as the image's allowance outside its hull;
- `topo::props::quad_lane` reads `envelope` as the map-residual boundary
  defect.

That reading is right for every statement but one. `MapResidualClosedForm`,
`MapResidualComposite`, `MapResidualIsoHull` and `MapResidualHermite`
bound exactly that quantity. `EnvelopeStatement::OnLocusHull` does not:
it bounds the CARRIER's distance from the chart surface, and the image's
own displacement is certified only at the `CERT_SAMPLES` schedule.

**What PR 3733 made true.** The rows a construction mints on an analytic
chart are now all image-bounded. A sphere's general circle (the oblique
fillet corner's octant) stores a `Fitted` row with `MapResidualHermite`,
a sound whole-span bound. Measured on the oblique trihedron's 10 rows:
the dense residual (20 001 points per row) is at most 1.3e-14 m, under
every envelope (at most 9.1e-11 m) and under the band at ε = 1e-9. So
the four octant faces whose `chart_boundary` now returns `Ok` are
described soundly.

**What remains.** An `OnLocusHull` row is a fitted image over a RUNG-3
carrier on an analytic chart. No kernel construction mints one (the
cyl×sphere germ-chord lane is banked); one reaches rest only through
`topo::Body::attach_pcurve`. Both readers would read its envelope as an
image bound.

Wanted: `chart_edge` and the quad lane read `statement` before
`envelope`, and refuse typed for `OnLocusHull` until a bound on that
image exists. CHART decides whether this row closes on that change or
waits for the germ-chord lane's first `OnLocusHull` producer.
