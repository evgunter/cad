---
id: lily-wall-8-at-1e-6-reads-the-rims-crossing-angle-in-band
kind: issue
title: At ε 1e-6 lily wall 8's rim escalates the NURBS pair's crossing angle at schedule sample 12 (levered margin 8.3e-6), before any image limb is read
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one, a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary]
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

## 2026-10-28: read by the shared row's certificate

The rim is now described as its walls' shared row (D2's structural
selector,
`the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance`),
whose certificate reads the same crossing angle through the same
reading (`edge_nurbs::feet_transversality`, via `row_transversality`),
at the certification schedule's samples and at the feet each wall's row
states. At 1e-6 it escalates at sample 3 of that schedule with the same
levered margin, 8.313e-6. The default-ε row now gets past the rim to the
interior-loop guard.

