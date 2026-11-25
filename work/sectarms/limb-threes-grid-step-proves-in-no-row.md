---
id: limb-threes-grid-step-proves-in-no-row
kind: issue
title: Limb 3's second reading of a chart, on the certificate's grid, proves in no row: 9,815 rows hold with the step removed
status: open
opened: 2026-11-25
---


Filed by the `sectarms/one-image-function` lane (PR 7537), from its
reviews' item 8.

## What

`ssi::certify::certify_branch` reads limb 3's chart operand in two
steps: the chart's own cells, and, where that refuses, the
certificate's grid (`ChartedNurbs::certificate_grid`,
`edge_nurbs::wall_grid`), in a trace's certificate and at rest alike.
Before PR 7537 a trace read the chart's own cells only and rest read
the grid only.

No row has the second step change a verdict. With the step removed
(limb 3 returns the own cells' refusal), the `ci` profile of `geom-brep`
(2,630 rows), `sweep` and `topo` (7,185 rows) is green but for
`ssi::certify::tests::every_rung_reads_the_operands_one_build_of_each_cell`,
which pins that the grid's cells are built (more than the wall's own
cell at its two readings), not that the grid proves anything
(measured 2026-11-25 on the PR's branch).

The grid's own rows (`edge_nurbs`'s `wall_grid_*`,
`tests/review_4553_pxn_probes.rs`) pin what the grid is, and that the
chart speeds over a few-span rational wall's own cells read up to a
thousand times loose; none shows a certificate that needs it.

The step is not free: the grid's enclosures are built again for each
certificate whose own cells refuse (`ChartedNurbs::charted_on`).

## Done when

One of:

- a fixture where the chart's own cells refuse limb 3 and the grid
  proves it, as a row; or
- the step and what only it reads are removed, with the measurement
  repeated over the slow set.
