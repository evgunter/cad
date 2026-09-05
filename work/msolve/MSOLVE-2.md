---
id: MSOLVE-2
kind: unit
title: The member chain: nested patterns and sibling distinctness at every level
status: parked
opened: 2026-09-05
blocked_on: [MSOLVE-1]
refs: [nested-pattern-mate-heads-refuse]
---


## Waits on

`MSOLVE-1`, which gives `Member` its operand and the walk its single
pattern level. This unit generalizes `Member.copy` to the chain of
structural indices the walk consumes (sibling distinctness at every
level, the loop-closure argument holding at each), loses `Copy` if it
must, re-keys `by_pair` and `edge_of`, and owes the loop-closure rows
for a nested member that nothing in the suite has. Ruled in by Ev on
PR 1731 (`nested-pattern-mate-heads-refuse`, parked on this unit).
Spec is written when MSOLVE-1 merges, against the walk as landed.
