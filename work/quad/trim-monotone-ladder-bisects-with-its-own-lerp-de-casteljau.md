---
id: trim-monotone-ladder-bisects-with-its-own-lerp-de-casteljau
kind: issue
title: The trim monotone ladder bisects with its own lerp-form de Casteljau beside compose's sub_segment
status: open
opened: 2026-10-11
cost: E
---


Filed by `encl/one-halving-driver`, whose sweep for de Casteljau
sub-row cuts found it.

## What

`props::quad::bezier_bisect` (`crates/geom-brep/src/props/quad.rs:4278`)
halves a bracketed Bézier control polygon at `λ = ½` for the trim
monotone ladder (`TRIM_MONOTONE_DEPTH` levels). It is a de Casteljau
sub-row cut of its own, written in the lerp form
`x + (y − x)·½`, which reads `x` twice and carries its width into the
difference. `geom_core::spline::sub_segment` is the tree's one
Bernstein sub-row cut: the convex form with ratios formed from exact
cut points and the hull meet, so a constant channel stays exact
(`convex_step`'s docs). `RPt2` is two interval channels, so the halves
are `sub_segment` per coordinate at `(0, ½)` and `(½, 1)` on `[0, 1]`.

## Next

Route `bezier_bisect` through `sub_segment` per channel and re-baseline
what moves. Its halves are cut from the previous half, which is the
tighter choice near a zero (measured for `ssi::section::refined_sign`
on `encl/one-halving-driver`).
