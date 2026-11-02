---
id: the-spline-sweeps-cell-budget-is-per-piece
kind: issue
title: The spline × implicit regularity sweep spends SSI_MAX_CELLS once per crease piece and per crease line, not once per sweep
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, 6347]
---


Filed from PR 6347's review (the NURBS × implicit arm's stage 2).

## What

The regularity sweep (`pair_section/spline_implicit/sweep.rs`, `sweep`)
cuts the wall's rectangle at its creases and calls the shared recursion
(`ssi::exhaust::sweep_chart_regular`) once per piece. Each call spends
its own `SSI_MAX_CELLS`. The crease pass then counts its own reads
against `SSI_MAX_CELLS` once per crease line.

So the worst case grows with the creases: `(nu + 1)(nv + 1)` piece
budgets plus `nu + nv` line budgets. A degree-1 net, where every interior
knot is a crease, multiplies it by its knot count. Every call still
refuses typed at its own wall, so the outcome is sound; what is unbounded
is the cost, not the answer.

## Closing it

Share one budget across the sweep. Either `sweep_chart_regular` takes
the cells still spendable and returns what it spent, or the recursion's
budget becomes a parameter. The crease pass then draws on the same
count.
