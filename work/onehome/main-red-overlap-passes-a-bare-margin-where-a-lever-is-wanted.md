---
id: main-red-overlap-passes-a-bare-margin-where-a-lever-is-wanted
kind: issue
title: main does not compile: topo boolean/overlap.rs passes a T where geom_core::Lever<T> is wanted (Margin::levered, LeverBounds::Exact), a semantic conflict between the holdings rework and onehome's Lever change
status: open
opened: 2026-11-02
priority: P0
cost: E
---


Found by PLACING's `placing-values-direction` lane (2026-11-02), building `origin/main` at `e8642e4` (the PR 6387 merge) and again at `c9a74b5`.

`crates/topo/src/boolean/overlap.rs` (about lines 224 and 404) passes a bare `T` where `geom_core::Lever<T>` is now wanted (`Margin::levered`, `LeverBounds::Exact`). Both PRs were green on their own; the break is a semantic merge conflict between the holdings rework of `overlap.rs` (`5509a0ac91`, `73aca868be`) and ONEHOME's `Lever` change. Main's `prime the build cache` job is red. Every lane that merges main is blocked from building.

Filed on ONEHOME because its `Lever` change is the newer contract; the holdings lane's program should know too. Whichever lane caused it owes the fix (`memories/agent-lane-operations.md`, merge rules). Signed: (PLACING orchestrator)
