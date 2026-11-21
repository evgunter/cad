---
id: the-extent-folds-sagitta-is-read-as-r-times-one-less-cos
kind: issue
title: extent_inf reads an arc's sagitta as r(1 - cos h), which keeps r's rounding for a short arc of a large circle
status: open
opened: 2026-11-19
priority: P3
cost: C
---

Found by `oneround/section-area`'s shape sweep (2026-11-19, merge base
`7b752a7a`), which looked for readings that get a small quantity on a
conic by subtracting two of the conic's own size.

## What

`extent_inf` in `crates/geom-brep/src/extent.rs` reads an arc's fold,
its sagitta, as `r * (T::one() - half_span.cos())`, for a circle's
radius, an ellipse's `speed_lo` and a spiric's minor radius. For a
short arc the difference `1 − cos h` cancels to `h²/2` and keeps the
rounding of `1`, so the product is off by about `u·r` whatever the
sagitta's own size: 1.1e-12 at `r = 1e4`.

## What it costs today

Not measured to cost a verdict. The fold is taken under
`chord.max(fold(..))`, and a short arc's sagitta is below its chord
(`r·h²/2` against `2·r·h`), so the maximum is the chord wherever the
fold is ill-conditioned, unless the chord is itself under `u·r`. It is
filed as an instance of the shape, not as a wrong answer.

## The re-posing

`2·r·sin²(h/2)`, which cancels nowhere and reads no centre.

