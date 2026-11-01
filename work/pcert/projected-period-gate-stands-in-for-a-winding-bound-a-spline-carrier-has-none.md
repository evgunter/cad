---
id: projected-period-gate-stands-in-for-a-winding-bound-a-spline-carrier-has-none
kind: issue
title: The projected lane's period gate is the only one-period statement for a spline carrier's image, which the edge certificate does not bound
status: open
opened: 2026-10-17
priority: P3
cost: M
---

Found by PCERT B2a's fix pass (PR 5244, the review's H3: "one rule").

## What is there

Every closed-form lane's image spans at most one period because its
carrier's interval does. The edge certificate decides
`interval_span_winding` for a circle, an ellipse and a spiric
(`crates/geom-brep/src/certify.rs`, `span_at_floor` and the circle arm,
refusing `CertifyError::WindingExceeded`), and each of those images'
angles advances monotonically one turn per carrier period. So the
harmonic, cone-section (focal) and spiric-wall lanes state no period
gate of their own: B2a dropped the harmonic lane's, and its fix pass the
focal and spiric-wall lanes'.

The projected lane (`crates/geom-brep/src/pcurve_cache/projected.rs`,
check 2) keeps `pcurve_azimuth_period` and its `TubePeriod` twin over the
image's swept box. It has to: its carrier is a sphere's general circle
or a spline net, and a net's parameter is no angle. The edge certificate
meters a net's span through its speed floor and bounds no winding, so a
net that winds twice around the axis over its domain would pass every
other check. For the net, this gate is the only one-period statement.

## What is owed

One rule, in one of two shapes:
- the edge certificate states a net's angular extent about a periodic
  surface's axis (a chart-side fact the edge certificate does not know
  today), and this gate goes like the others; or
- the gate stays and is named as the net's own statement. Then the
  sphere's general circle, which the edge certificate already bounds,
  stops paying for it (it reads the swept box, an enclosure, so it can
  refuse a circle whose true sweep is under a turn).

Either way the box's clearance is
`work/cleave/azimuth-period-gate-escalates-between-two-pass-readings.md`'s
remaining subject.
