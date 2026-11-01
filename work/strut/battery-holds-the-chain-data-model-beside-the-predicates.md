---
id: battery-holds-the-chain-data-model-beside-the-predicates
kind: issue
title: battery.rs holds the chain data model, the walk and a recourse audit beside the predicates its header promises
status: open
opened: 2026-09-13
priority: P1
cost: D
---


## The shape (BLEND unit 7's fix pass, both reviewers)

`crates/sweep/src/blend/battery.rs` promises, in its header, the
battery's predicates. What it holds besides them: the chain data
model — five public types (`Link`, `Chain`, `Junction`,
`ChainClosure`, `BatteryVerdict`) — the ~150-line `walk_chains` with
its junction record and remap, and the recourse audit
(`CornerConfig`'s run-out table and its readers). Unit 7 grew the
model by one type and the walk by its record; neither is a predicate.

## Fix shape

A `blend/chain.rs` (or `battery/chain.rs`) for the model and the walk,
`battery.rs` keeping the predicates and `run_battery`; the recourse
audit's home is the orchestrator's call (it reads `CornerConfig`,
which lives in `mod.rs`). `test_support::walked_chains` is the one
crate-private seam a suite reaches the walk through and moves with it.
