---
id: tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball
kind: issue
title: moved_walls_cross reads a planar face's spline or spiric edge as its whole carrier ball, so it may refuse a pair whose walls clear
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
---


Disclosed by PR 4311, which added the tilted read
(`moved_walls_cross`, `MovedWall::cut`, `crates/topo/src/shell.rs`).

The read cuts each moved planar face with the line its plane shares
with another. Line edges are cut at their vertices' sides, circle and
ellipse edges at their conic roots (`EdgeArc::Conic`). A planar face
bounded by a spiric or NURBS edge is read as that edge's chord plus
the chord of the line through the ball holding the arc
(`EdgeArc::Ball`, `carrier_ball`). That is a superset of the face, so
it never misses a crossing, but it may refuse a pair whose walls
clear: before the conic cut existed, the same ball reading of a
circle arc refused `shell_curved_mouth::a_sphere_window_refuses_at_check_7`'s
`120°` dome sector, whose meridian arc's ball (centred on the axis,
radius 1) covered the line at every wall.

No spline- or spiric-bounded planar face reaches the read in the
tree's rows today (the PR's differential over every shell row, the
editor-core corpus and `demos/tour` fired on none). Close it by
cutting such an edge on its own carrier: the side of the line along
the curve is a scalar function whose sign changes are the crossings
(a spline's control polygon brackets them; a spiric's quartic has a
closed form), so a certified root bracket per sign change replaces
the ball.
