---
id: carrier-walk-has-no-crossing-row-for-spline-edges
kind: issue
title: The in-plane carrier walk has no crossing row for a spline edge, so a planar face bounded by one refuses containment inside its control hull's ball
status: closed
opened: 2026-10-03
priority: P1
cost: M
branch: splice/carrier-walk-spline
pr: 5719
closed: 2026-10-25
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

## Moved to SPLICE (2026-10-25)

From CLEAVE (taken on request in chat; CLEAVE would not start a P3). Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: a link the spline operand's path runs through.

## Closed

`splitting::spline_arc::SplineArc` is the spline row, beside the
spiric's: `carrier_loop` holds a `Nurbs` edge as `LoopEdge::Spline`,
and `LoopEdge::Unrowed` and the walk's ball pass are deleted. Both
pieced edges read through one driver, `splitting::pieced_arc::PiecedArc`
(the boundary reading `contact` and the crossing count `crossings`,
moved out of `SpiricArc`); each carrier states its pieces, a ball per
piece and its own crossing test. A spline piece is a window of one knot
span, its Bézier net the scalar's own restriction of the span's
coefficients (`geom_brep::pcurve_cache::projected::piece_controls`, over
`SpanLocate::restrict_window`); it is settled against a ray by
its control points (all on one side of the ray line, or behind the
point) or by their offsets from the ray line being strictly monotone
with both ends signed. `Uncrossable { carrier: Spline }` remains only
where a spline's boundary reading runs out of depth or pieces.

Witnessed by `splitting::containment::spline_cap_rows::a_spline_cap_is_read_inside_its_control_ball`
(the walk and `boolean::contfp` on a cap bounded by a cubic and its
chord; every row refused `Uncrossable` on main), and
`splitting::spline_arc::tests::{the_row_reads_the_arcs_region,the_count_is_exact}_at_{f64,interval}`,
and searched by `splitting::spline_arc::fuzz::random_splines_at_{f64,interval}`.

Reachability stays unmeasured: no body today builds a planar face
bounded by a spline edge through a public door (the operand edge gate
and the split's `CurvedEdgeUnsupported` both refuse first).
