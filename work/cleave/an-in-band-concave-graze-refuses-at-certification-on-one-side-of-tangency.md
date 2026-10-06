---
id: an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency
kind: issue
title: a plane within the band of a hole's wall refuses its knife edge on one side of tangency and a chord certification on the other
status: open
opened: 2026-10-06
priority: P2
cost: M
---



## What

A plane δ off tangency with a round hole's wall (radius 0.5 in a 4 × 4 plate,
extruded 1), δ inside the band at the default ε (1e-9). Both normals give the
same payload. Measured on `cleave/concave-graze` (PR 4098), with CLEAVE
DR-4098's probes (`rv4098_near_graze_band`):

| azimuth θ | δ = −5e-10 (plane inside the hole) | δ = 0 | δ = +5e-10 |
|---|---|---|---|
| 0 (the seam) | `Reduce(KnifeEdge)` | `Reduce(KnifeEdge)` | `Reduce(KnifeEdge)` |
| 0.3 | `Join(Euler(Certification { ResidualExceeded { EndpointStart } }))` | `Reduce(KnifeEdge)` | `Reduce(KnifeEdge)` |
| π/2 | `Join(Euler(Certification { ResidualExceeded { EndpointStart } }))` | `Reduce(KnifeEdge)` | `Reduce(KnifeEdge)` |

At |δ| = 2e-9 every azimuth refuses in the band: `SliverVertex` at θ = 0,
`CrossingEscalated(BellyGraze)` off it. At |δ| = 1e-7 every azimuth answers.

On main before PR 4098, θ = 0 at δ = −5e-10 refused
`Reduce(ConsecutiveOnSectors)`, a kernel-invariant payload. The PR's rule (a)
read moved it to the knife edge. The off-seam certification refusals were the
same on main and on the PR's head.

## Why it matters

The three δ values are the in-band arms of one decision: whether the plane
grazes the wall. Two of them name the knife edge. Off the seam, the third
reads the plane as crossing the rim at two roots ~2e-5 apart. The join then
mints a chord between them that does not certify, and the refusal names a
certification residual (D4(i): one decision, one payload across its band).

## Where to look

The rim's crossing lane (`splitting/classify.rs`, the conic root lane) decides
two in-band roots on the hole side and a graze vertex on the other.
Rule (a)'s tangency read (`rules::apply_rule_a`) decides the wall tangent only
at a graze vertex. Whether the two-root reading should collapse to the graze
vertex in the band, or refuse as the band's own escalation, is the question.
The filleted hole's φ = 1.2 pose at 1e-12 is a sibling: there the
off-tangency is rounding, ~3.9e-9
(`a-convex-graze-of-a-cone-refuses-at-some-azimuths`).

## Found by

CLEAVE DR-4098 (review of PR 4098), MINOR-1.
