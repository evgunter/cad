---
id: a-spheroid-face-measures-its-closed-form-volume-and-area
kind: unit
title: A spheroid face measures inside its certified bound: the closed-form volume, and the area
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: an-ovoid-cannot-be-authored
blocked_on: [the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis]
---



Unit 7 of `an-ovoid-cannot-be-authored` (Ev, PR 7289). The bar:
"matches 4/3·π·a²·c inside its certified bound".

## What is true when it is done

- The measure door (`crates/geom-brep/src/props/curved.rs`, and the
  revolution lane `crates/topo/src/props/quad_lane.rs` where the sphere
  is read) has a spheroid arm. A spheroid zone between two latitudes
  has the polynomial volume `π·a²·(h − (z₁³ − z₀³)/(3c²))` over its
  axial extent; a full spheroid is `4/3·π·a²·c`; a wedge is its angle's
  share.
- Every body unit 5 builds measures inside its certified bound: the
  full prolate and oblate spheroids against `4/3·π·a²·c`, the zone
  against the formula above, the wedge, and the lily-shaped meridian
  against Pappus over its meridian.
- The area is inside its certified bound.

## Check first

A spheroid's area has no polynomial form: it reads `asin` (prolate) or
`atanh` (oblate) of the eccentricity, a transcendental the ruling's
"no transcendental" covers for the surface's fields only. Find whether
the measure door's lanes carry a certified enclosure of either at every
lane scalar (the interval transcendentals are their own cargo root,
`interval-transcendentals`). If not, the area reads through the door's
numeric quadrature lane. Which of the two is not ruled: if neither
certifies at `Sym`, **stop and report**.

## Rows and ground

- Changes no other row.
- Crosses SPIRICARM, NEARPOINT and QUAD (`props/curved.rs`,
  `quad_lane.rs`), MEASUREDOOR. Announce on QUAD's log.
- Runs beside units 6, 8 and 9.
- Review: **single (FULL)**: the closed form and its certified bound.
  Difficulty **M**.
