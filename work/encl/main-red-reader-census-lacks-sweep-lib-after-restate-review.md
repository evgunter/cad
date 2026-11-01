---
id: main-red-reader-census-lacks-sweep-lib-after-restate-review
kind: issue
title: Main red: reader_census's ledger lacks crates/sweep/src/lib.rs since the sweep-restate review
status: closed
closed: 2026-10-21
pr: 5533
branch: encl/main-red-restate
opened: 2026-10-20
---


**Main red.** Filed by the NURBS orchestrator from NURBS build C's full-workspace run (15,222 tests on main `43b23583` plus a geom-only change; red on clean main too).

`test-utils` `reader_census::every_site_that_reads_rust_source_is_in_the_ledger` fails: the ledger lacks `crates/sweep/src/lib.rs`.

The most recent change to that file is `f1cfa9146b` ("sweep-restate review: loft's # Panics names only the door it reaches; module docs name restate_stored_edge; file the stored-curve read ladder"), the follow-up to ENCL's PR 5418. **The attribution is by file history and not bisected**; ENCL should confirm. If another program's change is the cause, re-home this item.

**Done when** the row is green on main: `sweep/src/lib.rs` is added to the ledger with its disposition, or the source read it gained is removed.

## Closed

2026-10-21. A duplicate of `main-red-sweeps-restatement-census-owes-a-reader-ledger-line`, closed by the same fix in PR 5533: the ledger gained a `Shared` line for `crates/sweep/src/lib.rs`. The attribution is confirmed. The source read is `302b6705e8`'s `restatement_census` module from ENCL's PR 5418; `f1cfa9146b` only touched the file afterwards.
