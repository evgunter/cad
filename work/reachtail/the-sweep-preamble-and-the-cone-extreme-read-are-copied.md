---
id: the-sweep-preamble-and-the-cone-extreme-read-are-copied
kind: issue
title: sweep_split is a third copy of the gate → clone → sweep_and_settle preamble, and conic_clearance's cone arm re-spells first_harmonic_arm's extreme read
status: open
opened: 2026-10-06
priority: P4
cost: E
refs: [4135]
---


Left by the last fix pass of PR 4135 (the cone root lane), from its
first review (r1 Q1).

## What

- `boolean::sweep_split`
  (`crates/topo/src/boolean/mod.rs:3944`) is a third copy of the gate →
  clone → `sweep_and_settle` preamble, beside `sweep_traces_with_pad`
  (`mod.rs:3879`) and `sweep_records` (`mod.rs:3989`).
- `reduce::conic_clearance`'s cone arm
  (`crates/topo/src/boolean/reduce.rs:2655-2671`) re-spells
  `conic_quadric::first_harmonic_arm`'s extreme read
  (`crates/topo/src/boolean/conic_quadric/mod.rs:282`):
  `c₀ ∓ (A₁ + A₂ + noise)`.

## The fix owed

One preamble the three sweeps share, and one reading of a cone
harmonic's extremes that both the clearance rung and the door call.
