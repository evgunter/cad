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

## Moved to TESSLATTICE (the NURBS lattice: its share, its rows and cells, and what it costs on swept and lofted walls) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

## A second long wall: lily's one-loft leaf (2026-10-29, SPLICE)

Filed by SPLICE's lily-one-loft lane (PR 6042). `lily_leaf_a` became one
loft from a 0.34 m sheath (arc radius 0.4 m, 8 slabs) into the 5.1 m
blade (arc radius 8.2 m, 16 slabs), 25 stations. Each lens wall is still
ONE rational NURBS patch over the whole leaf. Its two walls went from
15138 to 24786 triangles at δ = 5e-3, a 1.64x growth that is the
tess-lint gate's only finding on that PR.

`docs/tess-budget-data/tess-budget-baseline.csv`, rows
`lily/lily_leaf_a` 2–3, before (cut `c3ff4a85a526`, the blade alone)
and after (cut `4809ce56162e`):

| face | triangles | nu × nv | mvv | grid / patch / span_opt cells | bands | realized_aspect |
|---|---|---|---|---|---|---|
| 2 before | 8502 | 21 × 119 | 9.56 | 4070 / 2499 / 3616 | 224 | 17.1 |
| 2 after | 13950 | 18 × 310 | 83.0 | 6500 / 5580 / 5632 | 352 | 121.8 |
| 3 before | 6560 | 17 × 126 | 9.51 | 3161 / 2142 / 3584 | 224 | 21.4 |
| 3 after | 10768 | 15 × 330 | 87.0 | 4977 / 4950 / 5632 | 352 | 147.9 |

`nu`, `nv` and `mvv` are the whole-patch counterfactual's
(`tools/tess-meter`, `NurbsColumns`): `mvv` is the sheath's curvature
taken as the face's sup, and `nv` what a whole-patch schedule would
cut. They show the counterfactual pricing the whole blade at the
sheath's span, the reading this row's first section gives for the
spring. The grid the lane BUILT is per band
(`crates/mesh/src/nurbs_cert.rs`, `NurbsCellGrid::band_schedule`, about
line 1065: each band's rows from its own `row_bound`), and it grew 4070
→ 6500 cells. What is read off the code, not yet measured per band:

- **Bands.** 224 → 352 is 8 more knot spans times the 16 dyadic reading
  pieces each forces as a row (`rational-pieces-force-rows`).
- **One face-wide metric.** `lattice_metric` (about line 738) is the
  geometric mean of the extreme band aspects, and `band_schedule`'s
  narrow-column raise (`let residual = b.aspect(du) / metric`, about
  line 1101) raises the rows of every band left below it. The sheath's
  bands lift the extreme aspect (`realized_aspect` 17 → 122), which can
  raise rows in the blade's bands, which the sheath does not touch.

How much of the 2430 added cells is each is unmeasured: a per-band dump
of `nvc` before and after the raise on this face would split them. If
the raise dominates, the finding is "one tight span re-rows the whole
face through the face-wide lattice metric", and the spring's walls may
be the same mechanism.
