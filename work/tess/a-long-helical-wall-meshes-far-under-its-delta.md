---
id: a-long-helical-wall-meshes-far-under-its-delta
kind: issue
title: a six-turn helical sweep's walls mesh 26x under their delta, one cell row per 1.7 mm along a 5.3 m wall
status: open
opened: 2026-10-03
---


Met by SHOW's `long-turn-helix-has-no-demo` (PR 3918): the spring in
`projectbox` (`demos/tour/src/projectbox.rs`, `spring`), a 0.05 m
square wire swept by `sweep_body` along a six-turn helix (radius
0.14 m, pitch 0.08 m, 193 stations, v-degree 3). Each of its four
lateral walls is ONE integral NURBS patch whose v direction runs the
whole coil, about 5.3 m.

## What the sweep reads

`docs/tess-budget-data/tess-budget-baseline.csv`, rows
`projectbox/spring` 2–5 (and `projectbox/cutaway_spring`, the same
body moved), at δ = 1e-2:

| face | triangles | nu × nv | worst_cert | worst_dev | grid / patch / span_opt cells | realized_aspect |
|---|---|---|---|---|---|---|
| 2 | 8536 | 2 × 3188 | 1.24e-3 | 3.77e-4 | 2136 / 6376 / 1946 | 1266 |
| 3 | 8203 | 2 × 3118 | 1.24e-3 | 5.54e-4 | 1800 / 6236 / 1504 | 1260 |
| 4 | 8566 | 2 × 3215 | 1.23e-3 | 3.84e-4 | 2136 / 6430 / 1946 | 1266 |
| 5 | 8271 | 2 × 3212 | 1.22e-3 | 3.82e-4 | 1844 / 6424 / 1510 | 1266 |

- **Far under δ.** The certificate is 8× under δ, and the resampled
  deviation 26× under. The v direction is cut about every 1.7 mm on a
  surface whose curvature radius is the coil's, 0.14 m.
- **Lopsided cells.** `realized_aspect` is about 1266, 3.4× the
  corpus's next-largest.
- **One scene now dominates a register metric.** These eight walls
  carry about 67k of the sweep's 494k triangles. They alone move the
  baseline's held span gain (`patch_cells / grid_cells`, pinned in
  `tools/tess-lint/tests/baseline_census.rs`) from 1.048 to 1.265.

## A reading, unconfirmed

`mvv` is about 1.5e3 on a parameter v ∈ [0, 1] spanning the whole coil.
A v-direction bound taken over one long patch prices every cell at the
patch's worst span, and the v parameter's speed (≈ 5.3 m per unit)
squares into it. Per-span bounds, or splitting a long skinned wall at
its knots before sizing, would size each cell from its own span.
Untested.

## Done when

The spring's walls mesh within a small factor of δ (or the remaining
factor is shown to be what the certificate costs on this patch, with
its mechanism named), and the census pins are re-derived from the
re-cut.
