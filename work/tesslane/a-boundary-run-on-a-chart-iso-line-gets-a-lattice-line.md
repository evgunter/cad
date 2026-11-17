---
id: a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line
kind: unit
title: A boundary run on a chart iso line gets a lattice line through it, and a face bounded by such runs alone sizes at δ
status: parked
opened: 2026-11-16
priority: P1
cost: M
blocked_on: [the-lattice-keeps-clear-of-the-boundary-and-every-winding-is-resolved, a-curved-face-is-routed-by-its-domain-not-its-carriers]
---



Unit 4b of the chart lane (`work/tesslane/plan.md`, The chart lane), cut from unit 4 (`a-curved-face-is-routed-by-its-domain-not-its-carriers`) on 2026-11-16.

## What it is

The plan's sizing rule has two halves: a boundary run on a chart iso line gets a lattice line through it and sizes at δ, and a run that crosses lattice lines sizes at the one named `UNPROVED_HEADROOM` until its lemma is proved (unit 5). Unit 4 routes a notched face to the chart lane and meshes it at the lane's present target, the headroom's share of δ, on the uniform lattice. This unit is the first half of the rule: on a face whose every boundary run is an iso line, the lattice takes a line through each run, every boundary point is a lattice point, and the face sizes at δ with no headroom, as a rectangle does.

## Why it waits

Where a lattice point may sit beside the boundary is the question before Ev on PR 6966 (the lattice keeps clear of the boundary; every winding is resolved). A lattice line laid through a run puts lattice points on the boundary on purpose, and two runs at nearly one coordinate put two lattice lines nearly on each other, so this rule and that one are one design and are cut together once that PR is answered.

## Scope

- Only a face bounded by iso runs alone changes its lattice and its target. Every such face refuses on main before unit 4, so nothing that meshes on main moves. A face with one oblique run keeps today's lattice and target until unit 5.
- It is density, not reach: the faces mesh without it, at about twice the triangles a rectangle of the same size takes.

## Done when

The notched pipe walls of `reach_probes` mesh at δ with no headroom and their triangle counts are stated beside the counts unit 4 pinned; a lattice line within rounding of a run or of another line is one line, pinned on a body that reaches it; `[mesh.sizing]`'s wording of the headroom names which faces it covers.

- 2026-11-16 — **PR 6966 is answered and merged**: a lattice node is not laid within a clearance of its own step from the boundary. This unit now waits on the unit that builds that rule and on unit 4, which are both in the lattice's and the router's code; it is cut after them, as the rule above read against the clearance as built.

- 2026-11-17 — **"About twice the triangles" above holds only at coarse δ, and not on a ruled face.** Measured by three designers on the fork of that date (row 279 of `docs/DESIGN-FORK-LOG.md`): the chart lane lays rows a cylinder or cone rectangle does not need, and the ratio grows as δ shrinks. A unit 45° cone, full turn: 8 against 58 triangles at δ 0.1, 24 against 604 at 0.01, 72 against 6168 at 0.001; slender cones 150 to 500 times. With its rows switched off the same lane emits 100 against the rectangle lane's 72, and that remainder is the headroom's finer rim chords. The ruled-chart half of this unit is cut out as unit 4c (`a-ruled-face-is-meshed-as-a-strip`); this unit keeps the sphere and torus rectangles.
