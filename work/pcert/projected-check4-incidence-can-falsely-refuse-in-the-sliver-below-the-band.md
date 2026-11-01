---
id: projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band
kind: issue
title: Check 4's projected incidence term is read on fixed parts and never halved, so a carrier whose true miss sits about 1% under the band can falsely refuse
status: open
opened: 2026-10-14
priority: P4
cost: M
---


Filed by PCERT closing `projected-check4-incidence-refuses-on-fixed-parts-without-halving`
(PR 5128's review).

## What

Check 4 of a net's projected row
(`pcurve_cache::projected::projected_envelope`, Net arm) reads its
incidence term once, on `projected::INCIDENCE_CUTS` (8) fixed parts per
knot span, through `projected_hull_lane` and `net_incidence`. That read
over-states the true sup:
- by up to 0.3% on the suites' honest rows;
- by up to 1.2% on constructed sign-changing carriers along a cylinder's
  axis (degree 3, 5 and 7).

Measured in the closed item's census.

So a carrier whose true incidence, after the envelope's other terms,
sits within about 1% under the band's zero refuses, where a halved read
would certify it. No suite row lands there: no read lies in
`[0.1ε, 10ε]`. But a fitted carrier near ε can reach it.

## Fix

Halve the incidence term against what the band leaves after the other
terms. The template is `ssi::certify::subdivide`, which
`projected::net_offset_sup` already runs: `FittedLane::projected_hull`
takes `cuts`, and every `SpanHull` carries `meters` and `floors`. A row
needs a constructed carrier in the sliver that refuses today and
certifies after.
