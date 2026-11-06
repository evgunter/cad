---
id: main-red-m10-sym-profiles-two-pinned-rows
kind: issue
title: main red: m10_sym_profile's two pinned rows are red on main at every eps (walk ledger)
status: closed
opened: 2026-11-05
priority: P2
cost: E
refs: [pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger]
branch: onecopy/main-reds-k-and-m10
closed: 2026-11-06
pr: 6560
---


Found by `onecopy/polish-meters` (PR 6537). Head's walk ledger is byte-identical to base `b9cc0096cb`'s at all three ε, so this is main's red, not the PR's. No open row covered it.

## What

`m10_sym_profile`'s two pinned rows are red on main. Its walk ledger's Decision forms no longer match the pin. An earlier instance was `pcurve-fit-domain-refusal-moves-the-m10-sym-walk-ledger` (PR 3612 → 3652).

## What closing it takes

1. Bisect to the commit that moved the ledger.
2. Re-pin it with the reason, or restore it.
3. Say why the move reached main red. It is likely the slow set again: these rows are outside the per-PR gate.

**Closed by PR 6560.** Bisected to PR 6504 (`143d99ec29`) and PR 6527 (`3fef8d7ade`), then PR 6562 (`5ff0801009`, +2 forms on three lines) while it was open; re-pinned at main's ledger (blocked residuals 66 → 94). The slab's ledger is now a fast ungated row; the movers are counted on `work/ciw/dependents-slow-sets-skip-a-kernel-pr.md` (Ev's revisit trigger) and the sentinel on `work/ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins.md`.
