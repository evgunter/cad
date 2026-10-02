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
