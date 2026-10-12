---
id: rational-speed-bounds-have-two-recipes-across-four-sites
kind: issue
title: A rational spline's derivative bound is computed by two different recipes across four sites, and their zero-span checks are spelled four ways with different NaN behaviour
status: open
opened: 2026-10-12
priority: P1
cost: M
design: true
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
