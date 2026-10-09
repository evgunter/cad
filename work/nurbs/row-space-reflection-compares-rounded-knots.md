---
id: row-space-reflection-compares-rounded-knots
kind: issue
title: pcurves' cap-wall row test compares a carrier's knots against a ROUNDED reflection of knots_u
status: open
opened: 2026-10-09
priority: P2
cost: E
---

## Finding

`crates/topo/src/pcurves.rs`, in the cap–wall rim's intrinsic row
candidates (the `row_space` test beside `pcurve_iso_seam_column`),
decides whether a carrier lies in the wall's u-spline space reversed
by building `ku.knots().iter().rev().map(|k| a + b - k)` and comparing
it with `==` against the carrier's knots. `a + b - k` is two roundings,
so an exact mirror can read as different, and a vector an ulp away from
the mirror can read as the same. `KnotVector::mirror_symmetric`'s docs
explain why the exact-sum test (`two_sum` pairs under IEEE equality) is
the one that decides a reflection identity, and
`NurbsSurface::reversed_v`'s docs explain why rebuilding the reflected
knots is not offered.

## Severity

Low. A false positive is only a candidate: every candidate is then
checked by the `pcurve_iso_seam_column` gap decision. A false negative
loses the row candidate, and the rim falls through to the next door.
The finding is that two knot-reflection tests now coexist and only one
of them is exact.

## Shape

Use the exact reflection test for two vectors (`k_i + k'_{m−i} = lo + hi`
over `two_sum`, with matching degree and length) as a `KnotVector`
method next to `mirror_symmetric`, and call it here. The current
`mirror_symmetric` is that method with `k' = k`.
