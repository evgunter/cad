---
id: a-line-tangent-to-a-cone-off-its-span-keeps-the-frontier
kind: issue
title: A line edge tangent to a cone's carrier at a point outside the edge's span refuses at the line x cone lane's tangency rung, though the edge is clear of the cone
status: open
opened: 2026-10-09
priority: P3
cost: E
refs: [VERBS-CONE]
---

Found by VERBS-CONE U7 (`germ/cone-roster-flip`) while measuring the
spec's P8 fixture through the public doors.

## Measured

The quarter cone (the triangle `(0,0) (1,0) (0,1)` revolved `π/2`
about `y`) against the brick `[0.5, 0.8] × [0.5, 0.8] × [−0.3, −0.1]`,
which is clear of the cone: every op refuses
`CurvedPierceUnsupported` on the brick's edge along `z` at
`(x, y) = (0.5, 0.5)`. That edge's line is tangent to the double cone
at `z = 0` (`ρ² = 0.25 + z²` against the parallel `ρ = 0.5`), 0.1 m
outside the edge's span `z ∈ [−0.3, −0.1]`. Moving the brick to
`[0.55, 0.8]²` builds every op at its closed form
(`sweep/tests/cone_operand_rows.rs`, P8).

## The cause, by reading

`reduce::line_cone_roots` decides `bool_line_cone_depth` on the
quadratic's vertex `t*` and returns `CircleRoots::Uncertain` on Zero
whatever `t*`'s place relative to `[t0, t1]`. Where `t*` is definitely
outside the span (by more than the band, at the line's speed), `G` is
monotone over the span and the endpoint signs decide it: no root inside
when they agree. The other rungs already read the span
(`bool_line_cone_root_slack`'s `outside`).

A conservative refusal, never a wrong body: an axis-aligned brick
whose corner line sits on a parallel's tangent line is a natural pose,
so it costs answers.

