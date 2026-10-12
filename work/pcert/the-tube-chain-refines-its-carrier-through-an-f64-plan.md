---
id: the-tube-chain-refines-its-carrier-through-an-f64-plan
kind: issue
title: The rung-3 tube's box chain refines its carrier through an f64 insertion plan at every scalar
status: closed
opened: 2026-10-08
priority: P2
cost: E
closed: 2026-10-12
branch: pcert/tube-chain-in-the-ring
pr: 4829
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

## Closed

The fix had already landed with `nurbs/knot-algebra-off-the-ring`
(PR 4553): `box_chain` reads `refined_net`, which refines the
homogeneous channels in the ring (`RingRefinement::refine`) and
de-homogenizes, so every hull encloses the described carrier's at
every scalar. `refined_net_encloses_the_exact_refined_net` pins the
net against Boehm's insertion in exact rationals.

What this row added is the seam itself:
`the_box_chain_encloses_the_exact_refined_net_at_every_scalar`
(`ssi/certify.rs`) checks every box of `box_chain`, at `f64` and at
`Interval`, against the exact refined control points of its span. A
chain read off the `f64` insertion plan misses five of them by its
rounding; the row's contrast arm and an env-gated mutant of
`box_chain` both show it.

The sweep of `ssi/` and `edge_nurbs.rs` for `refine_knots`,
`refine_plan`, `insert_knot`, `split_at`, `refine_to_union`,
`elevate_degree`, `remove_knot` and `lerp` found production refinement
only through `RingRefinement` (`certify::refined_net`,
`section.rs`); the `refine_knots` hits are test contrast arms.
