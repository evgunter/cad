---
id: an-iso-arc-rims-end-reads-through-its-map-at-interval
kind: issue
title: An IsoArc rim's chart end is read through its map, which at Interval may not be a point
status: open
opened: 2026-11-14
priority: P3
cost: M
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

## From `boxfinish/iso-image` (the image type, 2026-11-22)

What that unit changed under this row, and what it left:

- **An arc image holds its line as structure too.** `Pcurve::IsoArc` is
  `{ line: ChartLine, g0, gd, t0, angle, breaks, ends }`: the row it
  runs along is `line`, exact where the mint placed it; its map along
  the row (`g0 + gd·g(t)`, `iso_arc_g`) is untouched.
- **A corner of an arc rim is no longer read through its map.** Every
  reader of a joint's chart point reads the loop's lift
  (`topo::pcurves::Lift::joint_point`, `exit_point`): where the arc's
  row meets a seam's column the corner is the meet of the two held
  values. `chart_region::pcurve_entry`'s structural read (`at0 − t0`
  and `angle − (t1 − t0)` as exact zeros) is gone with it.
- **Still this row's:** an arc image's END that is no corner (a split
  rim's cut, the joint between two arcs on one row) is the map's value,
  `iso_arc_g` through `tan`, clamped on one side, and at `Interval` with
  a span end that is no point it reads `[1 − w, 1]`. No `Interval` arc
  loft builds yet to measure it on.
- **Found beside it, not changed:** `run_iso_arc_checks` reads the arc
  as a row whatever its `line` says: it takes the image's `v` as the
  side (`side_of(p0.y, …)`) and pays the `v` displacement
  `stretch_v·|pd.y|` as drift slack, which for a row is an exact zero
  by structure now and for an arc held as a COLUMN is its whole run, so
  the envelope refuses a column arc by size, not by naming it. The line
  image's checks read the axis off the structure since that unit; the
  arc's are left for the unit that owns the arc's map.

## Re-banded P3 (2026-11-24, M10)

With the switch's box arm off on main after PR 7394, no arc rim reaches
check 7's quadrature: not in the box studies, the tour, or the suites
of the five crates M10 ran. So this row holds nothing by measurement,
and leaves the switch row's `blocked_on`. What it says of an arc
image's end that is no corner is still true by reading; it is a latent
gap (P3) until a document reaches it. (BOXFINISH orchestrator)
