---
id: a-moved-rims-radius-is-positive-by-agreement-not-by-a-decision
kind: issue
title: transport_curve and offset_axial add the offset to a rim circle's radius with no decision, so a collapsing rim panics in Circle::new
status: open
opened: 2026-11-20
priority: P2
cost: M
---

A rim circle moved by an offset is built as `Curve3::circle(.., radius + d, ..)` with no decision on the sum, so an offset that takes a rim's radius definitely negative panics in `geom::Circle::new` (it panics where the radius is not a finite length or is definitely negative) on an input path, where the surface beside it refuses typed. Found by the review of PR 7143 (`oneround/lever-doors`).

**The surface decides, the rim does not.** `geom_brep::offset`'s `offset_surface` passes each moved radius through `offset_radius_floor` (`Positive`, else `OffsetError::RadiusFloor`) before it builds the cylinder, sphere or torus.

**The undecided sums**, each read in the tree:
- `topo::replace_face::transport_curve`, cylinder arm: `let radius = conic_k.radius(); Curve3::circle(*center, *ca, radius + d, *u_ref)`. The radius is the rim CIRCLE's own; the arm never binds the surface's. That the two are the same radius is an assumption, not something the code states.
- `transport_curve`, sphere arm: the homothety ratio `k = (*radius + d) / *radius` of the surface's radius, applied to the rim with no decision on its sign.
- `transport_curve`, torus arms: a parallel circle at `*major_radius + r_new * cos_v` with `r_new = *minor_radius + d`, and a meridian circle at `*minor_radius + d`.
- `topo::offset_axial::mint_carrier`, twice: `Curve3::circle(*cc, *axis, radius + ca.distance, *u_ref)` for a sphere seam and for a torus seam, `radius` the seam circle's own, after a decision on its centre and none on the sum.

So a rim's radius is non-negative by agreement with the surface's floor, not by a decision where it is formed. Where the agreement fails (a rim whose radius is not the surface's, or a caller that moves a curve without the surface's floor having run), the failure is a panic at the constructor.

**Wanted:** the rim's moved radius is decided where it is formed and refuses typed, or the mover takes the floor's verdict as an argument so the tie is in the signature.
