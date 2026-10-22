---
id: degree-elevation-recomposition-error-grows-as-one-over-the-closest-knot-gap
kind: issue
title: degree elevation's recomposition removals amplify rounding by about one over the closest distinct-knot gap, so evaluation invariance is conditioned, not absolute
status: closed
priority: P3
cost: M
opened: 2026-09-24
closed: 2026-10-20
---

## Finding

`geom_core::spline::algebra::elevate_plan` (`crates/geom-core/src/spline/algebra.rs`,
near `:800`) elevates by the Bézier route and then removes each interior
breakpoint back down through `remove_once` (near `:684`). The removal is
exact in ℝ, and its doc says the evaluation-invariance tests pin the
floating-point agreement. That agreement is not absolute. Both chains
divide by the knot ratio: the forward chain by `α_i`, the backward chain
by `1 − α_{i+1}`. Next to a breakpoint that sits a gap `g` from another
one, that divisor is about `g` over the span, so rounding is amplified by
about `1/g`.

**Measured** (RING-3 fix pass, PR 3153): the generator of
`crates/geom/tests/curves/review_m5_pr3_attack.rs`'s
`f5_insert_refine_elevate_are_evaluation_invariant_fuzzed` produces
degrees 1-5, up to two interior knots, coordinates in ±10 and weights
0.2-5. Over 20,000 of its curves, `elevate_degree(2)`'s sup disagreement
with the source curve on a 201-point grid was:

| smallest distinct-knot gap `g` | largest disagreement |
|---|---|
| ≥ 0.1 | 1.4e-13 |
| [1e-2, 1e-1) | 2.5e-12 |
| [1e-3, 1e-2) | 2.3e-10 |
| [1e-4, 1e-3) | 2.2e-9 |

The disagreement times `g` never exceeded 5.9e-13.

The row's fixed 1e-9 tolerance failed on hosted CI for
`CAD_FUZZ_SEED=0x827507a3dc90eac7` (run 35976636119). There, two
interior knots drawn 3.8e-4 apart at degree 5 disagree by 2.0e-9. That
failure reproduces bit for bit on `main` (`2dc4ce23ec`), so it predates
RING-3.

The RING-3 fix pass made that row's elevation tolerance
`max(1e-9, 1e-11 / g)`. That states the conditioning the kernel has; it
does not remove it.

## Fix

Only if a consumer needs elevation near coincident knots to agree better
than about `1e-12 / g`. The chains are exact in ℝ for an elevation
recomposition, so which equations the removal uses is free. A
conditioning-aware split would use the forward chain while `α_i ≥ 1/2`
and the backward chain after that, rather than Piegl–Tiller's
meet-in-the-middle. Such a split should apply to elevation only: in a
lossy `remove_knot_plan` the split decides where the error lands and what
the bound reports. It moves elevation's floating-point bits, so any
digest over elevated geometry is re-cut with it.

## Measured again (2026-10-20): the law grows with degree

Measured on `main` 43b23583 with coordinates in ±10, weights 0.2–5
or all 1, the sup over nine points per span, and the worst of 60
curves. Each curve has one close interior pair `(a, a + g)` on
`[0, 1]` and is elevated once (`elevate_degree(1)`):

| source degree | g = 1e-3 | 1e-4 | 1e-5 | 1e-6 | law |
|---|---|---|---|---|---|
| 1–2 | 4e-15 | 4e-15 | 5e-15 | 5e-15 | none |
| 3–4 | 4e-13 | 4e-12 | 5e-11 | 4e-10 | `1/g` |
| 5 | 2e-12 | 3e-11 | 4e-10 | 3e-9 | `1/g` |
| 6 | 2e-9 | 2e-7 | 1e-5 | 2e-3 | `1/g²` |
| 7 | 4e-9 | 6e-7 | 3e-5 | 6e-3 | `1/g²` |
| 8 | 3e-6 | 2e-3 | 1.7 | 7e2 | about `1/g³` |
| 9 | 2e-5 | 1e-2 | 3.2 | 2e3 | about `1/g³` |

- **Refusals.** At degree ≥ 8 with drawn weights, 5% (`g = 1e-5`) to
  75% (`g = 1e-6`) of curves refused with `WeightCollapse`.
- **Compounding.** Elevating by two compounds by the second step's
  degree: degree 5, `elevate_degree(2)`, `g = 1e-8` is off by 2.2.
- **The gap is relative.** What matters is a gap's ratio to the spans
  around it, not its size: degree 3 with 1000 uniform knots 1e-3 apart
  reads 5e-15.
- **Graded knots fail with no close pair at all.** Medians of 30 drawn
  curves on ±10 coordinates:
  - geometric ×1.15 spacing at degree 10 with 40 knots: **0.15**;
  - geometric ×2 spacing: refused from degree 8;
  - graded `t^1.5` at degree 10: 2e-9.
- **Uniform knots fail at degree 9.** Degree 9 reads 4e-9 with 40 uniform
  knots and **1.6e-3** with 80. Degrees 3–8 and 10 are at rounding
  (degree 10 with 50 knots: 7e-13).

**Callers.**
- **`NurbsCurve*::approximate`** (`crates/geom/src/curves/fit.rs`)
  elevates from degree 1 and 2 only, so it is exact. That holds for
  fit-like chord knots too, with gap ratios down to 1e-5 at 2000
  samples.
- **`compose_chain`** (`crates/geom/src/curves/compose.rs`) and
  **`skin`** (`crates/sweep/src/skin.rs`) raise user geometry to the
  largest degree among it. They read a degree-≥3 close pair `1/g` to
  `1/g³` off, silently.
- **Raised to P3** for that reason (silent, metre-scale, on user
  geometry), whatever the consumer count.

## Closed (2026-10-20)

`elevate_plan`'s recomposition now tracks a first-order error
estimate per control point. Each removal keeps Piegl–Tiller's midpoint
unless another forward/backward split's estimated worst error is more
than 2× smaller (`MIDPOINT_MARGIN`). `remove_knot_plan` keeps the
midpoint.

- **The plain `α ≥ ½` split was not enough.** On uniform knots `α` ties
  at exactly ½, so it chained errors from knot to knot and refused every
  curve at degree ≥ 6 with 50 knots.
- **The margin is empirical.** It was fitted on uniform, graded,
  geometric, jittered and close-pair knot families at degrees 3–10.
  Medians of 30 curves on ±10, degree 10 with 40 knots:

  | knots | midpoint | margin 1 | 2 | 8 |
  |---|---|---|---|---|
  | uniform | 3.1e-13 | 6.9e-13 | 3.1e-13 | 3.1e-13 |
  | graded `t^1.5` | 2.0e-9 | 2.7e-12 | 1.7e-12 | 8.2e-12 |
  | geometric ×1.15 | 0.15 | 6.1e-12 | 4.1e-12 | 1.1e-11 |
  | geometric ×2 | refuses | 1.1e-13 | 1.1e-13 | 1.7e-11 |

  - With no margin, uniform degree-10 near-ties doubled the median
    rounding.
  - A margin of 8 left graded rows up to 150× worse than 2.
  - 2 is at or within 2× of the best on every family.
  - **Rows pin both sides.** Margin 1 turns the split-choice unit row
    red; margins 4 and 8 turn the steep-graded row red. No row tells
    either tie rule (`<=` at the margin, nearest-the-midpoint among
    equals) from its inverse.
- **After the fix:**
  - close pairs agree to about 4e-14 of the largest coordinate at every
    degree ≤ 10 and gap ≥ 1e-9;
  - graded degree-10 knots agree within 1e-12;
  - uniform degree 9 reads 2–3e-13;
  - uniform degree 10 reads the midpoint's medians.
- **Bits.** Against the pure midpoint, the margin moves the bits of 90
  of 800 uniform shapes (degrees 1–10, 1–80 knots), all at degree 9 or
  10, at equal or lower error.
- **Refusals:** the `WeightCollapse` refusals at degree ≥ 8 are gone.
