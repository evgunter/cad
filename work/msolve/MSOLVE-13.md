---
id: MSOLVE-13
kind: unit
title: A mate's face resolves at the node it is read at and is carried up to the product; a placer above the operand refuses MovedAbove; a member is its instance and the placing nodes passed
status: closed
opened: 2026-10-03
priority: P1
cost: M
branch: msolve/13-read-at-operand
pr: 3969
closed: 2026-10-03
---


Spec: `docs/MSOLVE-13-SPEC.md`. Plan item 26: Ev ruled the defect on
`[ev]` PR 3695. Two designers converged in one round:
- the gate resolves a mate's face at its operand and carries it up to
  the product;
- a placer above the operand refuses `MovedAbove`;
- the walk descends a union;
- a member is its instance plus the placing nodes passed.

The two sentences that move (A5's, and A11 (5)'s member identity) are
agent text from MSOLVE-5 and MSOLVE-2, with no Ev ratification found.
Ev is told, not asked. Review tier: single, full. Waits for the disk
that MSOLVE-12's lane holds.

## Closed

Merged on PR 3969 (2026-10-03). Single full review on `3610f0432`:
APPROVE-WITH-FIXES, no MAJOR, and C1–C5 held. `msolve1::a5`'s moved
verdict was ruled right: its old pass was a fixture coincidence. The
fix pass R1–R10 is green at `9d63f6642`, and the orchestrator
spot-checked the diff against the rulings. The rows are in the PR
body.

