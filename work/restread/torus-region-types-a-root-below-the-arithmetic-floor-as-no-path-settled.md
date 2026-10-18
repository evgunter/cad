---
id: torus-region-types-a-root-below-the-arithmetic-floor-as-no-path-settled
kind: issue
title: The torus region types a root the first-harmonic door cannot meter at this band (Unsettled::Meter) as a graze, so the query refuses RayExhausted
status: open
opened: 2026-10-18
priority: P3
cost: M
---


Found in the review of PR 5269 (`restread/torus-region`), 2026-10-18.

## What

`boolean::torus_region`'s `found` (`crates/topo/src/boolean/torus_region.rs`)
maps every `RootRefusal::Unsettled` to `RayFault::Graze`, the meter's
`Unsettled::Meter` included, and the sphere's region does the same
(`sphere_region::SphereFaceRegion::ray`). A `Meter` refusal does not say
that the path grazed. It says the root's slack is past the band:
`circle_roots::first_harmonic_roots_in_band` charges a root's angle
arithmetic `HARMONIC_NOISE_ULPS` half-ulps of a turn at the speed of the
circle it parameterises, so on a circle of radius `ρ` no root resolves
once `ρ · 8·ulp(τ)` passes the band's zero. When every path meets that
limit, the query refuses `RayExhausted` ("no test ray settled… each grazed
the boundary"), which names the wrong cause and gives the wrong recourse:
the recourse that would help is a coarser band, or a smaller part.

## Measured

`section_cert_rows::a_torus_rectangle_trims_on_every_ring`, ring
`(100, 1)` at `CAD_TOLERANCE_EPS=1e-12`: the point a tenth of a radian
past the rectangle's major window crosses only meridian edges on
parallel legs (radius ≈ 101 m, a slack of ≈ 1.1e-12 m against a band
zero of 1e-12) and refuses `RayExhausted`; the row pins it
(`parallel_leg_resolves`).

## The shape of a fix

A typed refusal for a reading below the arithmetic floor at this scale.
It carries no margin to escalate, so it fits neither `RayFault::InBand`
(a tighter band would not decide it) nor `Blocked` as `ray_walk`
defines it (a coarser band does decide it). The likely route is a
`RayFault` kind of its own, ranked by `ray_walk::Evidence`, and a
`PointInSolidError` / `ContainError` variant whose recourse names the
band. That touches the public error enums and every door that reads
`ray_walk`, which is why it was not done in PR 5269.
