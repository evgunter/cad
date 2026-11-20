---
id: a-cylinder-faces-flux-sums-two-terms-of-its-far-axis-origins-size
kind: issue
title: A cylinder face's flux is R^2 A + origin.A-vector about its axis origin, two terms of size far.area that cancel to the face's own flux
status: open
opened: 2026-11-20
priority: P2
cost: M
refs: [a-corner-cut-tens-of-eps-deep-finishes-with-a-definite-volume-of-the-wrong-sign]
---


Found by the review of PR 7146 (`oneround/section-area`, frozen head
`f0fc46e1`), reading the curved-face volume lanes for the shape that PR
fixed in `split_section_area`: a small quantity read as the difference
of two terms of size `far·area`, `far` the distance to a conic's or a
surface's stored centre. By inspection; no instrumented run.

## What

`cylinder_face` (`crates/geom-brep/src/props/curved.rs:1554`) reads a
cylinder face's flux as

`flux = radius² · chart.area + (origin − O) · Σ loop_vector_area(edges, origin)`

with `origin` the cylinder frame's stored origin, on its axis. On a
face of a wall of radius `R` far from the axis (the plate of
`a-corner-cut-tens-of-eps-deep-finishes-with-a-definite-volume-of-the-wrong-sign`
at `R = 1e4`), both terms are about `R·area` (`R²` times a chart area
of `area/R`, and an axis origin `R` away against a vector area of
`area`), while the face's own flux `p·n·area` is about `|p|·area`. The
vector area is itself summed about that far origin
(`loop_vector_area(edges, origin)`, `props/loop_area.rs:61`: triangles
`(p0 − origin) × (p1 − p0)` of size `R·chord`).

## Why it may be that row's cause

The row's corner cut at `R = 1e4`, `d = 20 ε`: the face's area is about
`1e-14`, so each term is about `1e-10` to `1e-12`, and the solid's
volume about `1e-23`. The refusal's bracket `(lo, hi) =
(-7.67e-13, 1.80e-14)` (a bracket, `topo::ValidationError::VolumeSignUnresolved`'s
`volume` field, holding the truth) is about that size: what a chart-area
enclosure's width times `R²` would give. Reproduced at the review
(`crates/sweep/examples/review_corner_cut.rs` on
`oneround/section-area-review`): the same bracket on the merge base
and on the head, its last digits moved by the PR.

Not shown: that this term, rather than the aimed rays, decides the
`NegativeVolume` cells (`R = 20` included, where it is not far).

## The re-posing it suggests

Sum each face's flux about one point near the solid (the reference
point the other faces use, or the loop's own vertex), not about the
surface's stored origin, so neither term carries the distance to the
axis; the closed form of the chart term then pairs with a vector area
taken about the same point.
