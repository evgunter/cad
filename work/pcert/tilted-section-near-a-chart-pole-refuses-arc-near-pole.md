---
id: tilted-section-near-a-chart-pole-refuses-arc-near-pole
kind: issue
title: A tilted sphere section passing near a chart pole refuses Pcurves{Certify{ArcNearPole}}: the general-circle fitted image hits its refinement caps
status: closed
opened: 2026-10-02
priority: P2
cost: M
refs: [fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar, tilted-section-through-a-chart-pole-is-not-split-at-the-pole]
closed: 2026-10-08
branch: pcert/projected-image
pr: 4304
---


## Measured

Found in the review of PR 3817 (`reach/tilted-sphere-pair`) and
re-measured on its fix pass. Two unit balls, both `y`-poled, centres
`c` and `c + 1.4·(√(1 − s²), s, 0)` with `s = 0.7 + δ`, so the radical
circle passes at a distance of order `δ` from A's north pole; union:

| δ | ε 1e-9 | ε 1e-6 | ε 1e-12 |
|---|---|---|---|
| 0 (through the pole) | `ArcNearPole` | — | — |
| 1e-6 | `ArcNearPole` | — | — |
| 1e-5 | `ArcNearPole` | `ArcNearPole` | `ArcNearPole` |
| 1e-4 | builds, 7.868642399691235 | builds | `ArcNearPole` |
| ≥ 3e-4 | builds | builds | builds |

(the reviewer's own parametrisation measured the refusal out to offsets
of 1e-2 and a build at 5e-2.) The raise is the general-circle fitted
image's refinement (`geom_brep::sphere_circle`, `ImageRefusal::NearPole`
at `MAX_DEPTH`/`MAX_SPANS`, surfaced as
`PcurveCertifyError::ArcNearPole` by `pcurve_cache`), reached from the
boolean's pcurve mint: the arc is a real, valid edge whose image the
caps cannot bound.

## What a fix owes

An image bound for an arc that passes near (not through) a pole —
spans graded toward the pole without a fixed depth cap, or a pole-aware
chart image — so the δ = 1e-5..1e-4 rows build at every ε. The δ = 0
row is not this item's: an arc THROUGH a pole is the join's to split
(`reach/tilted-section-through-a-chart-pole-is-not-split-at-the-pole`).

## Closed (branch `pcert/projected-image`, 2026-10-08)

`ArcNearPole` is retired with the Hermite image. A general circle's
projected image refines its pieces adaptively toward the pole (to depth
20) until every piece's azimuth sector holds. The row's own
parametrisation was re-measured on union:

| δ | ε 1e-6 | ε 1e-9 | ε 1e-12 |
|---|---|---|---|
| 0 (through the pole) | `SectorRefused { Azimuth }` | same | same |
| 1e-6 | `bool_vertex_face_side` escalates (δ ≈ ε) | builds | builds |
| ≥ 1e-5 | builds | builds | builds |

The δ = 1e-6, ε 1e-6 cell is the boolean's vertex-on-face coincidence
at a separation of the band's own size, not the pcurve's. The δ = 0 row
stays the join's (`tilted-section-through-a-chart-pole-is-not-split-at-the-pole`).
`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`,
pins δ ∈ {1e-5, 1e-4, 3e-4} (all three tiers plus volume) and the δ = 0
refusal.
