---
id: a-curved-partner-beside-a-spline-certifies-as-a-witness-intersection-alone
kind: issue
title: A sphere, cylinder or torus beside a spline chart certifies at rest as a witness-selected Intersection alone: OnPair, TangentIntersection and a shared row still refuse Unimplemented
status: open
opened: 2026-11-12
priority: P3
cost: M
---


Filed by the `sectarms/spline-curved-at-rest` lane, from its sweep of
`geom_brep::certify`'s `resolve_spec`.

## What

`resolve_spec` now sends a witness-selected `Intersection` of a sphere,
cylinder or torus and a spline chart to the residual × NURBS lane
(`residual_nurbs_pair`, `edge_nurbs::curved_nurbs_limbs`). Its three
sibling arms still pair a spline chart with a plane or with another
spline alone, so the same two faces refuse `CertifyError::Unimplemented`
under any other description:

- `OnPair`: `plane_nurbs_pair` takes a plane; a curved partner falls to
  `resolve`, which refuses the spline side. The incidence lane
  (`edge_nurbs::plane_nurbs_incidence`) is a plane's by its door
  (`plane_lane_door`).
- `TangentIntersection`: both sides go through `resolve`.
- `Intersection` under a shared row: the walls are both spline or both
  analytic.

No row reaches any of the three with a curved partner today: the join
mints a carried chord as a witness-selected `Intersection`. A sphere
touching a spline wall along a curve (C7) or a rim a curved face shares
with a spline wall as a row would.
