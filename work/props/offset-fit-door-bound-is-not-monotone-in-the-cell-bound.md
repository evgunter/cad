---
id: offset-fit-door-bound-is-not-monotone-in-the-cell-bound
kind: issue
title: offset_fit: tightening a cell bound can raise the door's hull_sup, because the refinement marking reads the sup
status: open
opened: 2026-09-12
---


## The finding

`Composite::cell_bound` got strictly tighter for every cell of every
grid (PROPS mignitude-floor: the floor on `‖E‖` became a max over the
reading it had). On a FIXED grid that is monotone — the in-module row
`the_sign_witness_floors_norm_e_where_the_components_straddle_zero`
measures it cell by cell on two grids and no cell rises.

At the DOOR it is not monotone. Over `budget_faces`' 2x7x5 corpus, run
before and after the change, one cell got worse:

| request | before | after |
| --- | --- | --- |
| bumpy, `d = 1e-5`, `tol = 1e-6` | `8.189454e-10` on 780 cells | `8.337390e-10` on 810 cells |
| bumpy, `d = 1e-5`, `tol = 1e-9` | `8.189454e-10` on 780 cells | `8.337390e-10` on 810 cells |

(the other 23 cells that moved all improved, four of them by changing
face; 46 did not move).

## Why

`measure` marks for refinement every cell within a factor of two of
the round's sup (`let cut = hull_sup * 0.5`), so the marking reads the
sup — a RELATIVE test. Tightening cells by different factors reorders
which cells clear `cut`, so a strictly better per-cell bound can
produce a different schedule, a different interpolation, and a
slightly worse sup one round later. Here the new schedule is the finer
one (810 cells against 780) and still lands 1.8% higher.

Nothing certified is weakened: both numbers are certified sup bounds on
the same claim, both are three orders below the tolerance they were
asked for, and the request certifies either way. What the shape costs
is a reasoning aid — "the bound only goes down" is not available as an
argument to a future lane, and a re-baseline table cannot be read as a
per-row monotonicity proof.

## What would settle it

Either a marking rule that is stable under a uniform tightening (mark
on an absolute rung, or on rank), or a ratified note at `measure` that
the door's bound is not monotone in the cell bound and why that is
acceptable. This item is the disclosure, not the choice.
