---
id: the-fits-enclosure-solve-reaches-only-narrow-parameters
kind: issue
title: The fit's enclosure solve reaches only narrow parameters: plain interval elimination widens fast and refuses past a few hundredths
status: open
opened: 2026-10-19
priority: P2
---


Found while building C6 S5c (PR 5332). The INLANE orchestrator ruled it to its own item at P2.

## Finding

Since S5c, `geom::curves::fit::interpolate_columns_in` solves over a wide averaged knot. The collocation rows enclose every member (`unit_row_in`), and the solve is `curves/banded.rs`'s Doolittle without pivoting at `Interval`, which is plain interval Gaussian elimination. It refuses typed (`LsqError::LsqDegenerate`) where a pivot is not certainly nonzero. That reach is short, and well before it ends the result is loose.

**Measured** at 283906cb93, with ONE wide parameter. The setup is five parameters `[0, 0.3, m, 0.7, 1]`, where `m` is `0.5 ± w/2`, and the data columns are `[k, 1]` for k = 0..4. Each cell is the widest solved control entry, or the refusal:

| width `w` | degree 1 | degree 2 | degree 3 |
|---|---|---|---|
| 0.005 | 0.15 | 0.50 | 1.03 |
| 0.01 | 0.31 | 1.02 | 2.11 |
| 0.02 | 0.64 | 2.10 | 4.74 |
| 0.03 | 1.00 | 3.36 | 10.1 |
| 0.04 | 1.38 | 5.30 | 28.0 |
| 0.05 | 1.80 | 9.73 | refuses (pivot 3) |
| 0.08 | 3.38 | refuses (pivot 3) | refuses (pivot 3) |
| 0.1 | 5.19 | refuses (pivot 2) | refuses (pivot 2) |

At degree 3 the widest entry is already about half the data's range at `w = 0.01`, and the solve refuses from `w = 0.05`.

S5c's review fix pass forms each span's piece at the parameter intersected with that span's knots' brackets (`SpanLocate::clip_to`; at a dual it keeps the parameter's own tangent). That tightens the rows. On the S5c row's data at `w = 0.01`, degree 3's widest entry drops from 2.53 to 1.95. With three wide parameters at half-width 0.0005, it drops from 2.30 to 0.95. The S5c row registers both. The tables above predate the narrowing. A `max`/`min` envelope narrowing, which the delta-read rejected, reached past the parameter to a knot's far end: it loosened the three-wide case to 2.44 and a dual's tangent fourfold.

**Several wide parameters reach less** (S5c's review, PR 5332):
- Three wide parameters, `[0, .2±h, .35, .5±h, .65, .8±h, 1]`, data up to 8:
  - degree 3: the widest entry is 2.3 at `h = 0.0005`, and the solve refuses from `h = 0.01`;
  - degree 2: refuses from `h = 0.04`, and from `h = 0.01` with the wide parameters adjacent.
- Ten points, reviewer A's widest half-width that still solves:

| setup | degree 1 | degree 2 | degree 3 |
|---|---|---|---|
| 10 points, one wide | past 0.04 | 0.02 | 0.01 |
| 10 points, all wide | 0.01 | 0.005 | 0.001 |

**Where the width comes from. Two places, both owed:**
- **The rows lose the parameter's correlation with the averaged knots it enters.**
  - At degree 1 each member's knot is its own parameter, so every member's matrix is the identity and the exact answer is the data. Yet at `[0.495, 0.505]` the held middle row is `[0, 0.051], [0.951, 1], [0, 0.051]`.
  - At degree 2 every member's parameter lies in one fixed span, yet the held parameter overlaps two.
  - The rows are clamped to `[0, 1]` and each span's piece is narrowed to its knots (`fit.rs`, `enclosure_piece`), but no preconditioner recovers what the rows have already lost.
- **The elimination's dependency problem.** It compounds through `L` and `U`, and nothing uses the fact that every member's matrix is totally positive and near the midpoint matrix.

**The refusal carries no margin, and names degeneracy.** Past the reach, the fit (and through it the loft) refuses `LsqError::LsqDegenerate { pivot: NaN }` (`curves/banded.rs`, the pivot check in `factor`):
- every member's matrix is regular, so "degenerate" misnames it;
- a refusal on which members may disagree should carry its margin (C6), so that a caller can bisect.

Before S5c the same loft refused `CarrierKnotNotAPoint`, naming the knot.

## Owed

**Rows that keep the correlation.** Two candidates:
- form each row from the members' own knots as functions of the parameters (the averages are linear in them), for instance as an affine form or a `Dual` slope over the parameters' boxes;
- or decide a member's span symbolically where the structure fixes it. At degrees 1 and 2 the span is fixed for every member, as above.

**A solve whose width tracks the family, not the elimination:**
- **Preconditioned elimination.** Multiply the interval system by an approximate inverse of the midpoint matrix (computed in `f64`), so that the preconditioned matrix is near the identity, then eliminate. This is the standard Hansen–Bliek / Rohn route.
- **Or a Krawczyk / interval Newton step** around the midpoint solution, `x̃ − R(A x̃ − b) + (I − R A)X`. This certifies containment where it contracts and refuses where it does not.

Either must still refuse typed rather than answer an unsound enclosure, and it must keep the `f64` and point-scalar paths bit for bit (the preconditioner applies only where a row or a datum is wider than a point).

**A refusal that names what it is.** It is owed separately, at P2: `an-undecided-collocation-pivot-refuses-as-degenerate`.

Rows owed:
- the table above, re-measured: degree 3 at `w = 0.05` solves, and widths drop to near the family's spread;
- each member's own `f64` solve stays enclosed (the S5c row `over_a_wide_parameter_the_fit_encloses_every_members`, at the wider widths).
