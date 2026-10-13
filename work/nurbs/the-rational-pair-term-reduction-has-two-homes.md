---
id: the-rational-pair-term-reduction-has-two-homes
kind: issue
title: The joint pair-term reduction max ‖term(point)‖ / least has two homes, geom-core's pair_sup (unscreened, divided after the norm, folded with T::max) and geom-brep's patch_bound::pair_norm_sup
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## Found (ENCL, PR 4891's second delta review, 2026-10-13)

Two functions take the joint quotient-rule reduction
`max_{term, point} ‖term(point)‖ / least` over the same pair terms
(`geom_core::spline::RationalPairTerm`):

- `crates/geom-core/src/spline/derivative.rs` `pair_sup`, behind
  `rational_speed_sup` (read by `geom/src/curves/nurbs.rs`'s speed) and
  `rational_stretch_sup` (read by `NurbsSurface::stretch_sup`);
- `crates/geom-brep/src/patch_bound.rs` `pair_norm_sup`, behind
  `RationalForm::speed_sup`/`sum_sup`, which `patch_bound::rational_cells`
  and `ssi::enclose` (`CellNet::derivative_norm_sup`,
  `transverse_readings`) read.

They differ in three ways:

1. `pair_sup` reads every term at every point; `pair_norm_sup` screens
   each pair in `f64` and certifies only the winners (`screened_max`).
2. `pair_sup` divides by `least` after the norm (`s / least`).
   `pair_norm_sup` divides each term by the floor before the norm
   squares it, the order PR 4872 settled for certified reads, because
   squaring an undivided term from a net of subnormal weights underflows.
3. `pair_sup` folds with `T::max`; `pair_norm_sup` folds with
   `max_bound`, which keeps a `NaN` read.

## What is open

One reduction with one division order, which both homes call. The
candidates are the screened `pair_norm_sup` moved down into geom-core
over `T`, or `pair_sup` taking the floor first and keeping `NaN`. The
cost of the screen over `T` (whose `f64` image a non-`f64` `T` may not
carry cheaply) is the open point.

