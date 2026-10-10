---
id: indeterminate-display-offers-declare-on-a-poisoned-margin
kind: issue
title: geom-core: the bare Indeterminate Display offers 'declare the coincidence' on a poisoned margin
status: open
opened: 2026-10-10
---


(Filed by ENCL from the sweep of `topo-poisoned-escalations-offer-unfollowable-endings`. Pre-existing.)

## What

`geom_core::Indeterminate`'s `Display` (`predicate.rs`, `impl fmt::Display for Indeterminate`) renders the payload, then `UnderTail`'s poisoned arm, which puts "check the operation's inputs upstream, then" before `COINCIDENCE_RECOURSE`, then the build's unreadable-margin note. Today, at band (1e-9, 1e-8), a poisoned margin reads in full:

> margin is invalid (NaN or a refused enclosure) against the ambiguity band (1e-9, 1e-8) — Recourse: check the operation's inputs upstream, then declare the coincidence, or move the geometry; an unreadable margin may indicate a kernel bug worth reporting

The margin may be a NaN, or a site's `invalid_margin::invalid` contradiction. No declaration makes an unreadable margin readable.

PR 4475's rule keeps a poisoned margin's **decision's own** lever. The coincidence menu's declaration arm is that lever only where a coincidence is declarable.

The readers that still render the bare `Display` on a margin that may be poisoned include:
- `geom_brep::SectionError::Escalated` (`intersect.rs`);
- the blend and sweep doors that print `{source}`.

`ChartRegionError::Escalated` no longer reaches this text on a poisoned margin: since PR 4497 (ENCL row `topo-poisoned-escalations-offer-unfollowable-endings`) that arm ends in the defect ending, and only a margin that was read renders the bare `Display`.

## Repair shape

Decide at the geom-core home what the bare `Display` ends a poisoned margin in. Either it ends in the move arm alone with the note, or the readers that cannot declare compose `Indeterminate::undecided` with their own ending, as `SplitReduceError::CrossingEscalated` does. Then sweep the `{diag}` / `{source}` readers.
