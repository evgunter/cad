---
id: the-clearance-budget-pair-is-spelled-three-times
kind: issue
title: "{max_cell_depth, max_cell_pairs} is spelled three times: ClearanceConfig, the Sweep's fields and MinSeparationConfig"
status: closed
opened: 2026-10-28
priority: P1
cost: E
refs: [SHELL-3]
branch: clear/loops
pr: 5976
closed: 2026-10-29
---


## Finding

From the full review of PR #5909 (SHELL-3, the move into
`crates/topo/src/clearance.rs`), style lane Q1, reported as likely.
The shape predates the move, which put it in one file. The review's
report is summarised in `work/clear/log.md` (2026-10-28).
Filed by the CLEAR orchestrator.

## Closed
LOOPS (#5976) gives one walk, `walk` in `crates/topo/src/clearance.rs`,
that both questions run through a `Question` trait, and one budget pair,
`Budgets`, spelled once. A differential against the merge-base bodies
proved there was no behaviour change before it was reverted. A single
FULL review re-proved it (3,663 budget points per scope, both doors).
The review's MINOR (the `CellReceipt` type doc) and its Q5 and Q7 points
are fixed in the closing commit. Its Q1 points are filed as
`the-bound-sweeps-root-phase-is-a-second-walk-that-spells-abandoned-again`.
