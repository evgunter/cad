---
id: a-loops-winding-read-certifies-a-projected-nets-row-with-no-fitted-door
kind: issue
title: A loop's winding read (pcurves::tier3::read_loop) certifies a projected net's row at the closed-form door, which refuses it for want of the fitted door the pass could hold
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by SECT's `a-certificate-is-read-by-its-support` sweep
(branch `sect/certificate-by-support`, 2026-11-20), not reproduced on a
body.

`pcurves::tier3::read_loop` (the loop read under
`boolean::ops::ChartCache::describes`, W2's winding) certifies every
walked image whose door is not `CertifyingDoor::Fitted` through
`PcurveCache::certify`, the closed-form door, which passes no fitted
lane. A projected image's door is `CertifyingDoor::Either`, so a
projected image of a spline carrier's net (a fitted chord on a sphere,
cylinder, cone or torus face) reaches `run_projected_checks` with
`lane: None` and refuses `FittedLaneUnsupported` at check 4
(`projected::projected_envelope`'s `Net` arm), after checks 1–3. The
pass is generic over `T: AtRestPolicy`, so it could hold
`T::fitted_lane()` and certify through
`PcurveCache::certify_stated_image`, as `mint`'s `carry_rows` does.

The same shape at the site mint (`pcurves::site::site_rows`, which holds
no fitted door either) is closed by that unit: a pass with no fitted
door now refuses such a row unread (`Pcurve::needs_fitted_door`, in
`decision::certify_walked`), since it cannot certify it.

Open: whether `read_loop` meets a projected net's row on any body (a
face whose loop holds a fitted chord, read for its winding), and if so
whether the refusal reaches a user.
