---
id: a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite
kind: issue
title: The curved crossing lane refuses a spline span (EdgeCarrierUnsupported CurvedCrossing) though the ring-composite arm reads cylinders, cones, spheres and tori
status: open
opened: 2026-10-26
priority: P1
cost: M
---



Filed by `nurbs-edge-crossing-rung-is-the-ring-composite`'s lane: the
rung's curved half, left as its own item.

## What

`geom_brep::curve_face::composite::crossings` reads a spline span
against a cylinder, cone, sphere or torus as it does against a plane.
It composes each canonical form in its chart frame, and its rows
`a_rational_arc_crosses_a_plane_and_a_cylinder` and
`a_cubic_leaves_a_sphere_through_the_door` pin it. The boolean's curved
lane does not dispatch a spline span to it. `boolean/reduce.rs`'s
curved arm answers `(None, _)` with
`EdgeCarrierUnsupported { site: CurvedCrossing }`, and
`curve_face::route` marks `(Nurbs, Cylinder | Cone | Sphere | Torus)`
unbuilt.

## What it needs

Dispatch through `route`. Then the lane's rows for a spline: an end
on the carrier (`vertex_on_curved_face`), the one-sided cover, each
crossing placed against the face's trim (`curved_boundary_containment`)
and on a cone's nappe (`face_nappe_reading`), and a span lying on the
carrier, which the arm refuses as a tangency.
