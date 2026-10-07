---
id: the-corner-slice-arm-is-silent-on-cones-nurbs-faces-and-first-order-ties
kind: issue
title: Check 9's corner arm does not read a cone or NURBS/Approx face, a NURBS side, or two sides leaving along one tangent
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-corner-is-a-slice-of-its-face-tier-3-check, tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex]
---


## What

Check 9's corner arm (`validate.rs`, `pinch_corner_errors` and
`wedge_holds`) refuses a corner whose wedge holds another edge of its
loop at a point the loop passes twice (`PinchCornerCrossed`). It reads
the wedge in the tangent plane at the point, about the outward normal
`face_normal::face_outward_normal_at` gives. It is silent on:

- a face that door gives no normal for: a cone (its apex has none),
  and a NURBS or `Approx` face;
- a side on a NURBS edge, for which `side` reads no departure;
- two sides leaving along one tangent, and a cusp corner whose own
  sides do. First order cannot order them. For two lines that is a
  shared segment, which is the census's `EdgeEdgeOverlap`. For two
  tangent arcs it would need the curvature term (the C12.2
  second-order descent `splitting::neighborhood::chord` already
  carries).

No producer is known to reach any of these with a crossing. The JOIN
batteries the arm was measured on (PR body of
`join/tier3-pinch-checks`) build no cone or NURBS face through a pinch.

## The shape to give

A cone face off its apex can take the implicit gradient as
`sector_face::resolve` does. The tangent tie wants the second-order
term. A NURBS side wants its departure off the curve's first
derivative at the end.
