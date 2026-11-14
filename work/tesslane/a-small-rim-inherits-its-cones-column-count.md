---
id: a-small-rim-inherits-its-cones-column-count
kind: issue
title: a cone face's small rim is chorded at the face's column count: 20,000 chord points on a rim of radius 2e-8 m
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by a designer of the flat-triangle fork (`docs/DESIGN-FORK-LOG.md` row 269), who built it; two others reproduced it.

## What

The reach probes' frustum `revolved[(0,0), (1,0), (r,1), (0,1)]` with a small rim of radius `r` = 2e-8 m (20ε at the default ε), whole, at δ 1e-8. The cone face's lattice is a tensor grid, so the 2e-8 m rim gets the column count the 1 m rim needs: 20,449 columns. The chord pass follows the lattice step (`chords::tighten`, the rim-chord rule of `crates/mesh/README.md` `[mesh.sizing]`), so the rim is chorded into about 20,000 points 6e-12 m apart, and the planar cap on that rim inherits them.

Measured on main:

- 81,792 triangles; `check_mesh` passes; the volume is π/3.
- 6,780 triangles (6,610 with the body moved by `(1, 0, 1)`) sit on three boundary points with a cross product within 8 ulps of their coordinates times their longest side. All are in the cap's patch. Three consecutive chord points bend by about 1.1e-15 m. None is wound against its face in these runs.
- At δ 1.5e-8: 3,747. At 2e-8 and above: none. At 3e-9 the cone face refuses `CertificateExceeded`.
- Cut by the tilted plane of `trimmed`'s tests at δ 1e-4, the same small rim gives 226 certified triangles with a lattice corner whose least height is at or under ε (least 5.6e-10 m): the rim's chords are `r·θ` long.

## Why it matters

It is waste under "as coarse as possible": a rim of radius 2e-8 m needs a handful of chords at any δ the body can be meshed at. And it is the only construction found, in a census of 40.6 million triangles and three designers' probes, that makes a triangle whose winding f64 cannot resolve on a body valid at rest. Under the promise the fork recommends, that body refuses typed for δ between about 3e-9 and 2e-8 until this is cured.

## Where it belongs

The same question as `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`: what a tensor lattice lays at a cone's small end. A cure there (columns that thin toward the small end, or a rim chorded from its own sagitta where the lattice does not need its points) removes this instance. Not chosen here.
