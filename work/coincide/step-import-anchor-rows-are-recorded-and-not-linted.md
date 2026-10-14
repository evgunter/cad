---
id: step-import-anchor-rows-are-recorded-and-not-linted
kind: issue
title: STEP import's census rows are recorded on StepImport::Solid and nothing lints them
status: open
opened: 2026-10-10
priority: P3
cost: M
---


## What

`the-census-records-what-it-decides` retired the anchors and widened
the silence: every touching STEP import now records each touch between
two of its solids as a `Coincidence { site: CensusAtRest, cells:
RowCell::Result }` on `StepImport::Solid::coincidences`, cited by the
records on `StepImport::Solid::contacts` (`topo::record_at_rest`). An
import is not a document node (`pncad-py` calls `step_import`
directly and drops the rows, `py/value.rs` `import_step`), so nothing
names those rows' cells and the `unproven-coincidence` lint never reads
them. D7 step 4 says an imported assembly's contacts are
`unproven-coincidence` findings, and at no reading door are they
(B2's orchestrator ruling, 2026-10-10: file, do not build; re-pointed
by J0's review, 2026-10-11).

## What closing it takes

A home for the rows where the lint reads: either the import becomes a
document operation whose value carries `coincidences` (named in its own
output table, `RowCell::Result`), or the Python import surface reports
them through the same `Check.unproven_coincidence` shape.
