---
id: trim-monotone-ladder-bisects-with-its-own-lerp-de-casteljau
kind: issue
title: The trim monotone ladder bisects with its own lerp-form de Casteljau beside compose's sub_segment
status: closed
opened: 2026-10-11
closed: 2026-10-11
branch: nurbs/monotone-step-and-blossom
pr: 4551
cost: E
---


Filed by `encl/one-halving-driver`, whose sweep for de Casteljau
sub-row cuts found it.

## What

`props::quad::bezier_bisect` (`crates/geom-brep/src/props/quad.rs:4247`)
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

## Closed

Closed by the NURBS lane's PR 4551, which routes the cut through the
spline layer's one certified primitive. `bezier_bisect` halves each
channel with `geom_core::spline::restrict_bernstein` at `(0, ½)` and
`(½, 1)`: the blossom on `KnotVector::bezier(p, (0, 1))`, every level
through `Certification::between`, so no lerp-form de Casteljau of its
own survives. Each half is cut from the polygon it halves, as before.
`sub_segment` is gone with `convex_step`; the segment cut on
`[s0, s1]` is `blossom::restrict_segment` beside it.

`between` is the monotone corner form, each corner also met with its
convex form, then with its sources' hull, so a dyadic split such as
this halving's `½` rounds once, as the convex form's did.
