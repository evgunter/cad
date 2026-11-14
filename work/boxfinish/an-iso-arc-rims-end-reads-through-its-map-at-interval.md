---
id: an-iso-arc-rims-end-reads-through-its-map-at-interval
kind: issue
title: An IsoArc rim's chart end is read through its map, which at Interval may not be a point
status: open
opened: 2026-11-14
---


Found by `boxfinish/chart-ends`'s sweep. That unit made a line rim's
chart ends stored with its `Pcurve::IsoLine` image
(`Pcurve::iso_row`/`iso_column`, read by `Pcurve::span_ends`) so check
7's rectangle certificate reads them exactly at `Interval`. An arc rim's
`Pcurve::IsoArc` was left reading its ends through its map:
`span_ends` falls back to `eval`, and `iso_arc_g`
(`crates/geom-brep/src/pcurve_cache.rs`) computes `g = (t − t0)/angle`
through `tan`, clamped to `[0, 1]`. At `Interval` with a span end that
is not a point, the clamp closes one side only, so the end reads as
`[1 − w, 1]`, not `1`, and `topo::props::quad_lane::nurbs_face`'s
`exact` closure would refuse it as it refused the line rims.
`chart_region::pcurve_entry` reads the segment ends structurally only
where `at0 − t0` and `angle − (t1 − t0)` are exact zeros, which an
enclosure with width is not.

**Not measured.** On the switch-off probe (`boxfinish/probe-switch-off`)
with chart-ends merged, a two-section `Interval` loft over a profile
with one bulged side (bulge 1, v-degree 1) refuses at the skin door
first: `Skin(Structure(Built(CarrierWeightNotAPoint { index: 1 })))`.
So no `Interval` arc rim reaches the quadrature today, and whether its
end is a point there is inferred from the code, not read. The unit is to
measure it once an `Interval` arc loft builds (or through a fixture that
mints an `IsoArc` over a computed span directly), and if it is wide,
store the arc's segment ends the way `IsoLine` now does.
