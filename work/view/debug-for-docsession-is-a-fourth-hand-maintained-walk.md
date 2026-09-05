---
id: debug-for-docsession-is-a-fourth-hand-maintained-walk
kind: issue
title: Debug for DocSession lists fields by hand and is non-exhaustive, so a field added to Derived is silently absent
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: open
opened: 2026-09-05
---


Found by the #1885 style review (S3), one screen below the walk that
PR collapsed. Pre-existing.

## The duplication

`impl Debug for DocSession` (`crates/viewer/src/session.rs:1663-1674`)
names its fields by hand — `generation`, `landed_generation`,
`selection`, `hover`, `states`, `gesture`, `path` — and ends in
`finish_non_exhaustive()`. So a field added to `Derived` (the value
`Open` and `NewDocument` now reset by construction) is silently absent
from every debug rendering, exactly the way it used to be silently
absent from a clearing walk: nothing red, nothing missing at compile
time, and the omission is only visible to someone who reads a dump and
wonders what is not in it.

Neither `Derived` nor `LandedRun` derives `Debug`, which is what keeps
this impl hand-written. `Selection`, `Hovered`, `Generation` and
`PathBuf` all implement it already; the blockers are the `Box<dyn
EvalService>` and the document values, which is what
`finish_non_exhaustive` is standing in for.

## What resolving it looks like

Derive `Debug` on `Derived` and `LandedRun` (or write one impl for
each, once) and let `DocSession`'s render name the block rather than
its members, so the members travel with the declaration. Whether the
outer impl stays non-exhaustive is a separate question: it is honest
about `eval` and the documents, and it should stay non-exhaustive for
those and no longer for the fields a value now carries.

**Sweep before calling it fixed.** `impl Debug for DocSession` is the
only hand-written `Debug` under `crates/viewer/src/` today (grep
`impl.*Debug for`, one hit; `finish_non_exhaustive`, one hit), so the
sweep is currently trivial — but the pattern that matters is a
hand-listed field census of any kind, not the `Debug` trait, and that
grep does not find `Display` impls, serialisers, or panel inventories
that enumerate the same fields. Re-run both greps at the fix.
