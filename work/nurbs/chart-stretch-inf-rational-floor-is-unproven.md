---
id: chart-stretch-inf-rational-floor-is-unproven
kind: issue
title: chart_stretch_inf's rational inf rests on an unproven B-spline extension of Floater's Bézier bracket, in the unsafe direction for its consumers
status: open
opened: 2026-10-12
priority: P2
cost: M
---

## What

`geom-brep/src/pcurve_cache.rs` `chart_stretch_inf` divides the
polynomial `net_inf` floors (`inf_u`, `inf_v`, `area_inf`) by
`weight_ratio_factor` (`(w_max/w_min)²`). That extends Floater's
(1992) Bézier bracket to a B-spline net without a proof. A **lower**
bound on a stretch is the unsafe direction for its consumers (a chart
read that over-states how much parameter a metre buys), so an
unproven floor is a soundness question, not a reach one.
(Interval / error propagation class.)

The sup side moved to the pair-term bound
(`geom_core::spline::rational_speed_sup`, the rational-bound
design ruled 2026-10-12, PR `nurbs/one-rational-speed-bound`); the inf
side still reads the factor.

## Shape: the proven, centre-free twin (designer B)

Along `net_inf`'s direction `d` (unit), on each nonempty span or cell,

```text
inf_d ‖C′‖ ≥ min_{i, j} (term_ij · d) / w_max
```

with `term_ij = kᵢ·[w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(P_j − Pᵢ)]`,
the same pair terms as the sup: `C′·w` is a convex combination of the
`term_ij` evaluated at `C`, which lies in the hull of the `P_j`, so its
projection on `d` is at least the least projected term, and `w ≤ w_max`.
The division by `w_max` is right for a non-negative numerator and
`w_min` for a negative one: take the lattice min of both, as the
rational meter does. Build it beside the sup statement, route
`inf_u` / `inf_v` / `area_inf` through it, and retire
`weight_ratio_factor`.
