---
id: a-cone-and-a-one-band-ball-meet-in-a-loop-inside-both-faces-and-refuse-its-cut-in
kind: issue
title: A cone and a one-band ball off its axis meet in a loop inside both faces, and the cone's cut-in is not built (FallbackExtentUnsupported) where main's two lunes build
status: open
opened: 2026-11-26
priority: P2
cost: M
refs: [a-ball-against-a-donut-turned-a-quarter-refuses-its-cut-in-at-1e-6, close-cuts-is-not-total-on-crossings-path-cut-ins]
---

## What

`sweep::all a_cone_with_a_ball_or_a_cone_in_every_order::a_cone_and_a_ball_off_its_axis_refuse_their_loop_in_every_order`
is main's row (`e27a7a9ea8`, `cfac4e0abd`), named
`a_cone_and_a_ball_off_its_axis_build_in_every_order` there. On main `5e04ac6561` it
builds every op in both orders. On PR 7414's head, after main's merge
(`f2731c9f39` and later), every run refuses, typed, at ε 1e-9, 1e-6 and 1e-12:
`FallbackExtentUnsupported { operand, face: FaceKey(2v1) }`, with the
operand the cone in either order. The message reads: "a curved face
meets a face of the other solid in a closed loop interior to both
faces, which no edge event and no vertex probe can see; the loop is
certified, and cutting it in is not built". The row is not on the
one-band stack before the merge (`a8a9d3a9b6`).

The pose is the ball of radius 0.4 about `(0.9, 0.6, 0)`, poled along
`z`, against the cone `K` (base radius 1 at `y = 0`, apex `(0, 2, 0)`).
They meet in one loop on the cone's side. On main the ball's two lune
seams are the meridians through `±u_ref`. On head the ball has one wrap
edge, and no edge event reads the loop. So the loop reaches the
no-crossings path as interior to both faces, and the cut-in it asks
for on the cone's face is not made. The section pass's refusal stands:
typed and loud, with no body shipped wrong.

## Not yet known

Whether the cut-in is refused because the cut face is a cone (the
no-crossings path's cut-in reads a sphere's meridian), or at one of
the cut-in's readings, is not traced. Nor is whether turning the ball
so its wrap edge crosses the loop restores main's outcome.

## Pin

The row pins each run's outcome. For this pose it pins the refusal
above in all six runs (`BALL_PINNED`, citing this row); the two-cone
row stays `built`.
