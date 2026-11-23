---
id: row-raised-bands-cost-grows-with-the-band-spread
kind: issue
title: Rows raised for narrow-column bands cost a face in proportion to its band-aspect spread: up to 14x triangles on an uneven-station cylinder at e = 1e-4
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

`nurbs_cert::NurbsCellGrid::band_schedule` raises a band's rows when its
residual aspect over the face's lattice metric falls below `1/√8`
(`MAX_RESIDUAL_SQ`), which makes the uneven-station cylinder mesh at every
review δ instead of refusing. The metric is the geometric mean of the
extreme band aspects, so as a face's thin band gets thinner the metric
rises, and the thick bands, which hold almost all the area, take the rows.

Measured on degree-2 lofts through z = 0, e, 2e, 0.5, 1 at the review's
twenty δ (triangles, against no raise, on rows that meshed both ways):

| body | e | mean | max |
|---|---|---|---|
| cylinder, 4 arcs | 0.03 | 1.32x | 1.32x |
| cylinder, 4 arcs | 0.01 | 1.67x | 2.30x |
| cylinder, 4 arcs | 0.003 | 2.62x | 3.30x |
| cylinder, 4 arcs | 0.001 | 3.86x | 5.22x |
| cylinder, 4 arcs | 1e-4 | 11.00x | 14.32x |
| rounds, 2 and 4 arcs | any | ≤ 1.08x | 1.46x |
| degree-3 lofts | any | ≤ 1.07x | 1.48x |

The cylinder at e = 1e-4 totals 4.74M triangles over the twenty δ. A line
at `√15`, which the measured window also allows (`√8` to at least `6` all
mesh), would cost 3.52M; `√8` is the line the strip-sliver bound derives
(`MAX_RESIDUAL_SQ`'s doc).

At e = 1e-6 the cylinder still meshes (measured at `√15`: 236k–3M triangles
per δ). On the tour the cost is 0 (#5570: the `--sizing-only` sweep is
identical to its base).

tess-meter's split-slack column absorbs the row raise as the price of the
cap, and it has no raised-band indicator. It reads 0 on the tour today; a
body that pays this cost would show it only there.

## Alternatives measured and rejected

An area-weighted metric, with columns raised for wide-column bands and rows
for narrow ones, cures the same refusals but costs far more: 337x mean on
the cylinder at e = 1e-4, up to 1.9x on the rounds. The thin band's extra
columns run the face's full width.

## If a real body pays it

The lever is the metric: choosing it so the bands that hold the area sit
near square, and remedying the thin bands some way other than full-width
columns. Nothing to do until a body pays it.
