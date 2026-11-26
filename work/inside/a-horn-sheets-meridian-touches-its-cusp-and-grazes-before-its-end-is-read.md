---
id: a-horn-sheets-meridian-touches-its-cusp-and-grazes-before-its-end-is-read
kind: issue
title: On a horn sheet (R = r) a meridian edge touches the axis at the cusp in a double root, which the region reads as a graze before the leg's open end drops it, so every point below the rim of an apex-reaching face refuses
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-point-door-places-no-point-on-a-spindle-sheet-face, 7790]
---

Found by PR 7790's verifier (2026-11-25), on the fix that made a
meridian leg drop an unsided root at a sheet's open apex end.

## What

On a lemon, an apple or a near-horn, a face whose meridian edges run to
an apex now reads every point the probe tried. The residuals are 4–22
per sheet, all in-band (`Escalated(Indeterminate)` within about 1e-6
rad of an apex and 1e-9 m of a meridian edge).

On the horn (R = r, which `geom::spindle_sheet` admits; the needle-tip
body is one), nothing changed: 484 refusals per frame, and in a
cap-sector face 234 of 725 points read `RayExhausted` (every point below
the rim), which is not in-band.

Likely mechanism (the verifier's, not traced): at the horn's cusp the
meridian-edge circle meets the axis tangentially. Its meeting with the
leg's plane is a double root, `Met::Touch`, which `region::landings`
(`crates/topo/src/region.rs`) turns into `Graze` before any end is
consulted. The fix's end-first rule covers only `CoMeeting`.

## Witness

The verifier's probe: the cap-sector face (`wedgecap`) on
`R = r = 1`, plain and tilted frames. It is a rim arc at
`v0 = -0.3 v*` over `u ∈ [0, 0.6]` and two meridians from `v0` to the
apex; the query below the rim reads `Err(RayExhausted)` where the truth
is `Out`. The PR's committed row
`torus_region::a_meridian_drops_a_meeting_at_its_apex` covers the lemon;
a horn case beside it is this row's pin.

## Done when

A touch at a point the leg's own end excludes is dropped as an unsided
crossing there is. Say per verdict why, and pin the horn wedge.
