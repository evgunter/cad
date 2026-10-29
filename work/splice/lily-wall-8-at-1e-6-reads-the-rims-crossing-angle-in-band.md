---
id: lily-wall-8-at-1e-6-reads-the-rims-crossing-angle-in-band
kind: issue
title: Lily wall 8's sheath and blade walls are tangent at two points on their shared rim, so the rim's crossing cannot certify at any ε (first seen at 1e-6 as an in-band sample)
status: closed
opened: 2026-10-28
priority: P1
cost: M
refs: [a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one, a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary]
closed: 2026-10-29
---

Found by the rim-exact-image lane while measuring where lily stops next.
Until now it was recorded only as prose in
`ssiedge/plane-nurbs-certificate-bound-does-not-refine-with-eps`
(its 2026-10-28 ε 1e-6 note).

## What

At `CAD_TOLERANCE_EPS=1e-6`, demo-tour's lily wall 8 (the sheath grafted
on its blade, 9 stations) refuses at the result's rim with

`Certification { error: Escalated { check: Transversality, sample: 12,
cause: Indeterminate { margin: Value(8.313e-6), band: [1e-6, 1e-5),
predicate: "nurbs_pair_transversality" } } }`

This is `edge_nurbs::nurbs_pair_limbs`' per-sample crossing angle,
`Margin::levered(sin θ, arm)` with `arm` from `ssi::point_arm(κ, extent)`.
It is read at the feet in `chart_image`'s per-sample hook, so it comes
before any image limb, and the exact-image arm does not move it. The
same rim at default ε gets past it to limb 3
(`ssiedge/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary`).
The sweep row's 5-station blade does not reach it at 1e-6 either.

## What is open

Unmeasured: whether `sin θ` or the arm is the small factor at sample 12.
Also unmeasured: whether this is the walls' honest shallow crossing at a
coarse ε (and so a correct escalation), or an arm read too short. Measure
those two first. At 1e-12 the tour stands lily's walls down before wall 8
(the lower stem's declared frontier), so 1e-6 is the only coarse row
that reaches it.

## 2026-10-29: the walls touch on the rim, at every ε

The cause is the geometry. Once the rim's certificate bounds the walls'
crossing over the whole row (`ssi::row_crossing`), the sweep row's rim
(edge 9, `[0, 0.924]`) reads it definitely negative at `t = 0.64766` and
definitely positive at `t = 0.64769` on boxes 2e-8 wide (−5.7e-7, then
+9.0e-7): the two walls' tangent planes coincide at a point between, and
the walls swap sides across it. The rim is not transverse there, so no
certificate of it as a crossing may pass. A reading at samples alone
stepped over the point at 1e-9 and 1e-12, which is why this row once read
it in band only at 1e-6.

So this is lily wall 8's blocker at every ε. The sweep row
(`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`) stops at
it at 1e-9, 1e-6 and 1e-12; the tour's wall 8 at 1e-9 (the cover, no
sample) and 1e-6 (sample 3, levered margin 8.313e-6).

## 2026-10-29 (re-review): two points, read definitely

The tangency is two points, not one: on rim 1 the walls are tangent at
`u ≈ 0.3024` and `u ≈ 0.6977`, symmetric about `u = ½` (the PR 5959
re-review's reading off the walls' own derivatives); the `t = 0.6477`
sign change above is the second. The cover now reads the walls' crossing
over every state of the row (each piece through its own spans, no
precision assumption), and two zero-free pieces of opposite signs with
both walls' first partials continuous between them are a tangency, read
as a definite exact zero. A lower bound in band, with no sign change to
read, escalates instead. The sweep row refuses `NotTransverse` with no
sample (the whole-row reading's verdict) at 1e-9, 1e-6 and 1e-12; the
tour's wall 8 refuses the same at 1e-9, and at 1e-6 a sample's crossing
still escalates first.

## What would close it

A sheath and blade whose walls cross along the whole rim (a model change,
the scene's to make), or a description for a rim whose walls touch at a
point on it, which D2 has no form for: a seam that is transverse at some
stations and tangent at one is certifiable as neither.

## Closed (2026-10-29)

The scene no longer has this rim. Lily's long leaf is one loft from
sheath to tip, with the shared section an interior station, so there is
no sheath to graft and no boolean at the section (`[ev]` PR 5990,
question 3, approved). The finding stands in the sweep row
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`
(`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`), which keeps
the two-lofts graft as the witness that a rim tangent at two points
refuses. Lily wall 8 is now the leaf grown out of the stem, which
refuses at the curved crossing lane
(`spliceface/a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`).
