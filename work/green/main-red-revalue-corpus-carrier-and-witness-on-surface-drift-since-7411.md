---
id: main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411
kind: issue
title: Main red: ten revalue_corpus rows fail since PR 7411 (germ/plane-pose-far-frame): the re-valued DAG drifts from Sym's build on carrier_on_surface_{1,2} and witness_on_surface_{1,2}
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found by DOCTAIL's 7396 lane (2026-11-23) while running `editor-core`'s `revalue_corpus` locally. Filed by the DOCTAIL orchestrator. It was not reproduced by CI, because the push CI on main is queued.

## What

Ten `editor-core::all` `revalue_corpus::*` rows fail on origin/main, identically at ε 1e-6, 1e-9 and 1e-12. A re-valued DAG drifts from `Sym`'s build on `carrier_on_surface_{1,2}` / `witness_on_surface_{1,2}`. One of the ten is `revalue_a_chart_on_a_cap_tilted_at_zero` (from DOCTAIL's PR 7434); it is red for the same reason, not its own.

**Bisected on a clean main:** green at `1883da3` (#7437), red at `3547fe5`, which is **PR 7411, `germ/plane-pose-far-frame`** ("germ: the plane pose's offset is posed near its geometry; the ring path reads cross uncharged").

## Owed

The lane that caused the red owes the fix (`memories/agent-lane-operations.md`): GERM's, or whoever holds PR 7411's follow-up. Either the re-valuation reads the plane pose's new near-geometry offset the way `Sym`'s build does, or the build and the revalue instrument agree on one spelling.

Reproduce: `cargo nextest run -p editor-core --test all -E 'test(revalue_corpus::)'` on main.
