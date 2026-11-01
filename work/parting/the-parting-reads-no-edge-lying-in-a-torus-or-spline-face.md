---
id: the-parting-reads-no-edge-lying-in-a-torus-or-spline-face
kind: issue
title: The Boolean's parting reads no edge lying in a torus or spline face, so a rider there keeps one edge
status: dispatched
opened: 2026-10-26
priority: P1
cost: M
branch: parting/closed-and-torus-riders
---



Filed by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`
(PR 5793), a residue its rows disclose. P1: D1-wrong topology shipped
silently, the class those rows close for planes, cylinders, cones and
spheres. Unmeasured: no fixture of a torus or spline face holding a
rider is built.

## What

`topo::boolean::slit::carrier_on_surface` reads a line on a plane,
cylinder or cone and a conic on a plane, sphere, cylinder or cone;
every other pair answers `Ok(false)` by its last arm, so an edge lying
in a torus face (a circle of the torus's own families, or a Villarceau
circle) or in a spline face is never read as lying in it, and
`slit::walls_inside`'s tangent arm (`bend_across`) reads no bend on
either. Where both the edge's faces go into such a face's material,
the Boolean keeps one edge. The edge-carrier half (a spline or spiric
edge in a plane face) is SPLICE's
`work/spliceseam/a-spline-rider-reads-as-lying-in-no-plane-face.md`.

## Fix

Read a conic on a torus where it is one of the torus's circles (a
meridian, a parallel, a Villarceau circle), at the five points
`carrier_on_surface` reads a quadric at, which a circle meets a torus
at no more than eight of, so take nine; then the arc refuses `Curved`
as on a quadric. A spline face waits on a point-on-chart reading the
band can decide; until then refuse typed where an edge's ends lie on
the face and its midpoint reads on its chart.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)
