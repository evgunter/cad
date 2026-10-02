---
id: stacked-two-half-rods-with-aligned-seams-refuse-unpaired-loose-ends
kind: issue
title: Two stacked two-half rods with their seams aligned refuse Join(UnpairedLooseEnds) with every finding declared; rotated seams build
status: open
opened: 2026-10-01
priority: P2
cost: M
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

## Builds on TANG's branch (PR 3823, `tang/abutting-rim`)

The four loose ends are the mating circle's two semicircles, which the
declared-REST zip's straight-chord facing test cannot pair (their germs
are square to the chord) and `fan_edge_between` sees twice between one
vertex pair. PR 3823's zip matches germs along circle arcs both operands
carry first (`boolean/arcs.rs`, `arcs_along`), so the aligned pose
builds there: `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
stacks `rod_z(1, 0, 2)` and `rod_z(1, 2, 1)` (two semicircular wall
halves each, seams aligned), discs `Rest` and walls continuations, and
pins 3π, `(6, 10, 6)` faces, edges, vertices, one shell, tier 3 and 3′,
in both orders. This row can close when PR 3823 lands.
