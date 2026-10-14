---
id: rebuild-latency-table-redeclares-the-heatsinks-fins
kind: issue
title: m4_pr8_latency::rebuild_latency_table panics with VarNameTaken(fins), so the nightly rebuild-latency job has emitted nothing since 10-04
status: open
opened: 2026-10-12
priority: P4
cost: E
---


(Filed by the ENCL orchestrator from a nightly triage, 2026-10-12.)

Nightly job `rebuild latency (reporting)`, red since 10-04, panics at `crates/editor-core/tests/m4_pr8_latency.rs` `heatsink_at`. Repro:

`cargo test -p editor-core --test all -- --ignored m4_pr8_latency::rebuild_latency_table`

The panic comes from `DocEdit::DeclareVar { name: "fins" }` followed by `.expect("the fin count is a document parameter")`. The error is `VarNameTaken { name: VarName("fins"), holder: SpokenVar { .. } }`: the corpus heat sink already declares `fins`, so the test should set the existing variable rather than declare it again. Reproduces on origin/main bcaed4f75a. As a result `docs/perf-data/rebuild-latency/` receives no entries.
