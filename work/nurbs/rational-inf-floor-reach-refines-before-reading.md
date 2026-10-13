---
id: rational-inf-floor-reach-refines-before-reading
kind: issue
title: The rational inf floors refuse nets whose weights spread past about 1.7× in a cell; a tighter floor would recover reach
status: open
opened: 2026-10-13
priority: P2
cost: M
design: true
---

## What

`geom_core::spline::rational_stretch_floor` and `rational_area_floor`
(PR 4928) read each pair term over the whole cell's hull of active
points, with the cell's whole weight range. The relaxation is loose
where the weights vary inside a cell. On the biquadratic single-cell
nets of `sweep/tests/r2_probe_cert8.rs` (`WRITTEN_CHARTS`, weights
`1 + t·(w − 1)` of a steeper original), measured by the weight spread
`w_max/w_min`:

| net | enters up to | refuses from | the floor that reaches zero |
|---|---|---|---|
| flat-rational | `1.675×` | `1.70×` | the area floor (both per-axis infs positive) |
| bowed-rational | `1.675×` | `1.70×` | the area floor (both per-axis infs positive) |
| sheared-rational | `1.70×` | `1.725×` | the area floor and `inf_v` together |

The ratio-squared bracket this replaced entered the original nets
(spread `3.7×` to `7.5×`), without a proof. The witness set sits at
`1.6×`, and `probe_rational_charts_refuse_at_a_1_8_spread` pins the
refusal at `1.8×` with the area floor named as its cause.

## Shape

Two ways to a tighter floor, both still floors of the same surface:

- **Option one, the homogeneous numerator.**
  `‖S_u × S_v‖ = ‖M̃‖/W³` with
  `M̃ = W·(A_u × A_v) − W_v·(A_u × A) − W_u·(A × A_v)`, a polynomial
  tensor spline (as `offset_fit` forms one). Its B-spline coefficient
  hull projected on `d`, over `w_max³` or `w_min³` by sign, floors the
  area element without relaxing `S` over the hull of the active
  points, which is where the pair-term floor loses. It costs the
  product spline's coefficients, of bidegree about `(3p − 1, 3q − 1)`.
  The per-axis floors have the same form, `(W·A_u − W_u·A)/W²`.
- **Knot refinement.** Inserting knots shrinks each cell's hull and
  weight range, and the pair-term floor of the refined net converges
  toward the true inf at a cost in cells.

Neither has been measured against the other or against the pair-term
floor. The question is which, if either, belongs in
`chart_stretch_inf`, which `topo`'s `certified_arms` calls per face
pair; the curve meter's piece assembly
(`rational-inf-speed-has-two-recipes-curve-and-surface`) is the other
data point on reach.
