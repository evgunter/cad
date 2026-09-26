---
id: offset-fit-budget-face-speaks-for-a-round-whose-bound-rose
kind: issue
title: offset_fit: BudgetExhausted says the bound is still converging on a round whose bound rose, and carries the last round's bound rather than the best one reached
status: dispatched
opened: 2026-09-25
priority: P3
cost: E
branch: encl/offset-fit-loop-faces
---


Found by `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`'s
measurement (its PR carries the full trace).

## Measured

`fit_offset_at(bowed, 0.05, 1e-12)` (`crates/geom-brep/tests/approx_surface.rs`'s
`bowed()`), per round, today's main:

| round | grid | `hull_sup` | sampled on-locus max |
|---|---|---|---|
| 5 | 17×17 | 6.88e-11 | 4.02e-11 |
| 6 | 27×27 | **1.78e-10** | 6.11e-12 |
| 7 (budget raised) | 32×32 | 2.49e-9 | 6.11e-12 |

The door refuses at round 6 with `BudgetExhausted { achieved: 1.78e-10 }`
and the Display text "ran out ... with the bound still converging ...
the lever is OFFSET_FIT_BUDGET". Round 6's bound ROSE from round 5's.
The refusal is inside the variant's documented admission set
(`crates/geom-brep/src/offset_fit.rs`, `stall_verdict`: a first
non-improving round gets the both-directions fallback rather than a
verdict, and the budget test in `fit_offset_at` fires before that
fallback runs), but both halves of what it tells the caller are wrong
on this row:

1. **The lever.** Raising the budget by one gives `RefinementStalled`
   at round 7, not a certificate. The text names a knob that cannot
   help.
2. **The number.** `achieved` is the last round's bound. The best bound
   any round reached (6.88e-11 at round 5) is 2.6x smaller, and it is
   the number a caller sizes a tolerance against. `BoundNotFinite`
   already carries a `last_finite`; the finite faces carry no "best".
   `RefinementStalled` has the same shape (2.49e-9 reported against
   the 6.88e-11 reached).

## Why it happens here

The rise is the decomposition width described in
`work/props/f64-refinement-inside-an-enclosure-has-five-more-sites.md`
(its `insert_once_ring` row, where the lerp form is what makes the width
grow). If that is fixed, the loop's bound is monotone on this fixture
(measured: every round falls, and 1e-12 certifies at round 9), so this
row may stop reproducing on `bowed()`. It stays a defect of the
refusal's shape all the same: any non-monotone door bound
(`offset-fit-door-bound-is-not-monotone-in-the-cell-bound`) reaches the
budget face on a rising round, and the face then names the wrong lever
and the wrong number.

## What a fix looks like

Within the existing vocabulary: carry the best finite bound reached
(and its grid) on the budget and cap faces, and word the budget face
by what the last verdict actually was (still falling, or the
both-directions fallback not yet tried). No new variant is needed.

## `RefinementStalled` is reachable

Measured by the same lane; the fixtures and what reaching the face
means for `stall_verdict`'s reachability note are on the row that owns
that question, `offset-fit-stall-face-has-no-fixture`.
