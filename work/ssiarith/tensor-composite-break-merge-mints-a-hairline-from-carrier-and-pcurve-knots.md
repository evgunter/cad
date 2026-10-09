---
id: tensor-composite-break-merge-mints-a-hairline-from-carrier-and-pcurve-knots
kind: issue
title: limb 2's tensor composite merges carrier and pcurve knots exactly, so knots an ulp apart between the two curves open a hairline span
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [4438]
---

Filed by NURBS (`nurbs/refine-dir-hairline`, PR 4438) from its review.
The finding is marked unsure, and nothing here is measured.

In `ssi::certify`'s NURBS limbs, the hull closure builds
`tensor::surface_curve_residual(&sdata, &pdata, &cdata, &extra)`. Its
module comment says "both curves are decomposed onto the MERGED break
list by exact knot insertion". The uniform `chart_breaks` grid now
stays clear of either curve's knots (PR 4438: `GRID_CLEARANCE` of the
spacing). The merge of the two curves' OWN knots still compares
exactly, though. A carrier knot and a pcurve knot one ulp (or `1e-13`)
apart therefore give the composite a span that wide. That happens
when the OQ4-aligned fit is not taken and the two were fitted
separately to the same feature.

PR 4438 measured the same span shape elsewhere: on the props lane, a
round-0 width excess of `≈ 1.7e-15 / g`; on the box chain, an axis
error of `≈ 2e-18 / g`. Whether the composite's per-span Bernstein
hulls are hurt depends on whether that decomposition divides by span
width. Value hulls of a Bézier piece should not be, so this may cost
only a redundant span. That is the first thing to check:
`crates/geom-core/src/spline/compose/tensor.rs`, the break merge in
`surface_curve_residual`.
