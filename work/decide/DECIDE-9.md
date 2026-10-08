---
id: DECIDE-9
kind: unit
title: the decision read answers theorems: keep it behind every form that settles
status: closed
opened: 2026-10-02
priority: P2
cost: M
branch: decide/9-read-behind-theorems
refs: [the-decision-read-answers-theorems-the-must-carry-stations-would-prove]
closed: 2026-10-02
---

## What

`the-decision-read-answers-theorems-the-must-carry-stations-would-prove`:
with the read on, 32 of the pad's and 16 of the bracket's decisions
count `sign_gated`, although they are theorems with the read shut. That
breaks the read's own contract, which orders it behind every value-free
fold.

Phase 1 attributes them by predicate and by node, and confirms or
replaces the suspected mechanism: a node-level read ahead of a
cancellation above it. Phase 2 restores the theorems without moving any
decision's value.

Spec: `docs/DECIDE-9-SPEC.md`, deleted at close (`docs/doc-ledger/decide-9-spec.md`). Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-10-02).

## Closed (2026-10-02, PR #3807)

Merged after a single FULL review (APPROVE-WITH-FIXES, no MAJOR), a fix
pass and a delta review (MERGEABLE). The pad reads 925 and the bracket
1121 `symbolic_zero`, as with the read shut; no value and no `numeric`,
`registered` or `frozen` count moved. The mechanism was replaced: a zero
product carried its non-zero factor's gate. Successors:
`the-read-at-its-node-relabels-a-cancellation-above-it` (P2) and
`form-mul-carries-the-gate-of-a-factor-a-zero-annihilates` (P3).
