---
id: chart-boundary-recertifies-a-lifted-iso-line-off-the-knot-domain
kind: issue
title: chart_boundary re-certifies a walked image after lifting it by the loop's deck element; on a net closed in u with a nonzero Deck.u the shifted IsoLine leaves the knot domain and should refuse IsoUnsupported
status: open
opened: 2026-10-14
priority: P3
cost: E
---


## What

`topo::pcurves::chart_boundary` lifts each walked image by the loop's
summed deck element (`lift.apply`) and then calls
`PcurveCache::certify` on the lifted image. On a spline chart closed in
`u`, `Deck.u` shifts by the knot domain's length, so a nonzero element
moves a `Fitted`, `General` or `IsoLine` image off the domain, and those
certify only by a hull bound on the domain: an iso line leaving it
refuses `IsoUnsupported` ("the hull bound holds on the domain only").
A face whose loop crosses the seam of such a net would then refuse at
`chart_boundary`, which is reached from `boolean::ops` and
`section_cert_rows`.

Found by the angle-type designer fork on `[ev]` PR 5142 (2026-10-14).
Read, not run: whether a body reaches it needs a build (a net closed in
`u` whose loop walk sums a nonzero `Deck.u`).

## Fix

The `[ev]` PR 5142 answer: a lifted copy is the row together with its
deck element, never certified again, so `chart_boundary` carries the
pair. Pin the case with a row first; it goes red today if reachable.
