---
id: the-tube-chain-refines-its-carrier-through-an-f64-plan
kind: issue
title: The rung-3 tube's box chain refines its carrier through an f64 insertion plan at every scalar
status: open
opened: 2026-10-08
priority: P2
cost: E
---


Found by `pcert/projected-image` (PR 4304, the confirming review's
item-9-class finding, while making the tube's piece certified).

## What happens

`ssi::certify::box_chain` (`crates/geom-brep/src/ssi/certify.rs`) builds
the uniqueness tube's chain from `refined(carrier)` (same file, `fn
refined`), which calls `NurbsCurve3::refine_knots`. That forms its
insertion ratios and new weights in `f64`, rounded to nearest
(`geom_core::spline::algebra::refine_plan`), and applies them with
`lerp(x, y, T::from_f64(λ))` (`geom/src/curves/nurbs.rs`,
`apply_plans`). At the `Interval` scalar the chain's control hulls are
therefore those of a curve one rounding away from the carrier, not an
enclosure of the carrier's own. The boxes are padded by the ladder's
rung, which is far wider than the rounding, so the chain very likely
still covers the carrier; but no statement says so.

## Open

Refine in the ring: the homogeneous channels through
`algebra::refine_plan_homogeneous` and `CurvePlan::apply_certified`,
whose hulls enclose the described curve's, or pad each box by a
certified bound on the plan's rounding.
