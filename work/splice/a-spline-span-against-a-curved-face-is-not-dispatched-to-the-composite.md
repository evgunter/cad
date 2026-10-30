---
id: a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite
kind: issue
title: The curved crossing lane refuses a spline span (EdgeCarrierUnsupported CurvedCrossing) though the ring-composite arm reads cylinders, cones, spheres and tori
status: closed
opened: 2026-10-26
priority: P1
cost: M
closed: 2026-10-29
branch: splice/spline-span-curved-crossing
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

## Closed (2026-10-29)

`curve_face::route` builds `(Nurbs, Cylinder | Cone | Sphere | Torus)`,
and the curved lane reads a spline span through `reduce.rs`'s
`spline_wall_event`. The composite counts every crossing strictly
inside the span. Each crossing is told off a cone's far nappe and placed
against the trim, and the first one the face holds pierces. With no
pierce, the ends are read through `vertex_on_curved_face`. A count the
arm cannot settle is localized by `carrier_touch::off_face`, which
passes a touch that lies off the face. Otherwise the arm refuses
`SplineCrossingUnsettled`, a span lying on the carrier among them. Under
a one-sided cover, a touch with no end on the carrier is deferred to
the span's fragments. The rows are in
`crates/topo/src/boolean/curved_lane_spline_rows.rs`.

The three owed items:
- **The frame's rounding.** `composite_rows` now forms the chart frame
  in certification arithmetic. The axis is normalized, and the
  reference direction is made orthogonal to it and normalized. Row:
  `a_frame_off_orthonormal_reads_the_surface_it_names`.
- **An unsettled count.** Both the planar lane and the curved lane route
  an unsettled count through `carrier_touch::off_face`. `off_face` now
  reads a plane: its distance and foot are closed-form, its reach is
  unbounded, and the foot is placed by the planar door. Rows:
  `an_arc_touching_a_plane_outside_the_face_is_no_event` and
  `a_spline_touching_a_wall_below_the_face_is_no_event`.
- **The metering divisor's region: no change, because the premise does
  not hold.** `Meters::divisor` is not a bound on the gradient along a
  path to the surface. It is a factor of the form at each point:
  `|f(p)| ≤ d(p) · divisor(p)` at `p` itself. On the cylinder and the
  sphere this is exact. On the cone, `|f|` is the product of the
  distances from the two generator lines of the meridian plane, and the
  smaller of them is the distance. On the torus,
  `|f| = |m − r| (m + r) |M² − r²|`. Each point of the span lies in the
  net's hull, so the hull is the right region, and a path to the foot
  that leaves the hull charges nothing. Row:
  `the_divisor_is_a_pointwise_factor_of_the_form`, which pins the
  pointwise factor against the closed-form distances.

The composite also no longer refuses a divisor whose lower end reaches
zero, for a net whose hull holds a cone's apex: the meter divides by
the divisor's upper end alone.

Lily wall 8 now refuses one pass later, at 1e-9 and 1e-6: a stem vertex
pierces a leaf NURBS wall, and the vertex-on-face pass reads no outward
normal for a spline face
(`a-vertex-piercing-a-spline-face-reads-no-outward-normal`).
