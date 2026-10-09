---
id: sized-poisoned-ending-ignores-the-reading-and-the-file
kind: issue
title: geom-core: a sized decision's poisoned-margin ending says 'kernel bug' at every door and keeps a lever a NaN cannot follow
status: open
opened: 2026-10-09
priority: P3
cost: M
---



(Filed by the ENCL orchestrator from the review of PR 4461. Pre-existing.)

## What

A sized decision on a poisoned margin (`MarginDiag::INVALID`, NaN) ends through `MarginDiag::sized_recourse`'s unreadable arm (`crates/geom-core/src/predicate.rs` ~1187). That ending is the decision's lever plus `UNREADABLE_MARGIN_NOTE`, "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Two problems:

1. **The ending ignores the reading.** At rest or at the import door, a body may have come from a file, and the unread margin may then mean a damaged file. The note still says only "kernel bug". `Unsized` ends a poisoned arm in `defect_ending(reading)` instead, since PR 4453, which names "a kernel defect or a damaged file" at rest.
2. **The lever may not be followable on a NaN.** For example, `SHELL_ROLE`'s "thicken or remove the degenerate geometry": "remove" can be followed, "thicken" cannot. PR 4461's sort now carries this ending for a poisoned shell role.

## Repair shape

Make the unreadable arm of the sized ending reading-aware, so it names the file at rest and at the import door as `defect_ending` does. Decide whether a poisoned margin keeps the lever at all. Give the poison→note rule one home: today it is decided at `not_yet`, `LeverOnly::recourse`, validate `unnamed` and `MarginDiag::sized_recourse` (see `too-close-to-call-remainder`'s bullet (c)). That likely folds both rows' (c) into one unit. Texts move only on poisoned sized arms.
