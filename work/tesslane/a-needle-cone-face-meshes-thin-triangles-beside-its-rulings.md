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
`tesslane/chart-table`, PR 6950's third pass; least over the cone faces):

- a needle funnel, `revolved[(0,0), (0.002,1), (0,1)]` split by the tilted
  plane through `(0, 0.5, 0)`: `9.0e-7` (above) and `1.6e-7` (below) at
  δ `1e-5`; `9.2e-8` and `1.6e-8` at δ `1e-6`;
- a frustum whose small rim has radius `2e-7` / `5e-7`,
  `revolved[(0,0), (1,0), (r,1), (0,1)]`, at δ `5e-3`: whole (curved lane)
  `3.2e-8` / `7.9e-8`, its upper half split as above `3.1e-7` / `7.6e-7`;
- the unit funnel cut `1e-6` above its apex, upper half, δ `2e-2`: `9.6e-7`.

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
