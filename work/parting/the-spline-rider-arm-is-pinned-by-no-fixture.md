---
id: the-spline-rider-arm-is-pinned-by-no-fixture
kind: issue
title: The parting's spline-face arm (SplineFace) is reached by no fixture: the roster refuses a spline face's pair first
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Filed by the PARTING lane on `parting/closed-and-torus-riders`, which
added the arm.

## What

`topo::boolean::slit::holding` refuses `EdgeInFace` (`SplineFace`)
where an edge's ends lie on a spline face, its midpoint reads inside
it and both its faces go into the face's material (`walls_inside`,
which reads a spline floor's normal at its placement's in-trim foot,
`slit::spline_floor`). No fixture reaches it: a flat or wavy loft's
spline side less a revolved ring, or less a turned brick whose edge
lies in that side, refuses `CurvedPairUnsupported` at the roster
(`site: RevertRoster`, Nurbs against Cone or Plane) before the
parting runs.

## Fix

Pin the arm with the loft less the turned brick (its ridge lying in
the loft's `x = 1` side) once the roster admits a plane × spline pair
there; until then the arm is defensive.
