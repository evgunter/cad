---
id: indeterminate-display-offers-declare-on-a-poisoned-margin
kind: issue
title: geom-core: the bare Indeterminate Display offers 'declare the coincidence' on a poisoned margin
status: open
opened: 2026-10-10
---


(Filed by ENCL from the sweep of `topo-poisoned-escalations-offer-unfollowable-endings`. Pre-existing.)

## What

`geom_core::Indeterminate`'s `Display` (`predicate.rs`, `impl fmt::Display for Indeterminate`) renders the payload, then `COINCIDENCE_RECOURSE`, then, on a poisoned margin, the build's unreadable-margin note. So a NaN, or a site's `invalid_margin::invalid` contradiction, reads "Recourse: declare the coincidence, or move the geometry; an unreadable margin may indicate a kernel bug worth reporting". No declaration makes an unreadable margin readable.

PR 4475's rule keeps a poisoned margin's **decision's own** lever. The coincidence menu's declaration arm is that lever only where a coincidence is declarable.

The readers that still render the bare `Display` on a margin that may be poisoned include:
- `geom_brep::SectionError::Escalated` (`intersect.rs`);
- `ChartRegionError::Escalated` on a margin that was read (its poisoned arm ends in the defect ending since the ENCL PR for the row above);
- the blend and sweep doors that print `{source}`.

## Repair shape

Decide at the geom-core home what the bare `Display` ends a poisoned margin in. Either it ends in the move arm alone with the note, or the readers that cannot declare compose `Indeterminate::undecided` with their own ending, as `SplitReduceError::CrossingEscalated` does. Then sweep the `{diag}` / `{source}` readers.
