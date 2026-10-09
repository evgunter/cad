---
id: skin-union-mints-a-hairline-span-from-knots-an-ulp-apart
kind: issue
title: make_compatible's knot union keeps two section knots an ulp apart as two knots, so the skinned surface carries a hairline span
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [4438]
---

Filed by NURBS (`nurbs/refine-dir-hairline`, PR 4438) from its review.
That PR made the domain-uniform refinement grid drop a grid point near
a knot, because a span `g` wide carries rounding over `g` into every
derivative read off it (measured there: an enclosure width growing as
`≈ 1.7e-15 / g`, 47× at `g ≈ 2e-15`). The same span can be minted
elsewhere, and by the kernel itself rather than by input.

`sweep::skin`'s `make_compatible` (`crates/sweep/src/skin.rs`, the
`NurbsCurve3::refine_to_union(&elevated)` call ending it) puts every
section on the union of their knot vectors.
`geom_core::spline::algebra::union_refinements` merges interior values
on exact `f64` identity (`k.value() == knot.value()`). So two sections
whose knots sit one ulp (or `1e-13`) apart, e.g. sections built by the
same construction under different rounding, give the skinned surface
two knots that far apart. That is a hairline span in `u` that no
section has, and it is read by every derivative net downstream
(curvature, chart speeds, the props lanes' second derivatives).

Not measured here. Unsure whether a real loft reaches it: section
knot vectors that differ only by rounding are the case to construct.
Exact identity is `refine_to_union`'s documented contract (a
bit-identical common vector), so the fix is not obviously in the
union. It may be a snap of near-equal section knots before the union,
owned by the loft's section preparation.
