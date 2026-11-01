---
id: a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side
kind: issue
title: A whole ball intersected or united with a tilted brick that cuts a cap off it refuses UNCLASSIFIED_SIDE at a union vertex
status: parked
opened: 2026-10-13
priority: P2
cost: M
blocked_on: [closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut]
---


## What

Found by the sweep that closed
`a-sphere-cut-in-cap-fails-its-names-missing-upstream` (branch
`emit/naming-refusals`, 2026-10-13). Measured on origin/main's sources
too (17915e5cf6), so the defect predates that change.

Take the unit ball (a bulge-1 semicircle revolved a full turn about y)
and the brick of
`crates/editor-core/tests/emit_sphere_cut_in_cap.rs`'s `brick_toward`:
0.4 × 0.4 × 0.3, near face 0.985 from the origin toward latitude 68°,
azimuth 50°. The brick cuts a cap of height 0.015 off the ball.
`Intersect` and `Union` of the two, ball first, refuse:

`Naming(Emission("a member edge in a union vertex's name has a side no pairwise judgement classified against the members the name cites"))`

That is `emit_union::UNCLASSIFIED_SIDE`. `Subtract` (ball minus
brick) names. Unmeasured: the brick-first order, which vertex refuses
and through which member edge, and whether the kernel re-charted the
ball (`apply_recuts`) or cut it in (`apply_cut_ins`). The lens rows of
the closed row build the same cap on the lens's trimmed face and name
it up to its conventional vertex. So look first at what a whole
sphere's seam does at the circle's vertex.

## Measured (branch `emit/ball-brick`, 2026-10-14, on 490f61e732)

Pinned in `crates/editor-core/tests/emit_ball_cut_by_brick.rs`
(`a_ball_cut_by_a_brick_refuses_at_its_turned_pole_where_a_member_vertex_is_read`),
every op in both orders:

- Intersect and union refuse `UNCLASSIFIED_SIDE` brick first too.
  Subtract names in both orders.
- The kernel takes `apply_recuts`, not `apply_cut_ins`: the extent
  scan returns one re-chart (the ball's face) and no cut-in, in every
  op and order. The ball is one shell, so `apply_recuts` keeps no other
  shell and the turned shell is the rebuild itself. No
  `graft_solids_disjoint` call is made, so
  `work/wire/a-sphere-recut-grafts-its-rotated-shell-under-keys-no-record-carries.md`
  is not reached here and is not this row's cause.
- The re-chart turns the ball about its centre until its poles lie on
  the brick's axis, ±(0.2408, 0.9272, 0.2870). Each pole and meridian
  keeps its key but is no longer where the ball has it: the ball's
  poles are at (0, ±1, 0) and its two meridians lie in z = 0.
- The refusing vertex is a turned pole. For the intersect it is the
  one at +axis, the cap's centre. For the union it is the one at
  −axis. In `Flush::meeting` its lines are the ball's `Meridian(Seam)`
  and `Meridian(Pi)`. `group_member_edges` reads the turned meridian
  pieces as pieces of those edges by key, as a `From(m, e)` row. Both
  lines belong to the ball, so each is read against no other member
  (`others = {}`). The judgement classified the meridians only where
  they cross the brick's near face, so `sense_against` finds no class
  at the pole and refuses. The refusal comes before that, at
  `Flush::least_vertex`: it reads a member vertex as one at the member
  vertex's point in the member's own body, finds none at a turned
  pole, and so sends the vertex to `meeting` as a junction of member
  edges.
- The subtract names the same entities by key (`emit_topo`). Ball minus
  brick publishes `From(ball, Pole(step of the LineTo leg))` (the ball's
  vertex at (0, 1, 0)) at (−0.241, −0.927, −0.287). The turned
  meridians' pieces are published as pieces of the ball's seam and π
  meridian.

## Re-measured (branch `emit/union-curves`, 2026-10-14)

The union now counts curved member edges on one carrier as one line
at a vertex (`emit_topo::least_per_line` through `EdgeCarrier`), as
it counts collinear straight ones. The ball's `Meridian(Seam)` and
`Meridian(Pi)` lie on one great circle, so at the turned pole they are
one line, the least, and no seam: `Flush::meeting` takes its
one-line arm, finds no face there that bounds none of the line's sides,
and refuses `one member edge meets no face at a union vertex no seam
reaches` before any sense is read. All four intersect and union cases
refuse so; `emit_ball_cut_by_brick.rs` pins the new message. The
cause below and the question are unchanged: the vertex still reaches
`meeting` only because the turned pole is no member vertex in the
ball's own body.

## Cause

The re-chart moves a member's seam and poles while keeping their keys.
The union reads a member's edges by key but its vertices by position in
the member's own body. At a turned pole those two readings disagree,
and the vertex falls through to a junction rule that has no other
member to read a sense against. So no judgement could classify this
side, and the refusal is right not to default it.

## The question (design)

The README says what a union does with a member's entities where they
lie in the member's own body ("read off the finished body and the
members' own bodies"; "a vertex at a member vertex is that vertex"). It
does not say what a result entity is when the kernel has turned an
operand so that it descends from a member entity by key but lies off
it. Viable answers:

1. **The role moves with the chart.** A turned pole is the member's
   pole and a turned meridian piece is a piece of the member's
   meridian, wherever the re-chart put them. The subtract already names
   them this way. The union would read member vertices through the
   pairwise judgement's key lineage, or through the re-chart's rigid
   motion exported on `BooleanNaming`. Cost: a name such as the top
   pole denotes a point the rotation's sign chose, here near the
   bottom, and it moves whenever the brick moves the axis.
2. **A re-chart's moved seam and poles are chart artifacts**, no entity
   of the operand, as a chart cut is no edge of its operand
   (`emit_topo`'s "a chart cut survives in the result"). Every op
   refuses them typed, and the subtract stops publishing a moved pole
   under the ball's name. They then wait for (3).
3. **Retire the re-chart.** ORBIT's
   `closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`
   (P1, H) already plans "the cut for every escape, `apply_recuts` …
   retired". The ball's seam and poles would then stay where the ball
   has them, and this document would meet the cut-in rows the lens
   reaches (`emit_sphere_cut_in_cap.rs`: `ConventionalVertex` for the
   cap, the surviving cut for the union).

Recommendation: (3), with (2)'s typed refusal as the interim spelling
if the re-chart outlives this row. (1) would build union machinery on a
mechanism that ORBIT has already scheduled for deletion. It would also
make a name's position depend on a rotation sign that no recorded
verdict decides.

## Parked

Parked on ORBIT's
`closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`,
which retires the re-chart this row's question exists for (answer 3).
The question goes to a designer pair only if that row lands with the
re-chart kept. Until then intersect and union refuse, and
`emit_ball_cut_by_brick.rs` pins the subtract's turned pole, so the
test flips when the re-chart goes.

## Option 2 is being built (WIRE orchestrator, 2026-10-27)

PR 5829 builds option 2. A re-charted ball's turned seam edges and poles refuse typed as `NamingError::UnsupportedTurnedCell`: row 2, `NOT_YET_ENDING`, with ORBIT's retire-the-re-chart row as the way through. `emit_ball_cut_by_brick` now pins that refusal in all six cases. The subtract no longer publishes the turned pole. The sentence above, that the test "pins the subtract's turned pole", is superseded once 5829 lands.
