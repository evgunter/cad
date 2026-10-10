---
id: step-import-anchor-rows-are-recorded-and-not-linted
kind: issue
title: STEP import's anchor rows are recorded on StepImport::Solid and nothing lints them
status: parked
blocked_on: [contact-records-cite-their-decision]
opened: 2026-10-10
priority: P3
cost: M
---


## What

B2 (`contact-records-cite-their-decision`) records each STEP import
anchor (`step-import` `vertex_rest_contact`: two vertices within the
file's ε_in of `ImportContact::VertexRest::at`) as a
`Coincidence { site: ImportAnchor, cells: RowCell::Result }`, carried on
`StepImport::Solid::coincidences`, and the at-rest records cite it. An
import is not a document node (`pncad-py` calls `step_import`
directly), so nothing names those rows' cells and the
`unproven-coincidence` lint never reads them: an anchored contact in an
imported assembly is a value-decided coincidence D10 says is linted,
and it is silent (orchestrator ruling on B2, 2026-10-10: file, do not
build in B2).

## What closing it takes

A home for the rows where the lint reads: either the import becomes a
document operation whose value carries `coincidences` (named in its own
output table, `RowCell::Result`), or the Python import surface reports
them through the same `Check.unproven_coincidence` shape. F retires
`ImportOptions::declared_contacts` with the rest of the declaration
channels; if it lands first, the anchors and their rows go with it and
this row closes with nothing built.
