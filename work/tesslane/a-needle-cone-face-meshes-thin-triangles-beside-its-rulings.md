---
id: a-needle-cone-face-meshes-thin-triangles-beside-its-rulings
kind: issue
title: Cone faces near their apex or with a needle half-angle mesh triangles as thin as 2·area/L² ≈ 1e-8 beside their rulings; nothing bounds a triangle's shape
status: open
opened: 2026-11-14
priority: P3
---

## Finding

Nothing bounds an accepted triangle's shape on a cone face. Beside a ruling
a triangle's `2·area/L²` (`L` its longest side) goes as `(h/L)²`, `h` its
width across the ruling, and accepted, certified meshes read (measured on
`tesslane/cone-near-apex`, whose cone rows are uniform in `√|v|`; least over
the cone faces):

- a needle funnel, `revolved[(0,0), (0.002,1), (0,1)]` split by the tilted
  plane through `(0, 0.5, 0)`: `7.6e-7` (above) and `7.9e-9` (below) at
  δ `1e-5`; `7.7e-8` and `2.5e-10` at δ `1e-6`;
- a frustum whose small rim has radius `2e-7` / `5e-7`,
  `revolved[(0,0), (1,0), (r,1), (0,1)]`, at δ `5e-3`: whole (curved lane)
  `3.2e-8` / `7.9e-8`, its upper half split as above `8.5e-6` / `2.1e-5`;
- the unit funnel cut `1e-6` above its apex, upper half, δ `2e-2`: `1.4e-5`.

The thinnest are on the needle's face through its apex: a ruling is a line
edge with no chord point between its ends, so the triangles beside it fan
from its far end to consecutive points of the lattice's first column, and
each is as thin as its two rows are close. Rows graded toward the apex
(`chart_table`'s `lattice_rows`) stand closer there than rows uniform in
`v` did, which read `1.6e-7` and `1.6e-8` on that face; the faces ending
near an apex, whose small end the graded rows reach, read `3.1e-7`,
`7.6e-7` and `9.6e-7` under uniform rows.

Each mesh passes `check_mesh`, encloses its volume and lies within δ. The
reach probes' shape check (`crates/mesh/tests/reach_probes.rs`, `MIN_SHAPE`)
reads only "not on one line" (`1e-12`), because a triangle exactly along a
ruling cannot be built in the development and reads about `1e-17` in the
chart.

## Why it matters, and what would bound it

A thin triangle is certified and correct; it costs a downstream consumer
(normals, a renderer's shading, a FEA mesher) and is a quality question,
not a soundness one. A bound would come from the lattice's aspect at the
face's small end (the cone's `lever` row step against its circumference
there) or a quality pass after the CDT; neither is designed. Related to
`a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md` (the same
small-end geometry), not the same row: these meshes are accepted.
