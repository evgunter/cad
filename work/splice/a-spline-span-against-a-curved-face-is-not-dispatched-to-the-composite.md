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
- **The metering divisor's region.** The arm meters `f` into a
  distance by an upper bound on the form's gradient over the net's
  hull (`projected::Meters::divisor`). `|f(q)| ≤ max‖∇f‖ · d(q)` holds
  along the segment from `q` to its nearest surface point, and that
  segment can leave the hull. A piece within the band of the surface
  keeps that segment inside the hull grown by the band, so bound the
  gradient over that grown box. For the plane the divisor is one and
  the point is moot. (Orchestrator, reading PR 5789's fix pass.)

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## Moved to SPLICE (2026-10-29)

Moved from SPLICEFACE by `git mv`, by the SPLICE orchestrator's
sequencing call: this is lily wall 8's next stop. Wall 8 is now the long
leaf grown out of the lower stem (PR 6042, after `[ev]` PR 5990's
question 3), and the union refuses here first, a spline seam of the leaf
against the stem's torus wall.

**Witness:** `demos/tour/src/lily.rs` `wall_probes`, wall 8, pinned on
`EdgeCarrierUnsupported { operand: B, face_of: A, site: CurvedCrossing }`
with the edge a NURBS carrier of `lily_leaf_a` and the face the
`lily_stem` torus, at ε 1e-9 and 1e-6 (at 1e-12 the stem's declared
frontier stands the lily walls down first).
