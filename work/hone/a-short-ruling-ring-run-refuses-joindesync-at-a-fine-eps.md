---
id: a-short-ruling-ring-run-refuses-joindesync-at-a-fine-eps
kind: issue
title: a ring run along a short ruling refuses JoinDesync 'ring-run winding is degenerate' at ε 1e-12, where at 1e-9 it escalates
status: open
opened: 2026-10-16
priority: P2
---

Filed by TANG (U2p review, PR 5203), split out of
`work/pcert/a-short-ruling-on-a-cylinder-chart-mints-its-pcurve-at-the-seams-u.md`.

## What

`crates/topo/src/boolean/join.rs:4572` (`ring_winding_order`) reads a
ring run's decided winding. A margin in band escalates
`SelfCheck(RingWinding)`, but a margin decided Zero returns
`JoinDesync { what: DEGENERATE_RING_WINDING }` (`join.rs:4584`). That
is a desync, the kernel's own error class, reached by a geometric
input. The only Zero it rewrites is the pinch-ray tie
(`pinch_ray_refusal`, `join.rs:4530`).

## Reproduction (main)

B is a unit rod about `y` over `[−2, 2]`, its seam turned 0.4 rad about
its axis. The second operand is the brick `x ∈ [0, 2]`, `y ∈ [−h, h]`,
`z ∈ [0.301, 2]`, whose bottom face cuts B's wall along a ruling `2h`
long. Both operands are re-posed (a rigid motion off the axes), every
op:

| ε | h = 1e-8 | h = 1e-6 | h = 1e-3 |
|---|---|---|---|
| 1e-9 | escalates `SelfCheck(RingWinding)` | builds | builds |
| 1e-12 | `JoinDesync` "ring-run winding is degenerate" | `JoinDesync` | builds |

The direct poses fail earlier, at the ruling's pcurve (the pcert item
above).

## The question

A run along a ruling `2h` wide encloses an area of order `h`, whatever
the band's scale. So a Zero at ε 1e-12 for an `h` of 1e-6, where 1e-9
builds, suggests the area margin is levered or read so that the run's
true area falls inside the zero band at the finer ε. Either the
reading is wrong, or a definite Zero here is a degenerate input that
deserves a typed refusal with a recourse, not a desync.

