---
id: root-slack-meters-roots-outside-the-arc
kind: issue
title: The subdivision's root-slack meter charges every root round the turn, so a root off the arc that it cannot place refuses an arc it does not touch
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [degree-2-subdivision-doors-carry-no-root-slack-meter, an-edge-crossing-a-cone-face-has-no-root-lane]
---


Found by the lane that gave a cone face its crossing lane
(`an-edge-crossing-a-cone-face-has-no-root-lane`).

## What

`circle_roots::certified_subdivision` locates every root round the
whole turn (reported within `π` of the arc's midpoint) and, given a
`RootSlack` meter, refuses the whole answer (`Uncertain`) when any one
root's slack is not definitely inside the band — whether or not the
root can reach the arc `[t0, t1]` the caller asks about. The caller
(`reduce::wall_crossing`) reads only roots inside the span or at its
ends, so a graze on the far side of the turn refuses an arc it does not
touch. The meter holds the root to the band; what the span decision
needs is weaker — a root outside the arc must be placed no nearer the
arc than the band.

## Measured, on the line × cone lane (fixed there)

The same shape refused a real pose at ε 1e-12 in that lane's first
cut: a rod's seam line crossing a narrowing frustum's wall, whose line
passes 0.1 m from the apex with two roots at `t ≈ 1.91` and `2.13`,
both outside the edge's span `[0, 0.5]`, their slack 2.5e-12 m. The
line lane now meters each root's slack less its distance outside the
span (`bool_line_cone_root_slack`). Unmeasured on the conic doors: the
ellipse × torus door (`bool_ellipse_torus_root_slack`) and the conic ×
quadric door's cone arm (`bool_conic_cone_root_slack`) meter every
root.
