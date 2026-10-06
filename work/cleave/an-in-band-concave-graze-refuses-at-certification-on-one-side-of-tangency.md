---
id: an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency
kind: issue
title: a plane within the band of a hole's wall refuses its knife edge on one side of tangency and a chord certification on the other
status: closed
opened: 2026-10-06
priority: P2
cost: M
branch: cleave/inband-graze
closed: 2026-10-06
pr: 4179
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

The three δ values are the Zero arm of one decision,
`split_conic_belly_graze` (off the seam) or `split_vertex_side` (at the
seam). Both say the plane touches the wall within ε, and that sends the
contact to rule (a), whose knife edge is the one honest refusal. Off the
seam inside the hole, the root of that Zero arm was placed on one of the
residue's two crossings, about 2e-5 m from the tangency. Rule (a) then
read the wall as not tangent there, and the join refused a certification
residual that names neither decision. The in-band poses at |δ| = 2e-9
reach different decisions at different sites. Each escalates in its own
words, and both recourses say to move the plane. They are not this
row's subject (`## Built`).

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

**What landed.** On the graze arm, the root is the sinusoid's extremum
nearest the plane: `φ` or `φ + π`, on the side of the conic's centre that
`split_conic_graze_side` decides. This holds on either side of tangency
(`splitting/classify.rs`, `conic_plane_meet`). An undecided side
escalates as the belly graze. It can only arise when the conic's reach
is itself within the band. The Boolean's
readers of the same lane (`boolean/reduce.rs`, `boolean/offer_rows.rs`)
get the same root.

**After**, pinned by
`split_tangent_edge_curved::a_plane_off_a_hole_wall_reads_each_decision_across_the_band`
(every azimuth of `THETAS`, δ ∈ {0, ±ε/2, ±2ε, ±100ε}, both normals,
read per ε row): |δ| ≤ ε refuses `Reduce(KnifeEdge)` at every azimuth,
along the seam or through the rim vertex; ε < |δ| < Kε escalates; |δ| ≥
Kε answers, each piece at its closed-form volume. Red on main
at all three rows, first at θ = π/2, δ = −ε/2, `Join(Euler(Certification
{ ResidualExceeded { EndpointStart } }))`. At 1e-6 the absolute-δ probe
also gave that payload at δ ∈ {−5e-10, −2e-9, −1e-7} off the seam, and
`Reduce(SliverSector)` on `split_conic_departure` at θ = 0.3,
δ = −5e-10; all are the knife edge now. The lane's own row
(`classify::tests::belly_graze_trio`) pins the root at the extremum
for a plane ε/2 inside and outside, above the centre and below it.
`an_interval_graze_root_is_as_tight_as_its_phase` pins it under
`Interval` to 1e-14. Taking acos of a ratio at ±1 left it about 4e-8 wide
(review of PR 4179).

The band's edges, δ = ±ε and ±Kε, are not sampled. There the margin
differs from the threshold only by the rounding of the plane's offset
and of `(centre − origin)·n` (about 1e-18 at 1e-9), so which arm a pose
reaches is decided by that rounding and varies with azimuth.

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
segment 1e-7 deep, inside ε. The row now admits the whole where the
segment's depth is within ε, and only on the material side.
`split_through_a_seam_ruling::a_near_tangent_cut_along_a_cylinder_ruling_never_answers_wrongly`
(from #4158), at 1e-9, a = 0.3, t = 1e-5: the sliver is 5e-11 deep and
the cylinder lands whole on its material side. Every validity tier
passes (1, 2, 3, 3′).

**Sweep-crate Booleans that reach the graze arm** (review of PR 4179:
join2, `pi_seam`, `m9_3` kissing rounds, `wall_face_tangent_reach`).
Their exact tangencies had their root 1.5–2.6e-8 rad off the contact on
main and on the contact here. No output moved.

## Closed (PR 4179, 2026-10-06)

The graze arm's root is the conic's extremum, chosen by the decided side of the plane
(`split_conic_graze_side`), not the residue's crossing read off a `Zero` margin. A pose within ε
lands whole on its material side; a pose in the band escalates at the decision that reads it
(`split_vertex_side` at the seam, `split_conic_belly_graze` off it, rule (a)'s bend for the
knife edge); a pose past Kε cuts at its closed-form volumes. The three copies of the root solve
are filed on HONE (`the-conic-plane-root-solve-has-three-homes`).
