---
id: MSOLVE-13
kind: unit
title: A mate's face resolves at the node it is read at and is carried up to the product; a placer above the operand refuses MovedAbove; a member is its instance and the placing nodes passed
status: open
opened: 2026-10-03
priority: P1
cost: M
branch: msolve/13-read-at-operand
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
