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

## Built (branch cleave/inband-graze)

**Measured on main (c1199a2), the round hole at θ ∈ {0, 0.3, π/2, 2, 4},
δ ∈ {0, ±5e-10, ±2e-9, ±1e-7}, both normals.** At δ = −5e-10 off the
seam, `split_conic_belly_graze` decides the rim's margin `R − |D|`
(+5e-10) `Zero`, a graze, and the lane inserts its one root at
`φ + acos(−D/R)`. Inside the hole `−D/R` is 1 − 1e-9, so that root is
the residue's own crossing, 4.47e-5 rad (2.2e-5 m) along the rim from
the extremum; outside the clamp makes it exactly the extremum. At the
off-extremum vertex the wall meets the plane at 4.47e-5 rad, which
rule (a)'s `split_sector_coplanar` decides definitely not tangent, so
the knife edge is never read. The join then asks the section for the
chord along the contact, the section names the tangent ruling at the
extremum (`pc_parallel_gap` `Zero`), and the chord's start, 2.2e-5 m
off it, fails `EndpointStart`. The crossing lane and the section lane
placed one contact in two places.

**What landed.** On the graze arm the root is the sinusoid's extremum
nearest the plane, `cos(θ − φ) = −sign D`, on either side of
tangency (`splitting/classify.rs`, `conic_plane_meet`). The Boolean's
readers of the same lane (`boolean/reduce.rs`, `boolean/offer_rows.rs`)
get the same root.

**After**, pinned by
`split_tangent_edge_curved::a_plane_within_the_band_of_a_hole_wall_tells_one_story`
(every azimuth of `THETAS`, δ ∈ {0, ±ε/2, ±2ε, ±100ε}, both normals,
read per ε row): |δ| ≤ ε refuses `Reduce(KnifeEdge)` at every azimuth,
along the seam or through the rim vertex; ε < |δ| < Kε escalates; |δ| ≥
Kε answers, the two pieces' volumes summing to the plate's. Red on main
at all three rows, first at θ = π/2, δ = −ε/2, `Join(Euler(Certification
{ ResidualExceeded { EndpointStart } }))`. At 1e-6 the absolute-δ probe
also gave that payload at δ ∈ {−5e-10, −2e-9, −1e-7} off the seam, and
`Reduce(SliverSector)` on `split_conic_departure` at θ = 0.3,
δ = −5e-10; all are the knife edge now. The lane's own row
(`classify::tests::belly_graze_trio`) pins the root at the extremum
for a plane ε/2 inside and outside.

**The in-band payloads (|δ| = 2e-9 at 1e-9) are not this decision's.**
At the seam the seam vertex's own side is in band
(`SliverVertex`, `split_vertex_side`); off it the rim's reach is
(`CrossingEscalated(BellyGraze)`, `split_conic_belly_graze`). Each is
the escalation of the decision that reads the plane's distance from
that entity, both end in the split's coincidence recourse, and neither
moved. The graze is only decided once an entity is read on the plane.

**Also moved, all three ε rows** (the same root, read at the
extremum): the frusta of `a-convex-graze-of-a-cone-refuses-at-some-azimuths`
(narrowing θ = 0.3, widening θ ∈ {1.1, 2.9}) and the filleted slab at
φ = 1.2 now answer, so `CONE_GRAZES_REFUSED` and the slab's φ = 1.2
allowance are gone; the filleted hole at φ = 1.2 refuses its knife edge
at every row (was `SliverSector` / `KnifeEdge` / `Certification`); an
L-bracket cove (r = 0.5) refuses its knife edge across the cove
(`a_concave_graze_of_a_cove_refuses`; on main φ ∈ {3.6, 4, 4.2, 4.5}
refused `SliverSector`, 4.2 at margin 5.34e-9 as DR-4098 measured, and
`Certification` at 1e-12). `a_near_graze_of_a_cylinder_never_answers_wrongly`
at 1e-6, δ = 1e-7 inside: lands whole where main cut a 4.2e-11 m³
segment 1e-7 deep, inside ε; the row now admits the whole where the
segment's depth is within ε.
