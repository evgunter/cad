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
each chord cut from one carried arc pays for all of it.

## Where the time goes

Timed apart by both reviews of PR 6879 (opt-level 1, default ε, one
certificate of the rest rows' arc at s = 1):

| arc | the image (climb and per-sample hook) | limbs 1 and 2 | limb 3 | the certificate |
|---|---|---|---|---|
| sphere | 14 ms | 150 ms | 676 ms | 827 ms |
| torus | 12 ms | 93 ms | 1746 ms | 1840 ms |
| cylinder | 0.9 ms | 2.5 ms | 178 ms | 180 ms |

- **Limb 3 is 80 to 98% of it** (82%, 95% and 98% above). The image's
  climb, which this item first suspected, is about 1%.
- The tube has one box per span of the image. The climbed image has
  252 spans where the degree-1 image has 32, so a bent arc's tube reads
  eight times the boxes a plane's does.
- It is paid once per chord, on a carrier the trace already certified
  on its own knots, tube included (`ssi::certify_rung3`).

## What of it is avoidable

- **Once per carrier, not once per chord.** Every chord cut from one
  carried arc re-reads the whole arc's tube; a certificate keyed on the
  carrier and the pair would be read by each of its chords.
- **The tube's boxes hulled over runs of spans**, as the NURBS × NURBS
  lane's are (`PAIR_TUBE_BOXES`): 252 boxes of one image span each are
  finer than the theorem needs wherever a run's hull still excludes
  zero.

The plane's lane is the same shape at a tenth of the cost
(`a-carried-plane-spline-chords-at-rest-certificate-costs-half-a-second`).
