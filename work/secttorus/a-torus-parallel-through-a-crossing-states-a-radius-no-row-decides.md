---
id: a-torus-parallel-through-a-crossing-states-a-radius-no-row-decides
kind: issue
title: A torus parallel through a crossing or a tangent foot states p.x as its radius, a sum no row decides
status: open
opened: 2026-11-26
priority: P3
cost: E
refs: [conic-constructions-from-computed-sizes-unaudited, 7658]
---


Filed by GERM's conic-constructions audit (PR 7658, its table's row
17), moved here from the GERM finiteness row
`no-sign-decision-gives-a-conic-a-finite-size` because the file is
SECTTORUS's (`python3 scripts/work.py territory`).

## The site

`geom_brep::pair_section::torus::Ring::parallel`
(`crates/geom-brep/src/pair_section/torus.rs`, `fn parallel`) states
`Curve3::circle(.., rho, ..)` at the radius it is handed, and
`geom::Circle::new` panics on a negative one. Three callers:

- `CylinderTorusPose::tangent_parallel` passes the cylinder's radius,
  decided `ct_cylinder_radius`. Decided.
- `crossing_parallels` passes `p.x` of a point of
  `Circles::new(..).at(c1, c2).crossings()`: a crossing of the tube
  circle `(R, 0)`, radius `r`, with the partner's meridian circle.
- `SphereTorusPose::tangent_parallel` passes `p.x` of the coaxial
  tangency's `Circles::at(..).foot()`.

For the last two, `p.x` is `R + û.x·a ∓ û.y·h` as computed. In real
numbers it lies on the tube circle, so it is at least `R − r`, which
`Ring::of`'s ring row holds positive past the band. That is a
different expression from the size stored: the rounding of
`û.x·a ∓ û.y·h` is charged to nothing, and no row decides the sum the
circle stores.

## Reproduction

Not built. The route is a sphere or a partner torus coaxial with a
ring torus whose `R − r` passes the ring row by little, posed far from
the origin, so that the crossing's rounding (a charge of the
coordinates' size, `Ring::size`) exceeds `R − r`: the crossing's `p.x`
can then read at or below zero and `Ring::parallel` panics in
`Circle::new`. A row that builds one, or a proof that the ring row's
margin covers the crossing's rounding, closes this.

## What a fix has to supply

Either the sum decided where it is stored, on the crossing's own lever
(the row that would refuse, not a clamp), or the argument above made a
bound. With `[sizes]` (`docs/DESIGN.md`) the circle is minted from a
`Size`, whose mint refuses a non-positive radius typed rather than
panicking; the sign question here is the producer's pose decision
either way.
