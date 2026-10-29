---
id: the-bound-walk-and-min-separation-duplicate-one-frontier-loop
kind: issue
title: Sweep::run and min_separation duplicate the frontier, budget and split loop, and their overflow accounting diverges (refused vs abandoned)
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
refs: [SHELL-3]
branch: clear/loops
---


## Finding

From the full review of PR #5909 (SHELL-3, the move into
`crates/topo/src/clearance.rs`), style lane Q1, reported as likely.
The shape predates the move, which put it in one file. The review's
report is summarised in `work/clear/log.md` (2026-10-28).
Filed by the CLEAR orchestrator.
