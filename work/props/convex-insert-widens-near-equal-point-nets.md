---
id: convex-insert-widens-near-equal-point-nets
kind: issue
title: insert_once_ring: the convex form widens near-equal point nets, and the pxn envelope row went red
status: open
opened: 2026-10-01
cost: E
priority: P1
branch: ssi/convex-insert-widening
---


The convex form `β·c_{i−1} + α·c_i` that PR 3524 put in
`crates/geom-core/src/spline/compose.rs` `insert_once_ring` is wider
than the lerp form `c_{i−1} + (c_i − c_{i−1})·α` it replaced when the
two coefficients are near-equal point values. The lerp form multiplies
the ratio's rounding by the small difference `c_i − c_{i−1}`. The
convex form multiplies the rounding of both ratios by the full
coefficients. On a net of coefficients near 1 m that differ by 1e-12,
that is several ulps of 1 m per insertion against about one. The
convex form wins only where the inputs are wide intervals, which the
lerp form reads twice.

`docs/PROPS-CONVEX-INSERT-SPEC.md` says "a bound that GROWS is a
finding, not a re-blessing". PR 3524's CI selected tests by diff and
never ran the row that sees it:
`crates/geom-brep/tests/r1_pxn_probes.rs`
`the_certified_sup_bounds_the_dense_sampled_true_sup`. That row went
red on main at default ε, at 1.00676 against its 1.005 ratio ceiling
at a = 1e-12. The path is `plane_nurbs_limbs` → `certify_rung3` →
`nurbs_limbs` limb 2 → `tensor::surface_curve_residual`, which
decomposes the 257-point wiggle carrier through `insert_once_ring`.

The table below swaps only the combining line in `insert_once_ring`.
Default ε, certified `hull_sup` against the sampled truth:

| a | truth | lerp (before 3524) | convex (3524) | both, intersected |
|---|---|---|---|---|
| 0 | 0 | 1.7319e-14 | 7.5495e-15 | 3.5527e-15 |
| 1e-13 | 1.00142e-13 | 1.2233e-13 | 1.5410e-13 | 1.0655e-13 |
| 1e-12 | 1.000089e-12 | 1.000770e-12 | 1.006851e-12 | 1.000670e-12 |
| 1e-11 | 1.0000223e-11 | 1.0000154e-11 | (not reached) | 1.0000153e-11 |
| 1e-10 | 1.0000012e-10 | 1.0000032e-10 | (not reached) | 1.0000032e-10 |

The fix: both forms enclose the true inserted coefficient, so their
intersection does too, and it is at least as tight as either one.
