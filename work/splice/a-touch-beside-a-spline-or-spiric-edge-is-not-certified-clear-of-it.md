---
id: a-touch-beside-a-spline-or-spiric-edge-is-not-certified-clear-of-it
kind: issue
title: A carrier touch whose ball a spline or spiric boundary edge's box reaches is never certified clear of that edge, so it keeps the pierce door
status: closed
opened: 2026-10-06
closed: 2026-10-25
priority: P1
cost: M
refs: [4128]
branch: splice/touch-doors
---


Found by `reach/pierce-tangent-off-face`. A carrier touch is no event
when its ball holds no point of the face
(`crates/topo/src/boolean/carrier_touch.rs`, `ball_off_face`), and each
boundary edge is shown clear of the ball by `edge_clear_of_ball`: its
certified box clear of the ball's box, or else a lower bound on its
distance from the ball's centre. The bound exists for a line, a circle
and an ellipse. A spline or a spiric edge whose box reaches the ball
gets none, so it is never clear and the touch keeps the pierce door.

A curved face trimmed by another curved face (a wall cut by a crossing
wall, a sphere by a torus) has such boundary edges, so a touch near
one refuses where the geometry is clear. A per-piece distance bound
along the edge — the same bisection `carrier_touch::clusters` runs
along a span, over the edge's own certified boxes — would decide it.
Unmeasured: no fixture in the tree poses a touch beside such an edge.

## Moved to SPLICE (2026-10-25)

From APEX. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: with the gate gone, a lofted operand's spline edges reach this touch every time.

## Closed

The finding's premise had moved: a spline or spiric edge already had
a floor, the distance to one ball holding the whole arc
(`containment::carrier_ball`). It now also has a per-piece floor.
`containment::arc_distance_floor`'s ball arm carries the arc
(`ArcPieces`), read when the ball is not clear: the carrier's point
walk (`geom::Curve3::point_walk`, the one `clear_of` and
`curve_pair::walk` read too) halves the span, and a piece's floor is
`|q − m| − ℓ`, with `ℓ` its reach at `speed_range`'s bound. This is
the one floor the touch gate (`edge_clear_of_ball`) and the cell door
(`cell_region`) share, so both read it.

Witness: `carrier_touch_rows::a_ball_beside_a_spline_or_spiric_edge_is_cleared_of_it`
(a spiric edge through the door, a spline on the floor). Through the
cell door it is reachable today: `projected_row_chart_box::the_cell_door_reads_a_spline_edge_as_its_ball`
decides ten more cells beside its spline edge (re-baselined 65 → 75).
Through the boolean it is reachable once the operand gate goes.
