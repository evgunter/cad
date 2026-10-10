---
id: tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball
kind: issue
title: moved_walls_cross reads a planar face's spline or spiric edge as its whole carrier ball, so it may refuse a pair whose walls clear
status: closed
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
branch: shell/tilted-read-gaps
pr: 4467
closed: 2026-10-10
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

## Closed

2026-10-10, PR 4467. A spiric or spline edge is cut on its own carrier
(`ArcPiece`, `MovedWall::cut`), breadth first, as parameter windows.
Each piece's ball is centred on its window's midpoint point, with
radius a speed bound times the half-width: the spiric's
`carrier_ball`, or `spline_speed` for a spline, a closed-form bound on
`|C′|` from the derivative coefficients of the homogeneous net and the
weights. A piece crosses at its chord's end sides. A piece whose ball
reaches the line halves until its ball is within the band, within a
budget of 1024 halvings per edge per line. Leftover pieces add their
ball's cut, so the set read holds the true one. While the budget lasts
it exceeds the truth by a few bands; past it, by as much as the
leftover balls reach (a weight-40 parabola cut near its apex reads
`[0, 2.124]` against `[0, 1.951]`), which costs refusals, never a miss. A sample point or a ball the band cannot place counts with the
reaching side, so the read only ever grows. The ball's offset from the
line is taken as a vector's norm: the old `|w|² − along²` cancelled to
about `1e-8` noise.
A window runs the way its half-edge runs (reversed on a minus
half-edge). The review of PR 4467 caught the first version seeding
carrier-ordered windows with the face's own vertex order, which left
holes in a minus-held arc's cut and read a crossing pair clear. A spline
piece is read in the smaller of its speed ball and its control net's
ball.

The item's hypothesis held, and the case was reachable through `shell`.
The bowl sector (`common::shell_operands::bowl_sector`: an annular
meridian with a torus floor, revolved) refused `OffsetsCross` on the
merge base at every angle and wall probed, with "overlaps" of `0.3` to
`2.4` m, because its cavity's end caps are bounded by spirics whose one
ball reached the line. The walls clear. On this branch the read clears
it, and the sector stops at the props door, where a spiric-bounded cap
has no volume yet (`work/flux/spiric-bounded-face-area-is-unimplemented.md`,
evidence added there). Rows: `a_bowl_sectors_spiric_bounded_end_walls_clear`
(`verbs_shell`) and `the_tilted_cut_reads_a_spline_on_its_carrier`
(unit: a parabola's cut within a few bands of `[0, 0.75]`, where its
ball covered `[−0.32, 2.32]`, in both orientations at three lines),
`the_spline_speed_bound_covers_a_strongly_rational_arc` (red without
the bound's weight term), `speed_fuzz` (the bound over random rational
splines), `a_rational_spline_piece_is_read_in_its_control_ball`,
`the_tilted_cut_reads_a_spiric_on_its_carrier` (both orientations) and
`a_reversed_spline_wall_crossing_a_rectangle_refuses` (the review's
end-to-end case, overlap `0.06`).
