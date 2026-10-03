---
id: d10-one-way-to-say-intent-is-unbuilt
kind: issue
title: D10 (one way to say dependency, placement and intent) is ratified and unbuilt: the program that builds it, and the hold it carries
status: open
opened: 2026-10-03
priority: P0
cost: H
refs: [one-way-to-say-dependency-and-intent]
---


Ev ratified DESIGN.md D10 on PR #3990 (ruling
`one-way-to-say-dependency-and-intent`, closed). The code still has
every mechanism D10 retires (D10's last paragraph lists them). This row
is what the refactor hold waits on: a row parked by the hold carries
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`, and it fires when
this row closes, which is when the build has reached the ground the
row stands on. Units already started may finish (Ev). The build is
P0 throughout (Ev): it needs its own program, staged roughly as
(1) variables and no dimensioned literal in a slot, derived
parameters; (2) operations, the explicit product list, one dependency;
(3) spaces, placements as mates, the world frame, the per-space
computing frame; (4) the coincidence door, canonical carrier forms and
the `unproven-coincidence` lint, retiring declared pairs and the
undeclared refusals; (5) assertions and the at-rest lint quieting;
(6) the tangency constructions (a surface's trace in a sketch plane).
