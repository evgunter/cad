---
id: surface-knot-insertion-writes-weights-validate-counts-refuses
kind: issue
title: NurbsSurface knot insertion writes weights validate_counts would refuse (a subnormal net refines to a zero weight)
status: closed
opened: 2026-10-01
priority: P3
cost: M
closed: 2026-10-20
---

Filed by PCERT (PR 3614's fix pass) on PROPS' slate, because
`crates/geom/src/surfaces/nurbs.rs` is this program's ground.

## Finding

`NurbsSurface::new` validates weights as strictly positive and finite
(`net::validate_counts`). The refinement doors build their output
without that check:

- `insert_knot_u`, `insert_knot_v`, `refine_knots_u` and
  `refine_knots_v` rebuild the net through `from_u_columns` or the
  `apply_plans` struct literal.
- So a net those doors return can hold a weight that `new` refuses.

Witness, from the PR 3614 review probe on branch
`pcert/review-placeholder`:

```rust
let w = f64::from_bits(1);            // 4.9e-324, passes validate_counts
let s = NurbsSurface::new(unit, unit, square, vec![w; 4]).unwrap();
let r = s.insert_knot_u(0.5, 1);      // weights [5e-324, 5e-324, 0, 0, …]
```

Knot insertion keeps weights positive in ℝ, but the convex combination
of two subnormals rounds to `0.0` in floating point.

The curve side already guards against this at its consumer:
`NurbsCurve*::rational_span_scan`, in `crates/geom/src/curves/nurbs.rs`,
re-checks the refined weights and returns poison. On the surface side,
any consumer that trusts the `validate_counts` precondition is wrong
about refined payloads. `geom_brep`'s `weight_ratio_factor` (in
`pcurve_cache.rs`) trusted it until PR 3614 made it return poison for
such a list; PR 4928 retired that factor. `chart_stretch_inf` now reads
the quotient bound (PR 4954): a weight rounded to zero gives a zero
weight-power coefficient, so a sup reads `+∞` and a floor `−∞` or NaN,
which the door clamps to zero. That is a refusal, never a low finite
bound, but it is still a consumer standing in for the producer.

The fix belongs at the producer: the refinement doors should
re-validate their output and refuse with a typed `KnotAlgebraError`,
not hand back a payload `new` would reject. Every consumer should not
have to carry its own re-check.

## Also noted: the weight ratio overflowed on validated weights

`weight_ratio_factor` computed `(w_max/w_min)²`, which overflowed to
`+inf` on validated weights such as `[1e-200, 1, 1, 1]`. PR 4928
retired it: the inf floors divide by each cell's own weights and form
no ratio, so the overflow and its `weight_ratio_poison` pin are gone.

## Re-homed from FLUX to KNOT (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. KNOT collects the spline, net and fit doors in `geom-core` and `geom`: refinement inside an enclosure, the net-state reads, the NURBS derivative and projection doors, and the fit's refusals and solves. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Closed (2026-10-20)

Fixed at the producer by PR 4923 (`nurbs/insert-weight-collapse`,
merged `50d2297d9a`), which was not linked here, so this item stayed
open after it:

- **Every plan admits the weights it writes.** `d7b1f76831` routes
  each weight a knot-algebra plan writes through one check,
  `geom_core::spline::algebra::written_weight`, against
  `knots::weight_in_regime`. That is the same predicate `new`'s
  `check_weight_values` spells. Insertion, refinement, elevation and
  removal (its two unblend chains) all pass through it, and a weight
  out of the regime refuses `KnotAlgebraError::WeightCollapse` at its
  new-polygon index.
- **The surface doors inherit it.** `insert_knot_u/v`,
  `refine_knots_u/v` and `elevate_degree_u/v` build their columns'
  plans through `map_u_columns`, so the refusal reaches them before
  `from_u_columns` reassembles the net. Each column's scalar weights
  come from `CurvePlan::apply_rational` by the same formula as the
  checked channel (`a·w_y + (1−a)·w_x` for a blend, the two unblends
  likewise), so at `f64` they are the checked values.
- **The rows, one per door.** The surface row is
  `surfaces/nurbs_surface.rs`
  `a_surface_knot_operation_that_would_write_a_zero_weight_is_refused`:
  the witness above, least-subnormal weights, all six doors, index 1
  along each door's own direction so a mis-mapped transpose reads
  another slot. The curve twin is `curves/nurbs_differential.rs`
  `a_knot_operation_that_would_write_a_zero_weight_is_refused`. With
  `written_weight` admitting everything (measured 2026-10-20 on
  `219c657438`), both rows go red; restored, both pass.
- **The consumer re-checks are gone.** The curve's
  `rational_span_scan` went with the refined-span scan (`cab4b51ccc`).
  `weight_ratio_factor` was retired by PR 4928.

No code changes here: valid nets refine through the same plans,
bit for bit as before.
