---
id: a-mate-is-poisoned-through-a-member-its-own-solve-faulted
kind: issue
title: A mate reading a member whose offset check its own solve faulted is poisoned through it, losing its role and solve log
status: open
priority: P3
cost: M
opened: 2026-10-10
---

Found by review R2 of INTENT stage 2 F (PR 4566, minor m3), and the orchestrator's accepted call 2 in the same PR.

## What

A mate reads its members (D10), and a reader of a failed variable is poisoned through it. When the solve faults a member's offset check (`MateFault::OffsetUnchecked`) or another member fault, the mate that placed or checked that member is `Poisoned { through: member }` (`eval/mod.rs`, the poison loop after the mate's own-fault check). The cause is poisoned by its effect: the mate's role and its solve log (escalations) are lost, though the solve recorded both. `crates/editor-core/tests/msolve14_run_scalar.rs`'s C5 (`gauge-chain` at ε 1e-12) accepts a mate poisoned through the escalated member in place of an escalation on its log; A2's row expects the poison.

## What it wants

A ruling: either a mate's value stands on the solve's answer, so a member it faulted does not poison it (its key then needs a word for the failed member's absent key), or the poisoning stays and the log moves to the member's fault. Nothing wrong is served today; diagnostics are lost.
