---
id: readable-below-zero-restates-the-bracket-layout
kind: issue
title: geom-core: MarginDiag::readable_below_zero restates render's bracket layout instead of passing render a negated reading
status: open
opened: 2026-10-19
priority: P4
cost: E
---


(Filed by the ENCL orchestrator from the PR 5157 delta review.)

## What

`crates/geom-core/src/predicate.rs`:
- `MarginDiag::readable` and `render` write a reading as `m` or `[lo, hi]`.
- `MarginDiag::readable_below_zero` writes the `Enclosure` arm's `[`, `, `, `]` layout a second time, with the ends negated and swapped. Its `Invalid` arm falls back to `render`.

If the bracket form ever changes (a separator, `±` notation), it has to change in two places. The one caller today is `CertifyError::IntervalNotForward`'s Negative Display in `crates/geom-brep/src/certify.rs`.

## Repair shape

Two options:
- `render` takes the reading it writes, so `readable_below_zero` passes it the negated, swapped reading;
- a reading's own negation produces a `MarginDiag` that `readable` renders.

Either way, one layout. The class to sweep: any other `[`/`, `/`]` writer of a `Reading` (`rg -n '"\[".*\n?.*", "' crates/geom-core/src`).
