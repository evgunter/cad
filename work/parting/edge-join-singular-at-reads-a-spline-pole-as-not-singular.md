---
id: edge-join-singular-at-reads-a-spline-pole-as-not-singular
kind: issue
title: edge_join::singular_at answers false for a spline chart by its last arm, so a pole on a spline face is read as regular
status: open
opened: 2026-11-01
priority: P3
cost: E
---

Disclosed and left unscheduled by PR 6220's sweep for last-arm
`Ok(false)` readers (2026-11-01): `edge_join::singular_at` falls to
`_ => Ok(false)` for a spline chart, so a degenerate (pole) point of a
spline face is read as regular. No witness measured. Find a spline face
with a pole (a revolved or lofted cap converging to a point) reaching
the edge join, and either read the pole or refuse typed.
