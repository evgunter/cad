---
id: the-line-cone-lead-rung-is-levered-by-the-segments-length
kind: issue
title: The line × cone lead rung is levered by the segment's length, so a short edge reads as generator-parallel about ±15° off one
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [4135]
---


Left by the last fix pass of PR 4135 (the cone root lane), from its
second review (r2 m-2).

## What

`reduce::line_cone_roots` decides `bool_line_cone_lead` on
`Margin::levered(A/|dir|², L)` (`crates/topo/src/boolean/reduce.rs:3595`),
`L` the segment's length (`reduce.rs:3592`). On a short edge the lever
is small, so the rung reads a line well off any generator as
"parallel to a generator" and answers `Uncertain`.

Measured by r2: at scale 1e-3 and ε 1e-6, 13 of 240 random sweeps
escalate `bool_line_cone_lead`, at a margin of 6.36e-6 = `A·L` — `A`
about 0.06 on a 0.1 mm edge, roughly ±15° of direction.
`solid_contain::quadratic_roots` (`crates/topo/src/boolean/solid_contain.rs:4174`)
is stable at small `A`, so the lever is the rung's, not the solve's.
The filing `a-line-parallel-to-a-cone-generator-…` describes the
degeneracy itself, not this lever.

## The fix owed

A lever that measures the turn off a generator in the units the band
speaks (a length the roots move by), not the edge's length, with a row
of short edges at scale 1e-3 that answer and stay right.
