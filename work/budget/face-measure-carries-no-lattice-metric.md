---
id: face-measure-carries-no-lattice-metric
kind: issue
title: FaceMeasure and the budget CSV carry no lattice metric, so no band's residual aspect over it is recomputable or guarded
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

The trimmed lane triangulates over each face's lattice metric
(`mesh::nurbs_cert::lattice_metric`), and what slivers there is a band's
RESIDUAL aspect: its realized `s_u/s_v` over the metric. `FaceMeasure`
records `realized_aspect` (the raw-UV max over bands), but not the metric.
So neither the budget CSV nor tess-lint can recompute a residual, and claims
such as "the tour's worst band residual is 0.41–1.88" (measured with a local
mutant for `the-lower-malign-arm-is-unwitnessed`, #5551) have no register
and no guard.

That item's to-do 2 named this column. It closed with the snap's
retirement, so the column is scheduled here.

## To do

Add `metric` (and the worst residual over bands, either side of 1) to
`FaceMeasure`, the tess-meter CSV and tess-lint's reader, and fold it into
the next baseline re-cut together with
`snap-bands-column-reads-zero-since-the-snap-retired`. Then the residual
claims can be read off the CSV, and a report line can say which faces sit
furthest from 1.
