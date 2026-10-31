---
id: w5-loop-free-stage-before-the-plane-nurbs-whole-reading
kind: issue
title: W5: the interior-loop guard's loop-free stage before C3's whole reading of a plane × spline section
status: closed
opened: 2026-10-29
closed: 2026-10-31
priority: P1
cost: M
pr: 6150
branch: splice/w5-loop-free-sweep
---

## What

The interior-loop guard reads a plane × spline pair through C3's whole
reading (`PlaneSection::all`, PR 6026). The guard asks one fact of that
reading: that no closed loop of the section lies inside both faces.
W5 proves that fact cheaply. For each span of the wall, take one
interval enclosure of the wall's normal and bound it away from ±n, the
plane's normal. The height above the plane then has no critical point,
so the section has no closed loop.

Ev ratified W5 as a stage run before the whole reading on PR 5990
(2026-10-29). `[one-way.readings]` is re-phrased there: stages always
run together may jointly prove one rule. The probe on
`splice/probe-w5-vs-whole-reading` @ 2549fb00 measured the case:
- W5 costs 0.5–8 ms a wall. The whole reading of a plane cutting
  lily's blade costs 2–24 s a pair across the three ε.
- The whole reading refuses some pairs W5 clears, always for
  incapacity, never because it found a loop: an exact graze, a
  near-graze 100ε out, a wall 1 km from the origin, and a side leaving
  the plane at 1e-6 or less.

## What closing it takes

- W5's plane form runs as the stage before the whole reading, in
  `geom_brep::pair_section`'s `(Plane, Nurbs)` arm.
- W5 reuses the SSI's derivative boxes (`NurbsBoxes::deriv_box_closed`)
  and is decided in certification arithmetic.
- The `section_cert` doc names the stage.
- Rows: the probe's pairs. Those W5 clears clear, including the graze,
  which the whole reading refuses alone. Those it declines fall to the
  whole reading. Each row must red when W5 is removed or its bound is
  loosened.
- The pair form (det(N_F, N_G, P) one-signed) is not built: no witness
  needs it, and it clears no pair sharing a rim.

Order: after PR 6026, which builds the arm this stage stands in front
of.


Re-framed by `[ev]` PR 6121 (C5): W5 is the plane instance of the
pose classification's regularity sweep, and the guard reads that
verdict; it is not a predicate in `section_cert`.

2026-10-31: Review tier: single FULL (a bound leaning the wrong way
would clear a real loop).

## Closed

The plane × spline arm (`geom_brep::pair_section::plane_spline`)
classifies the pair in C5's stages before C3's whole reading:

1. the hull (`PlaneSplinePose::apart`, W0 moved out of `section_cert`),
   at every scalar, with or without a march door;
2. the boundary pass, C3's, inside the whole reading, unchanged;
3. the regularity sweep (`plane_spline::sweep`), through the march
   door: span cells refined to a floor, each regular (`n·S_u` or
   `n·S_v` one-signed on `deriv_box_closed`) or excluded (`φ` past the
   band's escalation), every cell that is not regular joined to the
   rectangle's boundary through cells excluded on its side, and no
   crease inside the rectangle. Its loop-free verdict is the section,
   `Components::Reaching`, with the sides the section can end on.

The guard (`section_cert::spline_plane`) reads the classification: a
`Reaching` verdict clears by W1′ where every side it names bounds the
wall, and is otherwise read whole (`PairSection::read_whole`); a
declined sweep is the whole reading, as before. The completion of
stage 3 (Krawczyk isolation of a critical point at a declined floor
cell, its Hessian's touch or node) plugs in at `Declined::Floor` and is
SECTARMS's, in `work/sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm.md`.
