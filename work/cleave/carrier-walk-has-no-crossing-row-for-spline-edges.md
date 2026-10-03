---
id: carrier-walk-has-no-crossing-row-for-spline-edges
kind: issue
title: The in-plane carrier walk has no crossing row for a spline edge, so a planar face bounded by one refuses containment inside its control hull's ball
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Split off `carrier-walk-has-no-crossing-row-for-spiric-or-spline-edges`
when its spiric half landed (`splitting::spiric_arc`). That row's
spline half remains.

`splitting::containment::carrier_loop` still holds a `Nurbs` edge as
`LoopEdge::Unrowed { center, reach }`, the ball about its control
points' bounding-box centre (`carrier_ball`). `carrier_walk` answers
only along a scheduled ray that definitely misses that ball (its
`point_in_arc_loop_reach` row), and refuses
`PointInLoopError::Uncrossable { carrier: Spline }` where none does —
every point inside the ball included. `LoopEdge::contact` reads such an
edge `EdgeContact::Unread`, so `boolean::contain`'s boundary pre-pass
cannot place a point on it either.

**What a fix needs**: a spline row beside the spiric's, with the same
two halves `SpiricArc` has. The boundary reading: subdivide the curve
(knot insertion) until each piece's control hull is definitely clear of
the point, or a point of the curve is within the band of it. The
crossing count: per piece, the hull definitely missing the ray, or the
piece's offset from the ray line definitely monotone (its control
polygon's offsets monotone, by the variation-diminishing property) with
both ends definitely signed. A piece unsettled at a depth abandons the
ray, as `SpiricArc::crossings` does.

**Reachability is unmeasured.** No fixture found builds a planar face
bounded by a `Nurbs` edge: loft's NURBS edges are its seam struts, on
the walls. A plane × NURBS-surface section through a boolean would make
one; nobody has run that probe.
