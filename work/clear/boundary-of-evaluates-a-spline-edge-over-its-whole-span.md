---
id: boundary-of-evaluates-a-spline-edge-over-its-whole-span
kind: issue
title: clearance's boundary_of encloses each edge by its carrier evaluated over the whole parameter span at Interval, the wide-argument read that encloses a spline surface cell by the whole line
status: open
opened: 2026-10-28
---


## Finding

`boundary_of` in `crates/topo/src/clearance.rs` encloses each boundary
edge by `curve.carrier().map_scalar(Interval::from_certified).eval(span)`
with `span` the edge's whole parameter interval. For a NURBS curve
carrier that is the same wide-argument evaluation that enclosed a NURBS
surface cell by the whole line (NURBSWIN: `inf` up to cells of 1/8 the
domain). The enclosure feeds only the analytic windows (a plane's
`u`/`v` range, a cone's reach), so a planar or quadric face with a
spline boundary edge would get an over-wide window, or refuse
`a face whose boundary carries no sound enclosure` on an infinite one.
Not measured on such a face: every loft and sweep fixture NURBSWIN ran
reads finite boundaries, and their walls' windows do not read it.

The likely fix is the curve's analogue of `Surface::rect_enclosure`
(the hull of the control points restricted to the span), or the
control hull of the whole carrier, which is a sound superset.

Filed by NURBSWIN (CLEAR), 2026-10-28.
