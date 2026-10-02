---
id: stacked-two-half-rods-with-aligned-seams-refuse-unpaired-loose-ends
kind: issue
title: Two stacked two-half rods with their seams aligned refuse Join(UnpairedLooseEnds) with every finding declared; rotated seams build
status: review
branch: reach/aligned-half-rods
pr: 3845
opened: 2026-10-01
priority: P2
cost: M
refs: [rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity, JOIN-1, dumbbell-joint-union-leaves-four-loose-ends]
---

Found by the review of PR 3657 (the continuation ruling), measured on
its head `d2d5b09076`.

## Repro

Two rods of radius 1 and height 1, each a cylinder whose wall is two
semicircular pieces (`circle_split(.., 2, ..)`), stacked on one axis:
z 0 to 1 and z 1 to 2. The top rod's seams are rotated by θ about the
axis. The detector offers five findings: the mating disc (`Rest`) and
four continuations among the wall halves.

- θ = 0 (seams aligned), every finding declared: refuses
  `Join(UnpairedLooseEnds { count: 4 })`
  (`crates/topo/src/boolean/join.rs:530`).
- θ = 0.7 and θ = π/2: builds, 6 faces, 3 recorded curved skips,
  volume 2π exact, tier 3 and 3′ valid. A third rod on top unions too
  (8 faces, 5 skips, 3π).
- Undeclared, or with only the mate declared, every θ refuses
  `UndeclaredCoincidence` on a wall pair, as ruled.

## What it looks like

With the seams aligned, each seam line of the lower rod ends exactly
where a seam line of the upper rod starts, on the mating circle. Four
loose ends fail to pair in the join, so the seam vertices on the
mating circle are the likely site. Not investigated further.

## Measured, and patched on the branch

On `origin/main` `cd49025f` the join refuses every pose of this stack
(`UnpairedLooseEnds`, 4 aligned, 8 turned); what builds the turned
poses is the declared-REST zip (`boolean::rest`). Its segment
enumeration pairs germs with the straight-chord facing test only. With
the seams aligned the mating circle carries two sites a half turn
apart, each germ is perpendicular to the chord between them (margin
~1e-16, decided zero), nothing pairs, and the join's refusal surfaces
verbatim. Past that, the two half-turn arcs between one site pair are
parallel edges, and `fan_edge_between` refuses `ParallelSeamEdges`.

This is ZIP's
`rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity`,
which `docs/JOIN-1-SPEC.md` assigns to JOIN-2 (the zip reads the join's
segments, and `enumerate_segments` / `fan_edge_between` go). The branch
patches the two functions in place instead; whether that lands ahead
of JOIN-2 is put to the orchestrator in the PR. Rows:
`crates/sweep/tests/reach_aligned_half_rods.rs`. The other three ops
refuse at every pose, as the rounded stack does
(`rounded-stack-subtract-and-intersect-refuse-fallback-extent`).
