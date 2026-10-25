---
id: snap-bands-column-reads-zero-since-the-snap-retired
kind: issue
title: The budget CSV's snap_bands column reads 0 since the column snap retired; drop it at the next baseline re-cut
status: open
opened: 2026-10-21
priority: P3
cost: E
---


## What

The malign-band column snap is retired
(`work/tess/the-lower-malign-arm-is-unwitnessed`). `mesh::budget::FaceMeasure::snap_bands`
is now always 0, and `tools/tess-meter`'s CSV and `tools/tess-lint`'s reader
keep the column so the baseline's shape does not move outside a re-cut.

## To do

At the next baseline re-cut (`tess-lint-gate-red-on-main-23-findings` is the
one pending), drop `snap_bands` from `FaceMeasure`, the tess-meter
`CSV_HEADER` and row, tess-lint's parser, admissibility table and report line,
and the baseline. `report_columns_pin` and tess-meter's derivation tests pin
the column list and move with it.

## Also from #5551's review

- tess-lint's report line still prints the snap-projected count, always 0
  (`tools/tess-lint/src/main.rs`, the constraint-activity block). It goes
  with the column.
- `Lane::Nurbs` still builds the whole-patch bound (`grid.patch()`) for
  every NURBS face, and since the snap retired its only reader is the
  budget meter's `muu`…`mv1` columns. Building it under the meter only, or
  reading those columns off the cell grid, saves a pass per face.
