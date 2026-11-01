---
id: the-span-derivative-scale-is-spelled-in-six-places
kind: issue
title: The span derivative-net scale p/(u_{i+p+1} − u_{i+1}) is spelled in six places across four crates; give it one home in geom-core spline
status: closed
opened: 2026-10-12
priority: P4
cost: M
closed: 2026-10-12
branch: nurbs/one-derivative-scale
pr: 4866
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

## Closed (NURBS, branch `nurbs/one-derivative-scale`)

`geom_core::spline::derivative` is the home:
- `span_derivative_scale(p, lo, hi) = T::from_f64(p as f64) / (hi − lo)`
  checks nothing. A zero knot difference is the caller's: a net refuses,
  a sup skips the pair, an enclosure reads the division's own answer.
  Each site keeps its own indexing and its own check.
- `derivative_net` moved here from `geom/src/curves/nurbs.rs`
  unchanged; it refuses on a non-positive structure difference.

Every routed site computes the same operations in the same order as
before, so no bit moves.

| site | disposition |
|---|---|
| `geom` `nurbs.rs` `derivative_net` | moved to `geom_core::spline::derivative_net`; its scale is the helper |
| `topo` `shell.rs` `spline_speed` | routed; the skip on a zero structure difference stays at the site |
| `geom-brep` `pcurve_cache.rs`, the surface derivative net | routed; skip stays |
| `geom-brep` `pcurve_cache.rs` `curve_rate_bound` (the row's `chart_stretch_inf` spline arm reads its twin net) | routed; skip stays |
| `geom-brep` `ssi/enclose.rs` `pair_scales` | routed at `Interval`: `Interval::point` is `Real::from_f64`, so `deg / (point(hi) − point(lo))` is the helper's own spelling; no check, as before |
| `geom-brep` `ssi/enclose.rs` `bezier_scales` (**seventh site**, `degree / (b − a)` over a Bézier block) | routed, likewise |
| `step-import` `recognize.rs` `derivative_sup`, both directions (found in review) | routed at `f64`: `pu as f64 / denom` is the helper's own spelling at `f64`; the `denom <= 0.0` skip stays |
| `geom-core` `spline/hull.rs` `deriv_coeff` | **left**: its fixed association `(c_{i+1} − c_i)·p / Δu` (D9, stated at the site) is not `Δc·(p/Δu)`, and the two round differently at `Interval` |
| `topo` `offset_derive.rs` `polynomial_speed` | **left**: `‖ΔP‖·p / dt` at `f64` associates the same way as `deriv_coeff`, so it is not the helper's product either |
| `geom-brep` `props/quad.rs` `raw_deriv` (found in review; pass 2 had mis-filed it as a de Boor ratio) | **left**: `p·Δc / (a − b)` at `Interval` is its own association, and its `a == b` slot is a zero filler |
| test-only copies: `geom/tests/curves/lt_r1_probes.rs`, `m8_14_long_turn_meter.rs`, `piece_coverage_meter.rs` | **not routed**: each is an independent oracle, and routing it through the helper would make it check the helper against itself |

`geom-brep` `pcurve_cache.rs`'s private surface net is renamed
`skipping_surface_derivative_net`, and its doc says how it differs
from `geom_core::spline::derivative_net` (a curve's, which refuses a
degenerate support). Unifying the two would reorder the surface net's
row-major output under its sup/inf folds, so the rename is the bit-safe
fix.

A unit row, `spline::derivative::tests::the_scale_is_the_sites_spelling_at_every_scalar`,
pins the helper bitwise to the sites' literal spelling at `f64`,
`Interval` and `Dual` over edge inputs. The downstream goldens did not
see a `p·(1/Δu)` mutant, and this row does.

The two rational speed-bound recipes these sites carry are filed as
`work/nurbs/rational-speed-bounds-have-two-recipes-across-four-sites.md`
(P1, design).

**Sweeps:**
- **Pass 1, the scale's shape:** `from_f64(p as f64) / (`,
  `Interval::point((p|degree) as f64)`, `* (p as f64) /`. It found the
  six sites and `bezier_scales`.
- **Pass 2, the knot difference's shape** (`[i + p + 1] − [i + 1]`,
  `.get(i + p + 1)`), aimed at pass 1's blind spot of a scale spelled
  through another name or conversion. Its other hits are de Boor and
  blossom ratios `(u − t_i)/(t_{i+p+1−r} − t_i)` (`spline/algebra.rs`,
  `spline/locate.rs`, `spline/blossom.rs`, `props/quad.rs`), a
  different formula.
- **Pass 3:** `derived_u`/`derived_v` in `spline/net.rs` go through
  `hull.rs`'s `deriv_coeff`.
