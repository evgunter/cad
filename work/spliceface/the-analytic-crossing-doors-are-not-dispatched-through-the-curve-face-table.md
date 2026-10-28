---
id: the-analytic-crossing-doors-are-not-dispatched-through-the-curve-face-table
kind: issue
title: The sweep's analytic crossing doors are routed by curve_face::route on paper only: the sweep reaches them through its own surface and carrier matches
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [a-span-is-read-against-a-faces-boundary-edges-first, nurbs-edge-crossing-rung-is-the-ring-composite]
---



Filed by SPLICE's boundary-first lane (PR 5768), from its review.

## What

`geom_brep::curve_face::route` names a door for every
`(CurveKind, SurfaceKind)` pair. Since PR 5768 the sweep dispatches the
spline-face door through it (`topo` `reduce::curved_face_arm`: a pair
the table routes to `Door::SplineFace` goes to `spline_face`). The
analytic doors the table names (`PlaneCrossing`, `LineRoots`,
`ConicQuadric`, `ConicTorus`) and the unbuilt `SplineComposite` and
`Spiric` doors are still reached through the sweep's own matches:
the planar lane on `face_plane` and `splitting::plane_crossing_lane`,
the curved arm on the surface variant and the carrier kind. So for
those pairs the table documents a dispatch it does not perform, and a
door moved in one place does not move in the other.

## Why it did not ride PR 5768

The curved arm runs several rungs before any root door, in an order
that is part of their soundness: the carrier-identity rung, the
one-sided cover, the equal-cylinder factored crossing, then the roots
by carrier kind. The planar lane is a separate branch of the sweep's
face loop. Binding each door to `route` means restructuring both
around the table, which is the "become arms when next touched" migration
the design names, and is larger than the boundary pass.

## Fix

Dispatch the sweep's crossing reading on `route(curve, surface).door`,
each door one function, and let `route`'s `implemented` be what the
dispatch reads; the converse arm
(`nurbs-edge-crossing-rung-is-the-ring-composite`) is a natural first
consumer, since it replaces the planar lane's `Unlaned` refusal with the
`SplineComposite` door.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
