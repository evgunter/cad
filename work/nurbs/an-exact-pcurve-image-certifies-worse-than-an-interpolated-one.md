---
id: an-exact-pcurve-image-certifies-worse-than-an-interpolated-one
kind: issue
title: surface_curve_residual certified an EXACT pcurve image worse than an interpolated one on the m8_4 seam, because to_bezier_spans_extra cut the extra breaks into a one-span image one after another (the suspected cells_touched overlap is refuted)
status: closed
opened: 2026-10-01
priority: P3
cost: M
branch: nurbs/exact-pcurve-image-remeasure
closed: 2026-10-10
pr: 4489
---


(SSI measurement lane, 2026-10-01. The numbers are measured; the
mechanism is a HYPOTHESIS, not measured.)

On the m8_4 seam (`crates/sweep/tests/m8_4_intersection_iso.rs`), the
wall's `u = 1` column is the carrier exactly. Handing
`geom_core::spline::compose::tensor::surface_curve_residual` the EXACT
image (the 2-control line `(1, t)`) certifies 9.96e-12 m on shipped code
and ≈ 5.9e-12 m under the convex-form `insert_once_ring`, roughly flat in
N. The interpolated degree-1 image through f64 foot points certifies
1.69e-12 m and 5.5e-14 m respectively. An exact input should not
certify worse.

Hypothesis: `cells_touched`'s closed overlap test (its banked NOTE 1)
includes the neighbour wall cell's polynomial extension when a
v-window ends bit-exactly on a wall knot, so the certified bound
depends on whether f64 feet happen to land bitwise on knots. The first
step is to measure which cells each image's windows touch.

## Measured on main (2026-10-10, after PR 4442)

(NURBS lane. Fixture: the loft's own seam carrier on the bowed wall,
which is that wall's `u = 1` column bit for bit; breaks injected as
`range_grid_points(t0, t1, N, carrier interior)`; interpolant through
`project_from_seed` feet at the N + 1 grid points.)

| N  | exact image | interpolated | exact restated on the grid knots |
|----|-------------|--------------|----------------------------------|
| 8  | 1.19e-14 m  | 6.24e-15 m   | 6.24e-15 m |
| 16 | 2.33e-14 m  | 9.52e-15 m   | 9.52e-15 m |
| 32 | 4.38e-14 m  | 1.64e-14 m   | 1.64e-14 m |
| 64 | 8.13e-14 m  | 2.87e-14 m   | 2.87e-14 m |

**The hypothesis is refuted.** The bowed wall is one Bézier patch
(`knots_u = [0, 0, 1, 1]`, `knots_v = [0, 0, 0, 1, 1, 1]`), and every
span of every image touched cell `(0, 0)` alone (360 of 360 windows,
instrumented). So `cells_touched` had no neighbour cell to pull in.

**The mechanism is the curve decomposition.** The exact line, restated
on the grid knots (its controls are the breaks themselves), certifies
bit-identically to the interpolant, because the feet land exactly on the
line. What separated the two was `compose::to_bezier_spans_extra`, which
inserted the extra breaks one after another into the whole net. A ring
insertion folds each new coefficient out of the previous one's interval,
so a one-span image cut at N breaks carried widths that grew with N (a
line on `[0, 1]` cut at the 254 interior 255ths: 178 ulps wide). The
interpolant's knots already are the breaks, so it paid nothing. The
carrier pays the same sequential cost, which is why both columns grow
with N.

## Closed

`to_bezier_spans_extra` decomposes onto the curve's own knots and then
cuts each extra break out of the Bézier segment it falls in, each
piece's coefficient `i` the segment's blossom at `(a^(p−i), b^i)` by
de Casteljau from the segment's own row (`sub_segment`, `p` convex
steps deep wherever the cut falls). On the m8_4 seam at 32 spans the
exact image and the interpolant now certify bit-identically, at
4.677e-15 m, and the exact image holds flat in N (4.22e-15 m at 8,
4.68e-15 m at 64). A segment of degree 1–5 cut at the 254 interior
255ths stays within 2.5, 2.75, 6.5, 5.5 and 10.5 ulps.

Pinned by `m8_4_intersection_iso::an_exact_image_certifies_no_worse_than_its_interpolant`
(exact ≤ interpolated, an absolute ceiling at 1.5× the measured
bound, and flatness from 8 to 64 spans), by
`compose::tests::a_segment_cut_many_times_keeps_its_rows_ulp_wide`,
and for soundness by
`compose::tests::cutting_extras_from_their_segment_encloses_the_exact_rows`.
