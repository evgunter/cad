---
id: restating-a-chart-image-across-an-authority-has-several-spellings
kind: issue
title: restating an edge as a chart image while carrying its declared authority across is spelled at four sites
status: open
opened: 2026-10-06
priority: P3
cost: E
---



## What

Four sites build "a derived chart image in chart K, keeping the edge's declared authority if it
has one" by hand. Each matches on `EdgeAuthority` and calls `EdgeDescriptionSpec::declared_by`:

- `splitting::finish::section_plane_restatements`: `EdgeDescriptionSpec::chart(chart)`, then
  `Declared(mc) => image.declared_by(mc)`, `Derived => image`.
- `splitting::finish::describe_section_boundary`'s smooth arm: a conventional
  `EdgeDescriptionSpec::chart(s_self)`, then `if let Some(Declared(mc)) = existing.authority()`
  re-declares it.
- `offset_restate::held_neighbour_image`: `EdgeDescriptionSpec::chart(moving).declared_by(mc)`
  for `Some(mc)`.
- `transform.rs`'s chart arm (`EdgeDescription::Chart`): the stated image is kept, and the
  authority travels through `map_mapped_curve`. That makes it a variant, not a copy. The others
  carry the authority verbatim.

`EdgeCurve::restated_description` (`geom-brep/src/certify.rs`) is the existing home for "this
edge's description as a construction states it". A sibling there could take the chart it is
re-imaged into, for example `EdgeCurve::reimaged_in(chart) -> EdgeDescriptionSpec`, and serve
the first three sites. `step-import`'s `adopt.rs` (`chart(chart).declared_by(mapped)`) declares a
fresh authority rather than carrying one, so it is outside this row.

## Owed

One home for the carry, or a reason the four spellings differ. Not measured: whether any two
sites disagree on an input.

## Found by

The review of PR 4158 (S5).
