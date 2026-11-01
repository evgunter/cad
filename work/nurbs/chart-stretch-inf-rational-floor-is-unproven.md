---
id: chart-stretch-inf-rational-floor-is-unproven
kind: issue
title: chart_stretch_inf's rational inf rests on an unproven B-spline extension of Floater's Bézier bracket, in the unsafe direction for its consumers
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-13
branch: nurbs/rational-inf-floor
pr: 4928
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

## Closed

`geom_core::spline::derivative` states both floors beside the sup,
from the same `RationalPairTerm`s through one cell builder (`cell`,
which the sup now reads too). The module docs carry the proofs:

- `rational_stretch_floor`: `W·S_u` is a convex combination of the
  pair terms at the cell's active points, so `S_u·d` is at least the
  least projected value over `w_max` (non-negative) or `w_min`
  (negative), their lattice min.
- `rational_area_floor`: `T(S) × U(S)` is affine in `S` (the
  `S × S` part vanishes), so `W²·(S_u × S_v)` is a convex combination
  of the crosses with both factors at one active point; the floor is
  the least projected cross over `w_max²` or `w_min²` by sign. It
  needs no product spline (the homogeneous `M̃` of option one); how its
  reach compares with option one's is unmeasured
  (`rational-inf-floor-reach-refines-before-reading`).

`NurbsSurface::{stretch_floor, area_floor}` are the doors; the
weights they read are in `weight_in_regime` at every producer (PR
4923), so they re-check nothing. `pcurve_cache` reads them along the
polynomial net's summed direction (`net_direction`, the sum of
`geom_core::spline::derivative_net` per column; `area_inf` along
`c_u × c_v`); `weight_ratio_factor` and
`skipping_surface_derivative_net` are gone.
At unit weights the per-axis floors read the polynomial net entries as
before; the area floor is now cell-local.

### Hit list

Patterns: `chart_stretch_inf|ChartStretchInf|area_inf`,
`nurbs_stretch_bounds`, `weight_ratio|w_max ?/ ?w_min|Floater|
(hi / lo).powi(2)`, `InfSpeed::new` producers; second pass over every
non-test reader of `weights()` / `weight_channel()` for a max-over-min
weight quotient.

| Hit | Disposition |
|---|---|
| `pcurve_cache` `nurbs_stretch_inf` | fixed: the floors |
| `pcurve_cache` `weight_ratio_factor` + its `weight_ratio_poison` row | retired; the producers hold the weight regime (PR 4923); `work/nurbs/surface-knot-insertion-writes-weights-validate-counts-refuses.md`'s citation of both updated |
| `pcurve_cache` `skipping_surface_derivative_net` | retired: `net_direction` sums `derivative_net` per column |
| `pcurve_cache` `net_inf` | now `floor_along` (the clamp and cap only) |
| `chart_stretch_sup` → `nurbs_stretch_bounds` | sup side, unchanged |
| `topo` `chart_region` `certified_arms` and its rows | reader, unchanged; green |
| `sweep` `r2_probe_cert8` written charts | the three rational nets refuse at the proven floor past about `1.7×` weight spread: their weights moved to `1.6×`, `probe_rational_charts_refuse_at_a_1_8_spread` pins the refusal at `1.8×` with the area floor as its cause, and `check_certified_arms` fails on a net that does not build; filed `rational-inf-floor-reach-refines-before-reading` |
| `sweep` `cert8_r1_probes`, `m9_2_chart_region_loft`; `step-import` `cert_n2r2_consumer_probes`; `geom-brep` `cert_n2r2_class3_probes` | readers; green |
| `geom` `NurbsCurve3::rational_speed_lower_bound` | a separate, proven derivation (the piece assembly); filed `rational-inf-speed-has-two-recipes-curve-and-surface` |
| `geom-brep` `offset_meters` `‖S_u × S_v‖` floor | its own per-cell hull derivation, `f64`-only; not a weight bracket |
| `mesh` `nurbs_cert` per-cell `w ∈ [w_min, w_max]` | a convex-hull bound with its own proof; not a weight bracket |
| analytic `InfSpeed` producers (`topo` `split`, `classify`; `geom-brep` `certify`; `sweep` `end_face`) | closed forms; no weights |

Blind spot: a weight quotient spelled through names the second pass
does not list. Every reader of the weight accessors was listed and the
ones with a ratio shape read by hand.
