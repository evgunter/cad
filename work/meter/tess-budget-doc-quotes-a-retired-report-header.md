---
id: tess-budget-doc-quotes-a-retired-report-header
kind: issue
title: docs/TESS-BUDGET.md quotes a report-header phrase the header no longer prints, and says the command omits opt_cells
status: open
opened: 2026-09-08
---



## What

`docs/TESS-BUDGET.md` describes `tools/tess-lint`'s report in two
places, and unit 9 moved the report out from under both. The document
is another lane's fence, so the unit that broke them files this rather
than fixing them in its own diff.

* **`docs/TESS-BUDGET.md:264`** — *"all four cell sums — `opt_cells`
  included, which the command does NOT print"*. The command prints it
  now: the four cell totals are one line each, under their column
  names. The sentence is drawing a real distinction between the
  command and `baseline_sizing_census.rs` (the command is the reading,
  the two test files are the guard) and that distinction survives —
  the command still does not assert the face counts or the two
  percentages. Only the parenthesis is false.
* **`docs/TESS-BUDGET.md:389`** — *"the report header's **at the
  cheapest split per cell** is `span_opt_cells`"*. The header no
  longer contains that phrase; it prints `span_opt_cells` by name,
  with `opt_cells` on the line above it. The clause the quote serves
  — that the qualifier is the whole of the difference between the two
  columns — is still exactly right, and the report is now a witness
  FOR it rather than the thing it has to warn about.

Neither is a wrong number; both are a document describing an
instrument it has stopped matching, which is the same class as
`tess-budget-doc-finding-block-stale` one turn later.

## Next

A `docs/TESS-BUDGET.md` lane. Both edits are one clause each and
neither moves a figure. The second site is worth doing WITH the first
rather than deleting: the pre-fix vocabulary table above it exists to
join old documents to today's columns, and "the report header spells
the column" is the sentence that retires the workaround.

## Was

disclosed by METER unit 9, in the PR that changed the header.
