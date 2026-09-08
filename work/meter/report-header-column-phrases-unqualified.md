---
id: report-header-column-phrases-unqualified
kind: issue
title: tess-lint's report header names its cell columns by phrase only, and "the cheapest split" is two different columns
status: closed
opened: 2026-09-07
closed: 2026-09-08
branch: meter/9-report-header-columns
refs: [tess-budget-doc-quotes-a-retired-report-header, report-constraint-activity-line-names-no-columns, tess-budget-doc-finding-block-stale]
pr: 2180
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

**"The cheapest split" is the phrase that bites.** Unqualified, it
names `opt_cells` in `tools/tess-meter/src/lib.rs:112` and in
`docs/TESS-BUDGET.md`'s column list; with the trailing *"per cell"* in
the report header it names `span_opt_cells`. What each column IS is
stated once, in `NurbsColumns`' field docs
(`tools/tess-meter/src/lib.rs:337-342`), and is not restated here —
that is the point of the finding. Only the qualifier separates the two
phrases, and dropping it is the mis-read that has now been made three
times in a row against `docs/TESS-BUDGET.md`'s pre-fix block — most
recently by this program's own opening note, which put the block's
`span_cells` line (154,129) against `span_opt_cells` (44,446) and read
a 3.5x factor as staleness.

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

## Next

Rowed in `work/meter/plan.md` as unit 9, after the eight the plan
opened with: the header's wording and `SceneTotals`' field docs are
decided together, and `tools/tess-lint/src/lib.rs` has to be free for
that.

## Was

disclosed by METER unit 2, in the PR that landed the document's cite-
not-restate fix.

## Closed

The block prints one line per column, under the column's own name:

```
  cell totals over the 64 Hessian-sized faces, one line per CSV column (tess_meter::NurbsColumns defines them):
    grid_cells         46019  the grid the lane BUILT, sized per knot-span cell (TESS-SPAN)
    patch_cells       110811  the whole-patch-sup counterfactual, at today's point selection
    opt_cells          93066  the cheapest split, under the WHOLE-PATCH bound
    span_opt_cells     44162  the cheapest split, PER CELL — the recoverable denominator
  patch_cells / grid_cells = 2.4x held; grid_cells / span_opt_cells = 1.0x still recoverable
```

Four departures from the Finding above, each deliberate:

* **A block, not a sentence.** The Finding proposed the names inline
  in the existing prose line. Four name-and-figure pairs in one
  sentence is where a reader loses a qualifier for the second time,
  and it gave `opt_cells` nowhere to go. A line each puts the two
  twins adjacent with their figures in one eyeline — 93,066 over
  44,162 — which is the arrangement that makes the mis-read hard
  rather than merely detectable.
* **The two FACTORS were the same defect and the Finding did not say
  so.** *"held"* and *"still recoverable"* are as true and as
  unjoinable as *"the cheapest split"*; they are printed as their
  formulas in the columns' own names now.
* **The glosses stay, one clause each**, because a report is read by
  people. They say what a column is FOR here and are not definitions;
  the two twins' glosses are one sentence differing in exactly the
  qualifier that separates them.
* **`SceneTotals`' field docs CITE rather than restate.** Each cell
  field now names the `Nurbs` field of the same name and lets that
  one carry the pointer to `tess_meter`, which is unit 2's ratified
  form. What is stated here and nowhere else is the invariant the
  header now leans on: each cell field is named for the CSV column it
  sums and carries no other name.

`opt_cells` **is** printed. It is a column of the sizing block, it is
summed by `SceneTotals`, and `baseline_sizing_census.rs` asserts it;
the report was the only site that had it and said nothing. It is also
the column an unqualified "the cheapest split" names, so printing it
beside its twin is what retires the collision rather than documenting
it. That no rule divides by it is a reason to gloss it, not a reason
to hide it — the report prints `patch_cells`, which no rule reads
either.

The pin is `tools/tess-lint/tests/report_columns_pin.rs`, three
claims over a synthetic fixture whose four cell sums are pairwise
distinct: every printed name is a column of `EXPECTED_HEADER` and
every `*_cells` column is printed (the direction `opt_cells` was lost
in), each figure is its own column's sum, and the two factors are
printed as their formulas. `CELL_TOTALS` in `main.rs` carries the
accessor beside the name, so the transposition the pin's middle claim
watches for cannot be written as a positional-argument slip any more.

**Residue, filed rather than fixed:**

* `tess-budget-doc-quotes-a-retired-report-header` —
  `docs/TESS-BUDGET.md` quotes the retired phrase and states that the
  command does not print `opt_cells`. That document is another fence.
* `report-constraint-activity-line-names-no-columns` — the same shape
  one line below the block, over the four indicator columns. Swept
  and left: no two of those columns share a phrase, so the cost is a
  lookup rather than a wrong number.
