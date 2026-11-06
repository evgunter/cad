---
id: main-red-m10-sym-profiles-two-pinned-rows
kind: issue
title: main red: m10_sym_profile's two pinned rows are red on main at every eps (walk ledger)
status: dispatched
opened: 2026-11-05
priority: P2
cost: E
refs: [pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger]
branch: onecopy/main-reds-k-and-m10
---


Found by `onecopy/polish-meters` (PR 6537). Head's walk ledger is byte-identical to base `b9cc0096cb`'s at all three ε, so this is main's red, not the PR's. No open row covered it.

## What

`m10_sym_profile`'s two pinned rows are red on main. Its walk ledger's Decision forms no longer match the pin. An earlier instance was `pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger` (PR 3612 → 3652).

## What closing it takes

1. Bisect to the commit that moved the ledger.
2. Re-pin it with the reason, or restore it.
3. Say why the move reached main red. It is likely the slow set again: these rows are outside the per-PR gate.
