---
id: the-banded-pivot-refusal-has-no-straddling-row-at-interval
kind: issue
title: No row drives the banded collocation solve's pivot refusal with a pivot whose bracket straddles zero, so a regression of structure_nonzero at Interval stays green
status: closed
opened: 2026-10-16
priority: P4
cost: E
refs: [the-skin-at-interval-encloses-the-nominal-skin]
closed: 2026-10-17
---


Raised in PR 5233's review (unilateral MINOR, by inspection). Not taken
there.

`geom/src/curves/banded.rs`'s `factor` refuses a pivot that is not
nonzero for every value it stands for (`geom_core::spline::structure_nonzero`).
At a point scalar the existing banded rows pin that arm. No row drives
it at `Interval` with a pivot whose bracket straddles zero, so
reverting the check to a value-channel `== 0.0` read would likely stay
green.

The skin only reaches the solve over a single v-span (wide parameters
with `k ≤ v_degree + 1`), where the Bernstein collocation matrix is
totally positive, so a straddling pivot is hard to build through
`sweep`. A row at `geom`'s `interpolate_columns_in` over hand-made wide
parameters close enough to collide would do it, and should refuse
`FitError::Lsq(LsqDegenerate)`.

## Closed

There is now a row at `banded::factor` itself, `a_pivot_straddling_zero_refuses_at_interval`:
- a 2×2 system whose second pivot is `[-1, 1]` refuses at pivot 1, with a NaN payload;
- the same system with the pivot `[0.5, 2]` factors.

Reading the pivot as `value_channel() == 0` would let the straddling one through.

Through `interpolate_columns_in` the arm is not reached. `check_ordered` refuses colliding parameters first, and a Bézier span's collocation pivots stay certainly positive.
