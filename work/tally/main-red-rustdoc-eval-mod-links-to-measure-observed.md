---
id: main-red-rustdoc-eval-mod-links-to-measure-observed
kind: issue
title: main red: rustdoc rejects editor-core eval/mod.rs's link to measure::Observed
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`: `unresolved link to measure::Observed` at `crates/editor-core/src/eval/mod.rs:414` (`/// ([`measure::Observed`]). A variable no measure is under is that`).

It first appears in the 11-03 nightly (run 41583529337). None of BOXES' merges touches `crates/editor-core/src/`, so it is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)
