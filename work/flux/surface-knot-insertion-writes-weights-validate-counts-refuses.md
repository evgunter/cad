---
id: surface-knot-insertion-writes-weights-validate-counts-refuses
kind: issue
title: NurbsSurface knot insertion writes weights validate_counts would refuse (a subnormal net refines to a zero weight)
status: open
opened: 2026-10-01
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
`pcurve_cache.rs`) trusted it until PR 3614, which now returns poison
for such a list.

The fix belongs at the producer: the refinement doors should
re-validate their output and refuse with a typed `KnotAlgebraError`,
not hand back a payload `new` would reject. Every consumer should not
have to carry its own re-check.

## Also noted: the weight ratio overflows on validated weights

`weight_ratio_factor` computes `(w_max/w_min)²`. For validated weights
such as `[1e-200, 1, 1, 1]`, that overflows to `+inf`, so the sup arms
become `inf`. This errs in the safe direction (it over-states, so the
lane refuses), but it means any rational net with a weight ratio above
about 1e154 has no usable sup arm. Pinned in `geom_brep`'s
`weight_ratio_poison` test. It is not a defect in this program's
ground; it is recorded here because it shares the weight-range question
with the finding above.
