---
id: a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease
kind: issue
title: A meeting on the joint of a spline face's rational arcs refuses as a crease: klein wall 3 stops there
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-closed-charts-seam-continuity-has-no-home]
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
