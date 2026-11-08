---
id: a-point-the-pre-pass-places-on-a-spline-boundary-edge-has-no-split-parameter
kind: issue
title: A point the boundary pre-pass places on a spline boundary edge, with no meeting behind it, refuses PointSplitCarrierUnsupported: the pre-pass holds no parameter on the edge
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Filed by the split-parameter lane (`splice/split-parameter`), from its
sweep of the sites that split a boundary edge at a point.

## What

`topo::boolean::reduce::split_other_at_point` now takes the parameter a
meeting reading certified on the boundary edge, where the caller holds
one (`curve_face::Boundary::Meets`' `on_edge`, carried through
`CurvedEvent::Pierce` and `carrier_cross::BoundaryCrossing::At`). Its
other callers hold none. Each has a point the face's boundary pre-pass
(`contain::boundary_pre_pass`, through
`splitting::containment::LoopEdge::contact`) placed `OnEdge`, and the
pre-pass's reading of a spline edge (`PiecedArc::contact`) answers a
margin, not a parameter. So `point_param` asks `Curve3::param_near`,
and a spline carrier refuses `PointSplitCarrierUnsupported`:

- an end of an edge read against a spline face's interior placed on
  its boundary (`reduce::spline_interior`, the
  `CurvedPlacement::Trim(Some(FaceContainment::OnEdge(_)))` arm);
- `reduce::along_boundary`, an end of an edge lying along a spline
  face's boundary edge;
- `reduce::vertex_on_curved_face_at` and the planar `vertex_on_face`, a
  vertex of the other operand landing on a boundary edge;
- the conic × plane and line × plane roots in the sweep
  (`reduce::sweep_direction`'s `ConicPlaneMeet::Roots` arm and its line
  sibling), a root placed on a plane face's boundary edge.

`boolean::overlap::half_plane` asks the same of a point inside a
boundary edge and refuses `OverlapRefusal::Unread` on a spline.

## Witness

None measured. A loft's or a sweep's spline seam or section edge with an
operand vertex on it reaches these doors. A row would build one: a spline
sheet (`reduce::curved_lane_spline_rows::spline_sheet`) with a block
corner on its spline edge.

## The fork

Either the pre-pass's spline reading names the parameter it placed the
point at (the piece whose box held it, contracted to a point), or the
split projects the point onto the spline (`NurbsCurve3::project`, as
`step_import::knot_split::interior_param_at` does) under a certificate
that the foot is the point's. The first keeps one reading. The second
is a second reading of the same point, which `split_other_at_point`'s
callers avoid by construction where a meeting holds the parameter.
