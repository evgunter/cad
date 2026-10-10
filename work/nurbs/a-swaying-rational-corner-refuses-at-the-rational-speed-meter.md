---
id: a-swaying-rational-corner-refuses-at-the-rational-speed-meter
kind: issue
title: The rational speed meter refuses a swaying corner the integral arm now meters, and takes a uniform-weight curve as rational
status: open
opened: 2026-10-10
priority: P3
cost: M
---


## What

`NurbsCurve3::rational_speed_lower_bound` (`crates/geom/src/curves/nurbs.rs`)
refuses the swaying corner that `a-swaying-loft-corner-refuses-as-a-vanishing-span`
made the integral arm meter. On the curve interpolating `(∓a, 0, z)`
through z = 0, 1, 2, 3 (`swaying` in
`crates/geom/tests/curves/swaying_corner_meter.rs`), true minimum speed
≈ 2.5–3 throughout, measured at the PR that closed that row:

| degree, a | integral arm (w = 1) | all weights 0.7 | w₁ raised 1.3× |
|-----------|----------------------|-----------------|----------------|
| 2, 4      | 2.21                 | 2.21            | 2.17           |
| 3, 0.5    | 2.87                 | 2.62            | 2.07           |
| 3, 2      | 1.70                 | −0.95           | −1.25          |
| 3, 4      | 0.38                 | −5.90           | −6.11          |

Two things:

- **Reach.** The rational arm's 16-split per-span scan projects on each
  refined span's control chord, and its `sup‖C − c‖·sup|w′|` term is
  span-sized; on a degree-3 swaying corner both lose at `a ≥ 2`.
- **Arm choice.** The arm is chosen on `w_j == 1.0` exactly, so a curve
  whose weights are all one value `w ≠ 1` — projectively the same
  polynomial curve — takes the rational arm and refuses where the
  integral arm answers (row 3, column 2). A loft's corner column over an
  arc-profile section carries one u-row's weight along `v`, which is
  this shape.

## Shape

Choose the arm on "all weights equal" (the integral arm is exact for a
constant weight), and give the rational arm the integral arm's piece
reading (Bernstein coefficients of the homogeneous derivative on
pieces) or a finer schedule; measure against the enclosure ceiling the
swaying-corner suite already has.
