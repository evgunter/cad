---
id: the-speed-meters-arm-choice-is-not-monotone-in-reach
kind: issue
title: The speed meter's arm choice is not monotone in reach: an equal-weight curve meters worse on the integral arm than one ulp off on the rational arm
status: open
opened: 2026-10-12
priority: P3
cost: M
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
