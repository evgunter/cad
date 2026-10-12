---
id: a-touch-beside-a-spline-or-spiric-edge-is-not-certified-clear-of-it
kind: issue
title: A carrier touch whose ball a spline or spiric boundary edge's box reaches is never certified clear of that edge, so it keeps the pierce door
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [4128]
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
