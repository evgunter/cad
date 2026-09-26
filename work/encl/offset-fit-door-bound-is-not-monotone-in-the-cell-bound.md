---
id: offset-fit-door-bound-is-not-monotone-in-the-cell-bound
kind: issue
title: offset_fit: tightening a cell bound can raise the door's hull_sup, because the refinement marking reads the sup
status: closed
opened: 2026-09-12
priority: P4
cost: E
branch: encl/offset-fit-loop-faces
rides_with: offset-fit-budget-face-speaks-for-a-round-whose-bound-rose
pr: 3294
closed: 2026-09-26
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

## The remedies a reader would reach for are not available

The 1.8% is not the bound moving on a grid. On the 810-cell grid the
request now lands on, the witness reading differs from the
componentwise one by at most a factor `1.0000304` on ANY cell
(`no_cell_rises_on_the_bumpy_grids_whose_door_bound_grew`), which is
four orders below the 1.8%. The review lane measured the other side
of the same fact and it is the decisive one: the OLD 780-cell grid
reads `8.189454e-10` under the witness bound too — the identical
digits it read under the componentwise one — so the whole difference
is the two runs walking different grids, parting at round 1 (196
cells against 224).

That rules out the two fixes the shape invites:

- **A marking rule stable under a uniform tightening.** The current
  `cut = hull_sup * 0.5` is already invariant under one — the scale
  cancels, so a uniform factor reorders nothing. An ABSOLUTE rung is
  strictly worse here: it is not invariant, and it would move the
  schedule on every re-baseline of any cell bound.
- **A marking rule that makes the door bound monotone.** No marking
  rule can: the schedule chooses which parameters the fit interpolates
  at, so a different schedule is a different fitted surface, and the
  bound on a different surface is not comparable with the bound on
  this one in either direction. Monotonicity at the door would need
  the fit to be fixed, and it is exactly what refinement moves.

What is left is the disclosure — that "the bound only goes down" is
not available as an argument about the DOOR, only about a cell on a
fixed grid — and, if a future lane wants it written where a reader
of `measure` will meet it, a ratified note there. This item is that
disclosure, not the choice.
