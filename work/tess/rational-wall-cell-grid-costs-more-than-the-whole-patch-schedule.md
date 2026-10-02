---
id: rational-wall-cell-grid-costs-more-than-the-whole-patch-schedule
kind: issue
title: on rational skinned walls the per-cell grid builds more cells than the whole-patch schedule (held < 1)
status: open
opened: 2026-10-02
priority: P3
---

Met while giving the tour lily's swept leaves lanceolate sections
(`demos/tour/src/lily.rs`, SHOW unit `lily-lanceolate-blade-sections`):
a lens of two arcs, swept along a nine-station spine and skinned at
degree 2 along the path, so every lateral wall is a RATIONAL patch.

## What the sweep reads

The committed baseline's rows for the two leaves
(`docs/tess-budget-data/tess-budget-baseline.csv`), per lateral wall:

| face | triangles | cells | grid_cells | patch_cells | span_opt_cells |
|---|---|---|---|---|---|
| `lily_leaf_b` 2 | 3878 | 1792 | 2016 | 630 | 1792 |
| `lily_leaf_b` 3 | 2534 | 1792 | 1344 | 408 | 1792 |
| `lily_leaf_c` 2 | 3642 | 1792 | 1904 | 493 | 1792 |
| `lily_leaf_c` 3 | 2298 | 1792 | 1232 | 297 | 1792 |

`tess-lint` reports `held` (`patch_cells / grid_cells`) at **0.3×** on
both scenes: the shipped per-cell grid builds about three times the
cells the retired whole-patch schedule would, on the same
certificates. Everywhere else in the corpus `held` is the TESS-SPAN
gain, at or above 1; the teapot's rational spout walls are the other
place it falls below (0.7× on the sweep this was measured beside).

## The mechanism, as the columns show it

`cells` is 1792 = 7 knot spans × 16 × 16: the rational arm's bound is
assembled on the cells of `patch_bound::RATIONAL_CERT_SPLITS`
refinement (`mesh::nurbs_cert::nurbs_cell_bounds` says so), and the
per-cell grid sizes each of those cells on its own. `span_opt_cells`
equals `cells` on every row — the cheapest split per cell is ONE
division per cell — so the per-cell schedule is floored at the
refinement's cell count, whatever the surface needs. The whole-patch
schedule, sized by one sup over the face, needs 297 to 630.

So on a mildly curved rational wall the per-cell grid's floor, not
its curvature, is what the triangles pay for. A polynomial wall's
cells are its raw knot spans (six on the kite leaves these replaced),
which is why the floor never showed.

## What would close it

Not this lane's to choose. Two shapes the numbers suggest: let the
per-cell grid merge cells whose own steps exceed the cell (a cell
needing a fraction of a division donates it to its neighbours), or
size a rational face by `min(per-cell, whole-patch)` since both are
certified. Either restores `held ≥ 1`.

## Cost today

The two leaves went 454 and 384 triangles (kite) to 6468 and 5992
(lens), with `total` slack 30.8× and 31.8× against the corpus's
other Hessian-sized scenes' 16–29×. Not a budget blow-up the scene
should be coarsened for; recorded so the floor has a row.
