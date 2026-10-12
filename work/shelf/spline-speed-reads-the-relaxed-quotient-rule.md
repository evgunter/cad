---
id: spline-speed-reads-the-relaxed-quotient-rule
kind: issue
title: topo::shell's spline_speed reads a rational spline's speed through the triangle-relaxed quotient rule, (max step + reach·max weight step)/least, where the joint maximum over terms and control points is never larger
status: open
opened: 2026-10-12
---



## Found (encl/one-chart-speed, 2026-10-12)

`crates/topo/src/shell.rs` `spline_speed` bounds a rational spline's
speed through the quotient rule taken over separate maxima:
`(max‖A′ coefficient‖ + d·max|w′ coefficient|)/min w`, with `d` the
farthest control point's distance from `control[0]`. That is the
triangle relaxation of `max_{i,j} ‖k_i − g_i·q_j‖/min w`, where `k_i`
is the `i`-th coefficient of `A′` about `c`, `g_i` the matching
coefficient of `w′`, and `q_j = P_j − c`. Each term is affine in
`C − c`, so its norm is largest at a control point. The joint maximum
is never larger than the relaxed one and keeps the cancellation
between `k_i` and `g_i·q_j` that the relaxation drops.

ENCL's `encl/one-chart-speed` moved the surface meters
(`patch_bound::rational_cells`) and `ssi::enclose` onto that joint
reading through `patch_bound::QuotientForm::speed_sup`. On the
meters' fixtures the relaxed reading was up to `1.125`× the joint
one. The curve's own figure has not been measured. `spline_speed` is
generic over `T: Real` and is not a certified reading, so the
surface's interval form does not drop in as it is. The joint maximum
over the curve's `(p)·(n)` term-offset pairs is the same reduction in
`T`.

## What is open

Decide whether `spline_speed`'s callers gain from the tighter speed.
They are `ArcPiece::along` in `crates/topo/src/shell.rs`, where the
speed times a window's half-width is the radius of the ball a spline
piece of `MovedWall::cut` is read in. If they do, take the joint
maximum. Measure the curve figure first.
