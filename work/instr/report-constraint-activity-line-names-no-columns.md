---
id: report-constraint-activity-line-names-no-columns
kind: issue
title: tess-lint's constraint-activity line prints four indicator columns under prose names
status: open
opened: 2026-09-08
---



## What

`tools/tess-lint/src/main.rs`'s constraint-activity line is the same
shape unit 9 fixed one block above it:

```
  constraint activity: 616 A-cap-bound band(s), 588 snap-projected band(s) of 616; worst realized s_u/s_v 795.51 (lily/lily_leaf_a face 7)
```

The four figures are `cap_bands`, `snap_bands`, `bands` and
`realized_aspect`, and the line names none of them. A reader joining
the report to the CSV resolves *"A-cap-bound"* and *"snap-projected"*
in `tess_meter`'s field docs first, exactly as they had to for the
cell totals.

**Lower stakes than the cell totals, which is why unit 9 swept it and
left it.** No two of these columns share a phrase, so there is no
collision to make and no demonstrated mis-read; the cost is a lookup,
not a wrong number. `bands` is also printed twice in one clause
(*"588 … of 616"*), where the block above would give it a line.

## Next

A `tools/tess-lint/src/main.rs` lane, with
`tools/tess-lint/tests/report_columns_pin.rs` as the shape to extend:
its `CELL_TOTALS`-style table carries the column name beside the
accessor, and the indicator block would take the same treatment. Worth
deciding at the same time whether `worst realized s_u/s_v` should say
`realized_aspect` at all, since the symbol is what a reader of
TESS-SPLIT recognises and the column name is what a reader of the CSV
needs.

## Was

disclosed by METER unit 9's sweep of the report's other print sites,
which is where the class was looked for.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
