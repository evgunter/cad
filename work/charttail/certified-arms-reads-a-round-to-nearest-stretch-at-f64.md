---
id: certified-arms-reads-a-round-to-nearest-stretch-at-f64
kind: issue
title: chart_region::certified_arms calls its spline arm certified, but at f64 it is built from a round-to-nearest stretch sup
status: open
opened: 2026-10-14
priority: P4
cost: E
---


Found by PCERT closing `nurbs-stretch-bounds-round-to-nearest` (PR 5128's
review).

## What

`topo::chart_region::certified_arms`
(`crates/topo/src/chart_region.rs`, the `Surface::Nurbs | Surface::Approx`
arm):
- It reads `geom_brep::chart_stretch_inf`'s `sup_u` and `sup_v` (geom's
  `NurbsSurface::stretch_sup`, through `nurbs_stretch_bounds`) into
  `trace_sup = (sup_u/inf_u)² + (sup_v/inf_v)²`.
- The skew discount `ρ`, the smallest singular value from that trace and
  `det_inf`, rises as the trace falls, so a sup read low raises the arm.

At `Interval` the sup is an outward enclosure, and the arm is a bound.
At `f64`, `stretch_sup` is the same expression in round-to-nearest (its
doc says so). So is everything around it: the per-axis infs, `area_inf`,
and the trace and determinant arithmetic. Under D4 ¶2 the f64 lane is an
estimate, and the f64 arm can come out above the true one by rounding.

PCERT's census (the `nurbs-stretch` closure) found the f64 sup inside the
Interval enclosure every time, about one ulp below its upper end at the
median. So the effect is ulp-scale.

## Question

The function is named, and documented, as producing *certified* arms at
every scalar. Either:
- state that at f64 the arms are D4 ¶2 estimates, and only Interval
  certifies; or
- make the f64 lane outward where the claim matters.

This is the same shape as EXCH's
`surface-promotion-claims-a-certificate-from-a-round-to-nearest-bound`.
