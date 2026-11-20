---
id: a-moved-rims-radius-is-positive-by-agreement-not-by-a-decision
kind: issue
title: transport_curve adds the offset to a rim circle's radius with no decision tying it to the surface's radius floor
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Found by the review of PR 7143 (`oneround/lever-doors`), behind the question of whether a curvature arm can be read off a negative radius.

`geom_brep::offset`'s `offset_surface` decides a moved radius: its cylinder, sphere and torus arms pass `radius + d` through `offset_radius_floor` (`Positive`, else `OffsetError::RadiusFloor`) before they build the surface.

`topo::replace_face::transport_curve` (`crates/topo/src/replace_face.rs`) moves the rim curves beside it. Its cylinder arm builds `Curve3::circle(*center, *ca, *radius + d, *u_ref)`, and its torus arm `Curve3::circle(*c, *ca, *minor_radius + d, *u_ref)`, with no decision on the sum. The rim's radius is positive because the surface's floor passed on the same `d` and the same radius, which the two functions agree on and nothing states.

So a rim circle's radius is positive by agreement, not by construction. A caller that transports a curve without the surface's floor having run, or with a rim whose radius is not the surface's, builds a circle of zero or negative radius, and `geom_brep::circle_curvature_arm` reads that as an arm.

**Wanted:** the rim's moved radius is decided where it is formed, or `transport_curve` takes the floor's verdict as an argument so the tie is in the signature.
