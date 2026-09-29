---
id: degree-elevation-recomposition-error-grows-as-one-over-the-closest-knot-gap
kind: issue
title: degree elevation's recomposition removals amplify rounding by about one over the closest distinct-knot gap, so evaluation invariance is conditioned, not absolute
status: open
priority: P4
cost: D
opened: 2026-09-24
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
