---
id: a-curved-partners-chord-certifies-at-rest-in-seconds
kind: issue
title: A sphere or torus chord on a spline wall costs 5 to 12 s to certify at rest in debug: the union of a ball across a loft wall takes 24 s where it refused in 7
status: open
opened: 2026-11-12
priority: P2
cost: M
---


Filed by the `sectarms/spline-curved-at-rest` lane.

## What

The at-rest certificate of a sphere × spline or torus × spline chord
(`edge_nurbs::curved_nurbs_limbs`) is paid at every chord the join
mints and again wherever rest re-certifies the edge. Debug, local, one
run each, at the default ε:

| arc | the whole reading | one at-rest certificate |
|---|---|---|
| sphere, s = 0.01 (carry rows' wall) | 1.7 s | 4.9 s |
| sphere, s = 1 | 5.2 s | 5.9 s |
| cylinder, s = 1 | 0.23 s | 1.5 s |
| torus, s = 1 | 4.8 s | 12.1 s |

`loft_prism ∪ ball_poled_z(0.25, (0, 0.9, 0.1))`: 7 s on main, where the
first chord refused `Unimplemented`; 24 s now, to the containment
refusal past the join.

The certificate reads the whole carrier each time
(`carrier_net::carrier_span`: a spline spans its own knot domain), so
each chord cut from one carried arc pays for all of it. Its parts were
not timed apart: the image's climb projects 33 + 65 + 129 + 257 feet,
limb 2 composes a degree-5 image with the wall over each of the
carrier's spans (554 for the sphere at s = 1), and limb 3 reads the
tube over the same windows.

The plane's lane is the same shape at a tenth of the cost
(`a-carried-plane-spline-chords-at-rest-certificate-costs-half-a-second`).
