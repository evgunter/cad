---
id: predicate-dimension-audit-doc-forces-the-whole-suite
kind: issue
title: an edit to docs/predicate-dimension-audit.md forces TIER=all (22 of 257 merges in a week)
status: open
opened: 2026-10-16
priority: P3
cost: E
---


## Measured (2026-10-16, CIW orchestrator)

In the same 7-day replay as `a-slow-set-edit-to-nextest-toml-runs-the-whole-suite`, 22 merges reached TIER=all only because they edited `docs/predicate-dimension-audit.md`. A single `docs/guide/north-star-audit.md` edit did the same.

## Wanted

Find what reads the audit doc: ci-filter's markdown-read-by-tests logic classifies it workspace-level. Scope it to the crates or suites that actually read it. If one census reads it, the doc's edits seed that census's crate.
