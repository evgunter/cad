---
id: contfp-has-no-typed-on-edge-row-for-an-elliptic-arc
kind: issue
title: contfp's boundary pre-pass has no typed OnEdge row for an elliptic arc edge: a point on one escalates bool_contfp_boundary from the region walk instead of naming the edge
status: open
opened: 2026-09-26
priority: P2
cost: D
---


Filed by CONTACT-4.

`boolean::contain::boundary_pre_pass` decides a `Line` edge by its
chord rows and a `Circle` edge through `point_on_arc`. It gives every
other carrier no verdict, because a conic's chord is a different curve.
On a planar face, an elliptical rim's chord runs through the face
interior. That covers `Ellipse` (a tilted cut through a cylinder), and
`Spiric` and `Nurbs` too.

For an `Ellipse`, the region walk that follows
(`splitting::containment::point_in_carrier_loop`) does read the edge on
its conic. A point on the arc comes back `OnBoundary`, and `contfp`
maps that to `ContainError::Escalated` (`bool_contfp_boundary`, margin
`Invalid`). So a point ON an elliptic edge is refused typed rather than
answered `OnEdge(edge)`. The reduction's callers split an edge at an
`OnEdge`, which is where the answer is needed.

Before CONTACT-4 the planar door read such an edge by its chord. A
point on the arc was then answered `In`/`Out` from the vertex polygon,
and a point on the chord (inside the face) was answered `OnEdge`. Both
were wrong answers. The refusal is the conservative remainder, not a
regression.

**Repair shape:** an ellipse arm beside `point_on_arc`. It needs the
distance to the ellipse, which is not closed-form, so it uses the walk's
unit-coordinate test (`ρ − 1` levered by the smaller semi-axis) and the
same `splitting::containment::arc_trim` for the trim. That needs the
walk's `ConicArc` frame to be reachable from `contain.rs`.
`reduce.rs`'s `split_other_at_point` refuses `Ellipse` separately
(`PointSplitCarrierUnsupported`), so both have to move before an
elliptic `OnEdge` is useful to the reduction.
