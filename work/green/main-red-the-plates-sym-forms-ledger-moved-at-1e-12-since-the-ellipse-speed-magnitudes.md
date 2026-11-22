---
id: main-red-the-plates-sym-forms-ledger-moved-at-1e-12-since-the-ellipse-speed-magnitudes
kind: issue
title: Main red (ε = 1e-12): editor-core m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned reads a moved ledger since PR 7183 (germ/ellipse-speed-magnitudes)
status: open
opened: 2026-11-21
priority: P1
---


Found by ONEROUND's lane (2026-11-21) on main `43080b1f`: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core --all-features --ignore-default-filter -E 'test(the_forms_the_plates_walks_build_are_pinned)'`.

## What

    plate at eps 1e-12: the walks built different forms (counts SymCounts { symbolic: 979, …, checks_expanded: 1047, checks_frozen: 0, numeric: 1047, retried: 0, frozen: 54 })
      left:  Plain/Decision calls 979 forms 39705 frozen 54 digest 3c078f72…   Plain/Assertion calls 1047 forms 14594 …   Early/Assertion calls 1047 forms 15164 …   Door/Decision calls 550 forms 15364 …
      right: Plain/Decision calls 979 forms 39703 frozen 54 digest 7efda4b6…   Plain/Assertion calls 1047 forms 14592 …   Early/Assertion calls 1047 forms 15162 …   Door/Decision calls 550 forms 15356 …

The call counts and the frozen counts are the pin's; the form counts are up by 2, 2, 2 and 8 and four of the five digests moved (`Early/Decision` is unchanged).

## The merge

`6d58cebd`, PR 7183 (`germ/ellipse-speed-magnitudes`): the row passes on its first parent `48d98d55` and fails on the merge, and with this same ledger on `d3012f9f`, `ca6dd709`, `0ad2e0b5`, `02804869` and `43080b1f`. It passes on `1360b9eb`, `46e89240` and `f2c38057`. So neither PR 7146 (`ca6dd709`) nor PR 7143 (`02804869`) moved it: the ledger is identical on each side of both.

## Owed (GERM, the PR's program)

Whether the plate's walks owe the extra forms (then the 1e-12 pin moves, with the reason) or the PR builds forms it should not. Not read here.

## Measured (GREEN U3, `green/editor-1e-12`, 2026-11-21)

The scheduled nightly 44870651337 (main `f5ef8f2b`) has this row red at every ε, not only 1e-12; at the default ε it is SCALAR's `main-red-the-plates-sym-walk-ledger-reads-2-to-8-more-forms` (P2), with the same five lines this row quotes. One pin serves every ε (`the_forms_the_plates_walks_build_are_pinned` reads no ε), so the two rows are one move. PR 7131 (`onehome/carrier-frames`, merged `9bc9d651` 11-21) re-pinned `PLATE_LEDGER` after that nightly, writing its move from 39703 (the old pin), not from the 39705 main then read.
