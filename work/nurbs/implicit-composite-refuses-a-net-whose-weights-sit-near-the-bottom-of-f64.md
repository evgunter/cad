---
id: implicit-composite-refuses-a-net-whose-weights-sit-near-the-bottom-of-f64
kind: issue
title: implicit_composite forms weight powers unscaled, so a net whose weights are ~1e-200 refuses where its unit-scaled twin certifies
status: open
opened: 2026-10-24
priority: P3
cost: E
---


## What

(ISO lane, PR 5635, 2026-10-24.) `geom_core::spline::compose::implicit_composite`
forms the homogeneous channels `g = w·(P − c)` and their products
(`w²`, and `S`, `T²` at `w⁴`) on the DESCRIBED weights. A net whose
weights sit near the bottom of the `f64` range underflows there and
refuses (`sup_bound()` is NaN). The same curve with its weights
rescaled by one power of two
(`spline::weight_scale::unit_scale_exact`, the move
`patch_bound::unit_scaled_weights` already makes) certifies. This is
sound but loses range.

Measured through `step-import`'s `composite_sup` (zero-radius cylinder
about the x axis), on a quadratic `[0,0,0] → [1.5,0.5,0] → [3,0,0]` with
weights `[k, 2k, k]`. The sup should not depend on `k`:

```text
k = 1, 1e-100, 1e-150, 3, 0.1 : 6.6666666666667e-1 (to rounding)
k = 1e-200, 1e-300            : NaN
```

Before PR 5635 this reached `try_line` only as a refusal of a rational
carrier. With the polynomial gate now asking constancy, a CONSTANT
`[1e-300; 3]` straight carrier reaches it too. That carrier stays NURBS,
conservatively, because PR 5635 made `try_line` refuse a NaN composite
(`f64::max` had dropped it to a residual of `0.0`).

## Fix shape

Read the composite on `unit_scale_exact`'s weights, where that rescale
is exact. It is the same curve, so the sup is unchanged, and the
underflow goes away.
