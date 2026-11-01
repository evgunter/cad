---
id: rational-speed-bounds-have-two-recipes-across-four-sites
kind: issue
title: A rational spline's derivative bound is computed by two different recipes across four sites, and their zero-span checks are spelled four ways with different NaN behaviour
status: closed
opened: 2026-10-12
priority: P1
cost: M
design: true
branch: nurbs/one-rational-surface-bound
closed: 2026-10-12
---

## What

Four sites bound the derivative of a rational spline from its control
net. They use two different recipes.

**Recipe A, homogeneous.** The bound is `(sup|A′| + d·sup|w′|)/w_min`,
where `A = w·(P − c)` is the centroid-translated homogeneous net, `w′`
the weight derivative net, and `d` the net's reach from `c`.
- `topo/src/shell.rs` `spline_speed` (curve, `T`);
- `step-import/src/recognize.rs` `derivative_sup` (surface, `f64`).

**Recipe B, polynomial × weight ratio.** The bound is the polynomial
control-difference sup times `(w_max/w_min)²` (Floater 1992, squared as
the conservative reading).
- `geom-brep/src/pcurve_cache.rs` `curve_rate_bound` (curve);
- `geom-brep/src/pcurve_cache.rs` `nurbs_stretch_bounds` (surface, with
  `chart_stretch_inf`'s sups reading the same nets).

The two recipes bound the same quantity differently. Neither is wrong,
but a consumer gets a different number for the same carrier depending
on which site it reaches. That is two implementations of one piece of
logic.

**The zero-span check** in these and their neighbours (PR 4866's hit
list) is spelled four ways, and they treat a NaN knot difference
differently:
- `== 0.0` skips (`shell.rs`, `pcurve_cache.rs` ×2), so NaN computes;
- `<= 0.0` skips (`recognize.rs`), so NaN computes;
- `!(du > 0.0)` refuses (`geom_core::spline::derivative_net`), so NaN
  refuses;
- `dt > 0.0` computes (`offset_derive.rs` `polynomial_speed`), so NaN
  skips.

`ssi/enclose.rs` `pair_scales` and `bezier_scales` check nothing, and
`props/quad.rs` `raw_deriv` uses `a == b`. Valid knot vectors are
finite and nondecreasing, so these agree on every constructed carrier.
They part only on inputs no constructor admits.

## The question

Which recipe should a rational speed bound use, and should the zero
check become one predicate? This is open: A carries the weight
derivative net explicitly, while B is one factor over the polynomial
bound and is already shared by the chart doors. Which is tighter on
which carriers has not been measured. Weigh it before building. The scale itself already has one home,
`geom_core::spline::span_derivative_scale` (PR 4866).

## Closed (2026-10-12)

Neither recipe was kept. The bound is the **pair-term** statement
designed in this row's design round:
`sup‖C′‖ ≤ max_{i,j} k_i‖w_{i+1}(P_{i+1}−P_i) − (w_{i+1}−w_i)(P_j−P_i)‖ / min_active w`
per span. Each term of `C′·w` is affine in `C`, and `C` lies in the
hull of the active points, so each norm is largest at one of them. It
is never above recipe A, and at unit weights it is the polynomial
control-difference bound. It is stated once, in
`crates/geom-core/src/spline/derivative.rs` (`RationalPairTerm`,
`rational_speed_sup`, and the tensor-cell twin `rational_stretch_sup`). It surfaces as
`NurbsCurve::speed_upper_bound` and `NurbsSurface::stretch_sup`
(`SupSpeed`, memoised outside a symbolic session; certified at
`Interval`, round-to-nearest at `f64`; a weight a refinement rounds
to zero divides to `+∞` or NaN).

All four sites read it:
- `shell.rs` `spline_speed` is deleted;
- `recognize.rs` `derivative_sup` is deleted;
- `pcurve_cache.rs` `curve_rate_bound` is deleted;
- `pcurve_cache.rs` `nurbs_stretch_bounds` and `chart_stretch_inf`'s
  sups read `stretch_sup`.

`offset_derive.rs` `polynomial_speed` went too. `ssi/enclose.rs`
`CellNet` builds its terms as the same `RationalPairTerm`.
`chart_stretch_inf`'s inf side keeps the weight ratio:
`work/nurbs/chart-stretch-inf-rational-floor-is-unproven.md`. The
second-order rational bounds are
`work/nurbs/rational-second-derivative-bounds-have-two-recipes.md`.

`geom-brep/src/offset_fit.rs` `m_tilde_sup` and `cell_terms`, raised
in review, are not a rational speed bound and were not routed.
`M̃ = w(A_u×A_v) − w_v(A_u×A) − w_u(A×A_v)` is the homogeneous normal
numerator, a polynomial product spline whose sup is its coefficient
hull, with no quotient and no `w_min`.

**The zero check is not one predicate; it is gone.** The bound walks
the nonempty spans (`KnotVector::span` answers `None` for an empty
one) and reads `first_derived_window`, whose pairs all have positive
support on a nonempty span. That invariant is stated once, in the
`geom_core::spline::derivative` module doc. The NaN-knot divergence
described above concerns a state no `KnotVector` can hold, since knots
are validated finite and nondecreasing at construction. It is not a
behaviour difference on any input.

These are unchanged:
- `derivative_net`'s and `deriv_coeff`'s refusals. They are not
  provably neutral on every caller, so they were kept.
- `enclose.rs` `pair_scales` and `bezier_scales`. They are already
  span-walking.
- `props/quad.rs` `raw_deriv`'s filler.
