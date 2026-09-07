---
id: report-header-column-phrases-unqualified
kind: issue
title: tess-lint's report header names its cell columns by phrase only, and "the cheapest split" is two different columns
status: open
opened: 2026-09-07
---


## What

`tools/tess-lint/src/main.rs:284-289` prints the sweep's three cell
totals as prose phrases with no column name attached:

```
grid cells over all Hessian-sized faces: 46019 used (per-knot-span-cell,
TESS-SPAN); whole-patch counterfactual 110811 (2.4x held), 44446 at the
cheapest split per cell (1.0x still recoverable)
```

Those are `grid_cells`, `patch_cells` and `span_opt_cells`. `opt_cells`
is not printed at all. Every phrase is correct against
`tools/tess-meter/src/lib.rs:337-342`, which is the definition of
record — this is not a wrong name, it is a name that cannot be joined
to a column without a second lookup.

**"The cheapest split" is the phrase that bites.** It names
`opt_cells` (cheapest split under the WHOLE-PATCH bound) in
`tools/tess-meter/src/lib.rs:112` and in `docs/TESS-BUDGET.md`'s
column list, and `span_opt_cells` (per-cell sizing AND the cheapest
split in each cell) in the report header's *"at the cheapest split per
cell"*. Only the trailing qualifier separates them, and dropping it is
the mis-read that has now been made three times in a row against
`docs/TESS-BUDGET.md`'s pre-fix block — most recently by this
program's own opening note, which put the block's `span_cells` line
(154,129) against `span_opt_cells` (44,446) and read a 3.5x factor as
staleness.

## Finding

The cheap fix is to print the column name beside each figure —
`grid_cells 46019 used`, `patch_cells 110811`, `span_opt_cells 44446`
— and to print `opt_cells` too, so that a document citing the report
can cite a column rather than a phrase, and so a reader holding an
older document can join its vocabulary to the current one without
opening the meter. `docs/TESS-BUDGET.md` now carries that join as a
table (the pre-fix block's four phrases against today's four columns);
that table is the workaround, not the cure, because it lives in the
document rather than beside the numbers.

**Not fixed in the unit that found it** (METER unit 2,
`tess-budget-doc-finding-block-stale`): the unit's fence is
`docs/TESS-BUDGET.md` and `tools/tess-lint/tests/`, and
`tools/tess-lint/src/lib.rs` was held by another lane at the time, so
a header change would have touched `main.rs` alone while the
`SceneTotals` field docs it should agree with were moving next door.
The header's wording and the module docs' should be decided together.

**Confidence:** sure for the two definitions and the phrase collision
(both read at their declaration sites); a judgement call whether the
header should carry column names at all, since it is a human report
and not a machine format.

## Was

disclosed by METER unit 2, in the PR that landed the document's cite-
not-restate fix.
