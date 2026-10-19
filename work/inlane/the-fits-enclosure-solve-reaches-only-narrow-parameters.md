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

**Measured** at 283906cb93. The setup is five parameters `[0, 0.3, m, 0.7, 1]`, where `m` is `0.5 ± w/2`, and the data columns are `[k, 1]` for k = 0..4. Each cell is the widest solved control entry, or the refusal:

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

**Where the width comes from.** The width is the elimination's own: the dependency problem compounds through `L` and `U`, and nothing uses that every member's matrix is totally positive and near the midpoint matrix. The rows themselves are tight. They are clamped to `[0, 1]` and hulled over at most two spans here (`fit.rs`, `unit_row_in`).

## Owed

A solve whose width tracks the family, not the elimination:
- **Preconditioned elimination.** Multiply the interval system by an approximate inverse of the midpoint matrix (computed in `f64`), so that the preconditioned matrix is near the identity, then eliminate. This is the standard Hansen–Bliek / Rohn route.
- **Or a Krawczyk / interval Newton step** around the midpoint solution, `x̃ − R(A x̃ − b) + (I − R A)X`. This certifies containment where it contracts and refuses where it does not.

Either must still refuse typed rather than answer an unsound enclosure, and it must keep the `f64` and point-scalar paths bit for bit (the preconditioner applies only where a row or a datum is wider than a point).

Rows owed:
- the table above, re-measured: degree 3 at `w = 0.05` solves, and widths drop to near the family's spread;
- each member's own `f64` solve stays enclosed (the S5c row `over_a_wide_parameter_the_fit_encloses_every_members`, at the wider widths).
