---
id: an-exact-pcurve-image-certifies-worse-than-an-interpolated-one
kind: issue
title: surface_curve_residual certifies an EXACT pcurve image worse than an interpolated one (9.96e-12 vs 1.69e-12 m on the m8_4 seam); suspected cells_touched's closed overlap pulling in a neighbour cell when a window ends bit-exactly on a wall knot
status: open
opened: 2026-10-01
priority: P3
cost: M
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

After the decomposition cuts each break out of its own Bézier segment
(at most 2p insertions per sub-segment):

| N  | exact image | interpolated |
|----|-------------|--------------|
| 8  | 6.95e-15 m  | 4.22e-15 m   |
| 32 | 8.58e-15 m  | 5.57e-15 m   |
| 64 | 9.19e-15 m  | 5.57e-15 m   |

The exact image still certifies about 1.5× the interpolant. That is the
floor of the 2p interval insertions a one-span image pays at each
sub-segment, which an image already on the breaks does not pay.
