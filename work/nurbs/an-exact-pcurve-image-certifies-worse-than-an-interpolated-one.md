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
