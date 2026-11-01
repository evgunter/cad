---
id: bits-witness-slice-workaround-outlived-its-defect
kind: issue
title: bits_witness takes a slice only to dodge a gate defect that PR 2030 fixed, and its comment cites a work/issues path that no longer exists
status: closed
opened: 2026-09-06
priority: P4
cost: E
closed: 2026-10-26
---


## Finding

Filed by the GATES orchestrator from PR 2030 (the `scripts/gates/bit-identity-debug-only.sh`
lane) and its style review. `crates/topo/src/source.rs:205-213`'s
`bits_witness` takes `pairs: &[(T, T)]` because the gate once ended a
`#[cfg(debug_assertions)]` item at the `;` inside an array-typed
signature (`[(T, T); N]`) and reported the fold ungated (DOCM-2, PR
1860; worked around at `b59b2203`). PR 2030 fixes the reader, so the
slice is now a workaround for a defect that no longer exists, and the
comment beside it cites `work/issues/bit-identity-debug-only-gate-ends-an-item-at-a-semicolon`,
a path that has moved twice (it is `work/gates/…` and closes with PR
2030). TOPO's file; whether the signature returns to the array form or
stays a slice on its own merits is TOPO's call, and the comment is
stale either way. Sequenced after PR 2030 lands.

## Closed (2026-10-26, tracker sweep)

Superseded: the subject is gone from the tree. On main (`bb25fa4f0b`) there is no `bits_witness` anywhere under `crates/`, and `crates/topo/src/source.rs`, the file the row cites, no longer exists; the deletion predates this checkout's shallow history, so no commit is cited. The stale `work/issues/bit-identity-debug-only-gate-ends-an-item-at-a-semicolon` citation went with it: no file under `crates/` cites that path.
