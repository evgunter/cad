---
id: the-speed-meters-arm-choice-is-not-monotone-in-reach
kind: issue
title: The speed meter's arm choice is not monotone in reach: an equal-weight curve meters worse on the integral arm than one ulp off on the rational arm
status: closed
opened: 2026-10-12
priority: P3
cost: M
branch: nurbs/one-speed-meter-arm
closed: 2026-10-12
---

## What

`NurbsCurve3::speed_lower_bound` (`crates/geom/src/curves/nurbs.rs`)
sends a curve whose weights are all equal to the integral arm, and any
other to the rational arm. Since PR 4804 the rational arm's piece
assembly (`rational_piece_assembly`, 32 pieces of the numerator
`A′w − Aw′`) out-reaches the integral arm's 16-piece reading on a
swaying corner. On the swaying corner at degree 3, `a = 4`
(`swaying` in `crates/geom/tests/curves/swaying_corner_meter.rs`):

- equal weights: the integral arm reads 0.385;
- one weight one ulp off: the rational arm reads 1.656;
- true minimum speed: about 3.0.

So the same geometry meters four times worse for being exactly
polynomial. Measured by PR 4804's FULL review.

## Shape

Either join the arms on an equal-weight curve (the rational arm's
argument holds for any positive weights), or raise the integral arm's
reach to match (`INTEGRAL_METER_SPLITS` = 32, or its own numerator
reading). Re-measure the loft cost (PR 4803's table) either way.

## Closed (2026-10-12)

`INTEGRAL_METER_SPLITS` is 32, the rational piece schedule's count. On a
polynomial carrier the integral arm now reads what the rational arm does
one ulp off (to the last ulp at `f64`, about `1e-12` relative at
`Interval`): swaying degree 3, `a = 4`, 0.128 → 0.552 of the
ceiling (the ulp-off reading); 254 random integral nets, p10 0.741 →
0.946, median 0.981 → 0.996. Loft cost 11 → 16–18 ms (`Interval`, 40
and 24 sections). Joining the arms instead was measured and refused:
7–24× loft cost for no reach over this (the table is in the PR body).
`an_equal_weight_curve_meters_no_worse_than_one_ulp_off`
(`geom/tests/curves/swaying_corner_meter.rs`, to `1e-9` relative) is
red at 16 pieces. Off the swaying family a gap remains: 4.3% of random
integral nets read higher one ulp off, at worst by 4.6% (degree
elevation and the rational arm's refined scan), filed as
`the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net`.
Raising the count also moves about 5% of degree 2–10 nets down
(`the-integral-meters-piece-refinement-is-not-monotone`), and degrees
past the 32-piece exact range read fewer pieces (`piece_count`).
The arm still reads the value channel; the `Dual`/`Sym` half is
`the-integral-speed-meter-drops-a-dual-weights-tangent`.
