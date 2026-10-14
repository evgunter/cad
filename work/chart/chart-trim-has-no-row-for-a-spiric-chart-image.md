---
id: chart-trim-has-no-row-for-a-spiric-chart-image
kind: issue
title: A curved face's chart trim has no row for a spiric chart image, so a torus wall or cap bounded by one cannot enter it
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Found by CLEAVE's sweep for the in-plane carrier walk's spiric row
(`work/cleave/carrier-walk-has-no-crossing-row-for-spiric-or-spline-edges.md`),
which answers a PLANAR face bounded by a spiric. The chart-space trim is
the curved faces' sibling: `chart_region`'s `pcurve_entry` reads each
edge's chart image as a straight segment, and refuses
`Pcurve::Spiric { .. }` with "Spiric image is not a straight segment".
The comment there names the planar trim limb's need for a
curved-boundary polygon as "the props/tessellation frontier, not a
missing arm here"; no row schedules it.

A spiric's chart image is `p0 + pm·f(t) + pa·sin t` on the cap and the
`atan2(f, d)` azimuth on the torus wall (`geom_brep::SpiricImage`). The
in-plane walk's row (`topo::splitting::spiric_arc`) certifies a ray ×
spiric crossing count by halving the arc against its speed and
acceleration bounds; the same device in chart coordinates needs those
bounds for the image, not the carrier.

**Reachability is unmeasured**: which public door reads a torus wall
bounded by a spiric through this trim (and whether an earlier gate
refuses first) has not been probed.
