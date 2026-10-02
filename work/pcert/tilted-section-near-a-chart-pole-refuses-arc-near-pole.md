---
id: tilted-section-near-a-chart-pole-refuses-arc-near-pole
kind: issue
title: A tilted sphere section passing near a chart pole refuses Pcurves{Certify{ArcNearPole}}: the general-circle fitted image hits its refinement caps
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar, tilted-section-through-a-chart-pole-is-not-split-at-the-pole]
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
