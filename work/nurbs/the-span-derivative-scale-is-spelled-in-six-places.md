---
id: the-span-derivative-scale-is-spelled-in-six-places
kind: issue
title: The span derivative-net scale p/(u_{i+p+1} − u_{i+1}) is spelled in six places across four crates; give it one home in geom-core spline
status: open
opened: 2026-10-12
priority: P4
cost: M
---

## What

PR 4824 gave `crates/geom/src/curves/nurbs.rs` one home for the span
derivative net (`derivative_net`). Its FULL review found the same
scale `p/(u_{i+p+1} − u_{i+1})` elsewhere. Each site's spelling:

| site | form | zero knot difference |
|---|---|---|
| `topo/src/shell.rs` `spline_speed` | homogeneous centroid-translated steps, scale in `T` over held knots, folded into a sup of norms | skipped |
| `geom-brep/src/pcurve_cache.rs`, the tensor row/column derivative net | `(at(i+1, j) − at(i, j))·factor`, `T`, held knots | skipped |
| `geom-brep/src/pcurve_cache.rs` `chart_stretch_inf`'s spline arm | `‖ΔP‖·scale` folded into a sup, `T`, held knots | skipped |
| `geom-core/src/spline/hull.rs` `deriv_coeff` | `Interval` only, its own association | (its own) |
| `topo/src/offset_derive.rs` `polynomial_speed` | `f64`, `‖ΔP‖·p/dt` | (its own) |
| `geom-brep/src/ssi/enclose.rs` (the `degree / (t_{i+degree+1} − t_{i+1})` scale) | `Interval`, per surface direction | (its own) |

All six are the same formula. They differ in the scalar, in the
association (scale applied to the difference, or folded into a norm),
and in what a zero knot difference does: `derivative_net` refuses, the
sup-folds skip.

## Shape

One `T`-generic scale (and net) in geom-core `spline`, beside
`SplineCoeffs::derivative_coeffs`, taking the zero-difference policy
from its caller. `derivative_net` moves there. Each site keeps its
operation order where a golden pins it bit for bit.
