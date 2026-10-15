---
id: rebuild-latency-table-redeclares-the-heatsinks-fins
kind: issue
title: m4_pr8_latency::rebuild_latency_table panics with VarNameTaken(fins), so the nightly rebuild-latency job has emitted nothing since 10-04
status: closed
opened: 2026-10-12
closed: 2026-10-15
priority: P3
cost: E
---


(Filed by the ENCL orchestrator from a nightly triage, 2026-10-12.)

Nightly job `rebuild latency (reporting)`, red since 10-04, panics at `crates/editor-core/tests/m4_pr8_latency.rs` `heatsink_at`. Repro:

`cargo test -p editor-core --test all -- --ignored m4_pr8_latency::rebuild_latency_table`

The panic comes from `DocEdit::DeclareVar { name: "fins" }` followed by `.expect("the fin count is a document parameter")`. The error is `VarNameTaken { name: VarName("fins"), holder: SpokenVar { .. } }`: the corpus heat sink already declares `fins`, so the test should set the existing variable rather than declare it again. Reproduces on origin/main bcaed4f75a. As a result `docs/perf-data/rebuild-latency/` receives no entries.

## 2026-10-15 — moved from TINT to CIW (CIW orchestrator, at Ev's direction)

TINT has no orchestrator. This red sits in a nightly job that CIW owns, and its fix is the test's own (a stale premise in the test, not a kernel defect), so CIW takes it. If the fix turns out to need kernel code, it is routed to the crate's owner.

## Closed

`heatsink_at` writes the fin count into the heat sink's own `fins` with `DocEdit::SetVarValue` (the value door: it keeps the variable's definition and refuses an unknown name), instead of declaring a second `fins`. Sweep for the same shape (a test declaring a name a corpus fixture already declares: `fins`, `h`, `n`, `hole_r`) found no other hit; every other declarer of those names builds on an empty document. The PR records the hit list and the local nightly-shaped run. (CIW lane)
