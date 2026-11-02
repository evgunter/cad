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
domain). `window_of` calls `boundary_of` for EVERY face, a NURBS
wall included, and refuses `a face whose boundary carries no sound
enclosure` when it returns none, so an infinite edge enclosure would
refuse the face whatever its carrier. The extent it builds is read
only by the analytic windows (a plane's `u`/`v` range, a cone's
reach), where an over-wide one widens the window. Not measured on any
face: every loft and sweep fixture NURBSWIN ran reads finite
boundaries.

The likely fix is the curve's analogue of `Surface::rect_enclosure`
(the hull of the control points restricted to the span), or the
control hull of the whole carrier, which is a sound superset.

Filed by NURBSWIN (CLEAR), 2026-10-28.
