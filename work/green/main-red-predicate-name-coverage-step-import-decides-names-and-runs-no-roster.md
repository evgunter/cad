---
id: main-red-predicate-name-coverage-step-import-decides-names-and-runs-no-roster
kind: issue
title: Main red (slow set): predicate_name_coverage finds step-import deciding predicate names with no roster, since PR 7091 (the interp layer)
status: closed
opened: 2026-11-21
priority: P1
closed: 2026-11-22
pr: 7371
---


Found by ONEROUND's lane (2026-11-21) on main `71068072`, `cargo nextest run -p test-utils --all-features --ignore-default-filter -E 'test(predicate_name_coverage)'` (slow set, about a minute).

## What

`test-utils::all predicate_name_coverage::every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided`:

    `step-import` decides predicate names and runs no roster — give it `tests/recourse_roster.rs` on `test_utils::source::roster_violations`, or list it in KNOWN_MISSING with the row that will

`KNOWN_MISSING` is empty and `crates/step-import/tests/` has no `recourse_roster.rs`.

## The merge

A first-parent bisect from `b7e8f5c4` (2026-11-17, passes) to `b2773f6b` (fails): last good `b8f6c9d2` (PR 7111), first bad `1490c99c`, PR 7091 (`importgate/interp-layer`). Red on every main tested since (`800e4d35`, `c0c5f89d`, `1360b9eb`, `46e89240`, `4d2d9942`, `bd8b2c9b`, `71068072`), so it predates every ONEROUND merge of 2026-11-20. What in PR 7091 made the census read step-import as a deciding crate was not read.

## Owed (IMPORTGATE, the PR's program)

A roster for step-import on `test_utils::source::roster_violations`, or a `KNOWN_MISSING` line citing the row that will add it, as the message says. The class is `work/green/a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster.md`.

## 2026-11-21: greened by a KNOWN_MISSING line in GREEN's PR (branch `green/small-reds`)

**Why the census sees step-import.** PR 7091 gave the crate its first decide site: `FileBand::read` in `interpret.rs` calls `k_stats::decide(name, …)` on its caller's name. On the merge's first parent, `crates/step-import/src` has no `decide*(` call. Const-named sites came after it (`build.rs`, `snap.rs`).

**Why there is no roster.** The names reach `decide` through `FileBand::read` and its forwarders. Most of those are spelled with tokens that other calls share (`read`, `on`, the closure `zero`), next to refusal builders that take a reading's name as a label. A roster on the shared harness has to rename those funnels to `*_as` spellings and decide which of the builders are carriers. That is work in IMPORTGATE's band layer, so it is filed there: `work/importgate/step-import-runs-no-recourse-roster-over-its-band-funnels.md`. `KNOWN_MISSING` in `crates/test-utils/tests/predicate_name_coverage.rs` cites it, and the ratchet fails the coverage row once the roster runs.
