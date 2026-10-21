---
id: row-raised-bands-cost-grows-with-the-band-spread
kind: issue
title: Rows raised for narrow-column bands cost a face in proportion to its band-aspect spread: up to 8x triangles on an uneven-station cylinder at e = 1e-4
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

`nurbs_cert::NurbsCellGrid::band_schedule` raises a band's rows when its
residual aspect over the face's lattice metric falls below `1/√15`
(`MAX_RESIDUAL_SQ`), which makes the uneven-station cylinder mesh at every
review δ instead of refusing. The metric is the geometric mean of the
extreme band aspects, so as a face's thin band gets thinner the metric
rises, and the thick bands, which hold almost all the area, take the rows.

Measured on degree-2 lofts through z = 0, e, 2e, 0.5, 1 at the review's
twenty δ (triangles, against no raise, on rows that meshed both ways):

| body | e | mean | max |
|---|---|---|---|
| cylinder, 4 arcs | 0.03 | 1.18x | 1.32x |
| cylinder, 4 arcs | 0.01 | 1.43x | 1.65x |
| cylinder, 4 arcs | 0.003 | 2.12x | 2.62x |
| cylinder, 4 arcs | 0.001 | 3.05x | 3.92x |
| cylinder, 4 arcs | 1e-4 | 8.24x | 10.75x |
| rounds, 2 and 4 arcs | any | ≤ 1.03x | 1.26x |
| degree-3 lofts | any | 1.00x | 1.00x |

At e = 1e-6 the cylinder still meshes, at 236k–3M triangles per δ. On the
tour the cost is measured in #(B)'s body.

## Alternatives measured and rejected

An area-weighted metric, with columns raised for wide-column bands and rows
for narrow ones, cures the same refusals but costs far more: 337x mean on
the cylinder at e = 1e-4, up to 1.9x on the rounds. The thin band's extra
columns run the face's full width.

## If a real body pays it

The lever is the metric: choosing it so the bands that hold the area sit
near square, and remedying the thin bands some way other than full-width
columns. Nothing to do until a body pays it.
