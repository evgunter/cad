---
id: topo-planar-offset-cramer-anchors-at-the-world-origin
kind: issue
title: topo's planar offset corner and plane-plane section solve their three-plane Cramer systems from the world origin, so the point sits about u·|p|/det off its planes
status: open
opened: 2026-10-19
priority: P2
cost: E
---


Found by the sibling sweep of `band/blend-corner-ball-cramer-anchors-at-the-world-origin`
(PR `band/arms-centre-placement`), whose fix anchored `sweep::blend::arms::corner_ball`
at its first vertex.

## The sites

- `crates/topo/src/offset_together.rs` `cramer` (called from
  `solve_planar_corner`): each plane arrives as `(n, d)` with `d`
  measured from the world origin, and the point is
  `(d₁·(n₂×n₃) + d₂·(n₃×n₁) + d₃·(n₁×n₂))/det`.
- `crates/topo/src/offset_derive.rs` (the plane–plane section, "Three
  planes: the two given, and the one across `dir`"): `o = Point3::origin()`,
  `dᵢ = nᵢ·(qᵢ − o)`, the line's origin `o + x`.

## Why it matters

The right-hand sides carry `u·|p|` of rounding, and the cross products'
own rounding (`u` absolute) times `|d| ≈ |p|` projects onto every
normal, so the point sits about `u·|p|/det` off its planes, not
`u·(|p| + size/det)`. Measured on the blend twin (generic normals, a
vertex ~1 km out): 229 ulps of `|p| + r/det` off a plane at
`det ≈ 1e-4`, against 1.5 after anchoring.

## What it needs

Solve for the offset from a point the system already has (the old
vertex; `q₁`), i.e. right-hand sides `nᵢ·(qᵢ − q₁) + offsetᵢ`, then add
`q₁` back — as `corner_ball` now does. A row should pin a corner far
from the origin with near-dependent normals.
