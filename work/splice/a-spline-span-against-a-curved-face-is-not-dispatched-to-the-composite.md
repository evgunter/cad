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

## Owed with the dispatch (from PR 5789's review)

- **The frame's rounding.** The composite is formed on the chart frame
  crossed into certification arithmetic from the frame's stored
  values. For a plane that is the plane itself. For a quadric or a
  torus the frame is an orthonormal twin, unit only to rounding, so the
  radial terms carry that rounding unmetered. Charge it, or form the
  frame in certification arithmetic, before a curved face is
  dispatched.
- **A tangency off the face's trim.** The planar lane refuses
  `SplineCrossingUnsettled` for any span the arm cannot settle, even
  where the tangency lies outside the face's trim. The analytic doors
  instead route an unsettled count through `carrier_touch::off_face`,
  which places the meeting off the face and passes. Route the arm's
  `Unsettled` the same way, for the plane too. This is over-refusal,
  not unsoundness.
