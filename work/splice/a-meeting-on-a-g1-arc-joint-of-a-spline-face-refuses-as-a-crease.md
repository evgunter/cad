---
id: a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease
kind: issue
title: A meeting on the joint of a spline face's rational arcs refuses as a crease: klein wall 3 stops there
status: parked
opened: 2026-10-27
priority: P2
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-closed-charts-seam-continuity-has-no-home]
blocked_on: [the-crease-arms-retire-once-no-carrier-carries-a-c0-knot]
---

Filed by the SPLICE lane that built the analytic span × spline face
arm (`geom_brep::curve_face::spline_face`).

## What

Klein wall 3 (`demos/tour/src/klein.rs`, `wall3_pinned`) now refuses
`SplineCrossingUnsettled { operand: A, edge: 4v1, face: 3v3,
face_of: B, why: Crease }`. Measured: the loop's wall 3v3 is degree 2
in `u` on knots `[0, 0, 0, 0.5, 0.5, 1, 1, 1]`, two rational arcs of
the loft's circle section, so `u = 0.5` is a knot run of multiplicity
the degree and `KnotStructure::crease_runs` names it a crease. The
bulb's edge 4v1 lies in the plane `y = 0`, which holds that iso-curve,
and meets the wall on it (the floor box it stopped on read the edge's
point 1.4e-5 m from the chart's `u = 0.5` point at the same `v`).

The arm refuses rightly by its contract (C5, the crease class): it
reads no Jacobian across such a knot, and cuts the chart there. But a
rational circle's arc joint is tangent continuous, so the surface has a
tangent plane there and the meeting is an ordinary transverse one.

## The likely fix

A tangent-continuity reading beside `crease_runs` for an interior run
of multiplicity the degree (the two one-sided derivatives parallel,
decided), as `a-closed-charts-seam-continuity-has-no-home` asks for a
closed chart's seam. Where it holds, the arm can isolate a meeting on
the joint by a Krawczyk test whose Jacobian takes the two sides' hull,
the mean-value inclusion holding for a function C¹ on each side and
continuous across.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

Moved from SPLICEFACE to SPLICE on 2026-11-02, by `git mv`, when SPLICEFACE merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Parked on the crease arms' retirement (SPLICE, 2026-11-03)

`tesssize/the-crease-arms-retire-once-no-carrier-carries-a-c0-knot` names this row as one that becomes unreachable under DESIGN [topology] (Ev, `[ev]` PR 5913: every carrier C¹ strictly inside its cell; no reader carries a crease arm), and closes it. Parked on it rather than built, as `a-creased-spline-gets-no-second-order-clearing` was.

## Klein wall 3 no longer reaches the crease (FRAME, 2026-11-03)

From `frame/turned-onto`. The klein loop's start frame rolled a quarter turn (it is now the world frame, the least turn onto the `+z` start tangent), so its circle sections start in the plane `y = 0` and the line this row measured is now the loop's NURBS seam edge, not the arc joint inside wall 3v3. Wall 3 now stops on `a-rim-arc-lying-on-a-cylinder-refuses-as-a-curved-pierce` instead, and this row has no live witness in the tour.
