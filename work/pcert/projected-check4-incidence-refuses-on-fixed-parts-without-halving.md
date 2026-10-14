---
id: projected-check4-incidence-refuses-on-fixed-parts-without-halving
kind: issue
title: geom-brep: a projected row's check 4 reads its incidence on the fixed INCIDENCE_CUTS parts and refuses without halving them
status: open
opened: 2026-10-11
priority: P3
cost: M
---


Filed by `encl/rung3-hull-refine` (row `analytic-rung3-hull-limb-refuses-on-fixed-parts-without-subdividing`), which made the analytic rung-3 limb halve the parts of the same projected hull.

## What

Check 4 of a net's projected row (`pcurve_cache::projected::projected_envelope`) reads its `Incidence` term once, through `pcurve_cache::projected_hull_lane` with no extra cuts: each knot span in `projected::INCIDENCE_CUTS` fixed parts, converted to metres by `projected::net_incidence`. A part whose Bernstein bound does not clear inflates the envelope, and the row refuses, with no halving. A tighter bound is one round away.

## Repair shape

The lane now takes `cuts` (`FittedLane::projected_hull`), and every `SpanHull` carries a certified metre bound (`meters`) and end floors (`floors`). So check 4 could run `ssi::certify::subdivide` over the incidence term as `projected::net_offset_sup` does, with a care that net does not have: the envelope sums several terms, so the incidence must be halved against what the band leaves after the other terms, not against the band alone. Unmeasured: how many rows refuse on this term today is not known; measure before building.
