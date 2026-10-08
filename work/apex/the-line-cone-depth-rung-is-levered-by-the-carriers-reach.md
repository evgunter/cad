---
id: the-line-cone-depth-rung-is-levered-by-the-carriers-reach
kind: issue
title: The line × cone depth rung divides by the carrier's reach R, so near the apex a clean transversal pair refuses inside δ ≈ √(2εR)
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [4135]
---


Left by the last fix pass of PR 4135 (the cone root lane), from its
second review (r2 m-1).

## What

`reduce::line_cone_roots` decides `bool_line_cone_depth` on
`disc/(|A|·R)` (`crates/topo/src/boolean/reduce.rs:3608`), with
`R = max(|q(t0)|, |q(t1)|, |q(t*)|)` the line's reach from the apex
(`reduce.rs:3605`). The bound the rung rests on,
`|res(t*)| ≥ |Q(t*)|/|q(t*)|`, holds pointwise, so dividing by the
largest of three reaches only loosens it. Near the apex `Q(t*) ~ δ²`
(`δ` the line's distance from the apex), so a clean transversal pair
refuses inside `δ ≈ √(2εR)`.

Measured by r2 on a steep line, `α = π/4`, scale 1: the nearest
answered `δ` is 2.5e-4 m at ε 1e-9 and 7.9e-3 m at ε 1e-6. At ε 1e-6,
21 of 240 random end-to-end sweeps escalate `bool_line_cone_depth`.

## The fix owed

Read the depth through `|q(t*)|` alone (the pointwise bound), or a
reach that is a floor at `t*`, and show the refusal zone shrink to the
apex rung's own band with no new wrong answer against an oracle (the
row `line_cone_rows::roots_beside_the_apex_are_placed_along_the_edge`
is the place-along-the-edge oracle to extend).
