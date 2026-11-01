---
id: a-rational-rim-with-a-near-zero-weight-is-not-live-on-equal-pieces
kind: issue
title: A rational rim with a weight near zero is not live on equal pieces: its 1/w² peaks over a sliver of the parameter no schedule resolves
status: open
opened: 2026-10-25
priority: P3
cost: M
---


Filed by FLUX's `flux/rational-rim-vector-area` lane, from PR 5696's
review.

## Measured

`geom_brep::props::rational_rim` reads a rational rim per piece, about the
piece's centre. That makes degree-7 spans and wide weight spreads live
(the rows of
`a_high_degree_or_wide_weight_rim_holds_its_truth_and_narrows` in
`crates/geom-brep/src/props/rational_rim.rs`). One class stays out of
reach: a weight near zero.

The quadratic with control points `(1, 0)`, `(1.5, 1.2)`, `(−0.2, 1)`,
weights `(1e-6, 1, 1)` and reference `(1, 0, 0)` holds its 40-digit truth
`0.96999359711476739872` at every resolution. Its enclosure is:

| pieces | width |
|---|---|
| 16 | 1.0e50 |
| 1024 | 9.7e31 |
| 16384 | 9.1e19 |

The width shrinks about 1000× per doubling, still nowhere near the
schedule's reach of 8192 pieces a span within `FACE_WORK_WALL`. Before
the per-piece forms, a reviewer measured 6.6e5 at 2¹⁷ pieces.

The integrand is `B × B′/w²`, with `w ≈ 1e-6 + 2t` near `t = 0`. So
`1/w²` peaks near `1e12` over about `1e-6` of the parameter, and its
Taylor coefficients over a piece of that width grow as `(h/1e-6)ᵏ`.
Equal pieces cannot resolve that sliver.

## Shapes

- **Geometric pieces toward a small weight.** Grade the piece widths by
  `w`'s size on the span, so the first piece near the small weight is
  about `w`'s own scale.
- **A reparametrization that equalizes the weights**: the Möbius
  transformation of a rational Bézier span, which leaves the curve
  unchanged and moves its weights toward one. Applied per span before
  the rows are read.

Either way the lane stays sound today: such a rim reads a wide enclosure
and its face stops at the work wall with a sound bracket.
