---
id: the-integral-meters-piece-refinement-is-not-monotone
kind: issue
title: Refining the integral speed meter's pieces is not monotone: about 5% of nets read lower at 32 pieces than at 16
status: open
opened: 2026-10-12
priority: P4
cost: M
---

## What

`NurbsCurve3::piece_assembly` (`crates/geom/src/curves/nurbs.rs`) bounds
each piece by projecting its Bernstein coefficients on that piece's own
direction (their sum). Halving a piece gives each half its own
direction, and a half's coefficients need not dominate the parent's in
its new direction, so a finer schedule can read lower. PR 4824's FULL
review measured it moving `INTEGRAL_METER_SPLITS` from 16 to 32: about
5% of degree 2–10 random integral nets read lower at 32, the worst by
24% (its probe case 2349, degree 10: 1.3019 → 0.9953;
`piece_coverage_meter.rs` pins it). The swaying family, which the count
was raised for, gains throughout (degree 3, `a = 4`: 0.128 → 0.552 of
the ceiling).

## Shape

Make refinement monotone: a sub-piece also tries its parent's direction
(one more dot per coefficient), or the meter takes the max of the 16-
and 32-piece readings. Cost reference (`Interval` loft, 40 sections
degree 3, median of 20): 11.1 ms at 16 pieces, 16.2 ms at 32; the max of
both is about their sum of piece costs, near 21 ms by that table.
