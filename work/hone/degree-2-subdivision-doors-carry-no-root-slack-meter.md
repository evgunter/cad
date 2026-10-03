---
id: degree-2-subdivision-doors-carry-no-root-slack-meter
kind: issue
title: The degree-2 subdivision doors (circle x torus, circle x tilted wall, ellipse x sphere and wall) carry no root-slack meter; the ellipse x torus door shows unplaceable roots are certified without one
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [line-roots-carry-no-root-slack-meter, ellipse-edge-crossing-a-torus-has-no-root-lane]
---


Found by the REACH lane that gave the ellipse × torus cell its root
lane (`ellipse-edge-crossing-a-torus-has-no-root-lane`).

## What

`circle_roots::certified_subdivision` takes an optional root-slack
meter (`circle_roots::RootSlack`): a located root is off the true one
by at most the residual's reading plus its running rounding bound, over
the residual's least slope on the root's monotone piece, at the
carrier's top speed, and the meter refuses a root whose slack is not
definitely inside the band. The ellipse × torus door
(`ellipse_roots::torus_roots`) hands it one
(`bool_ellipse_torus_root_slack`). `circle_roots::half_angle_roots`
passes `None`, so its three callers certify every bisected root that
reads ON the surface, whatever its slope:

- `circle_torus::circle_torus_roots` (the general arm);
- `circle_cylinder::circle_cylinder_roots` (the tilted arm);
- `ellipse_roots::ellipse_roots` (the ladder arm, sphere and wall).

## Evidence (on the degree-4 door, not on these)

On the ellipse × torus door, with the meter disabled, the mpmath oracle
(`scripts/oracles/ellipse_torus_mpmath.py`, 3000 draws of
`ellipse_roots::torus_rows::dump_for_the_mpmath_oracle`) found 145 of
the 426 poses whose answer the meter changes certifying roots up to
6.6e-11 m of arc (66 zero bands) from the true root at ε 1e-12 — grazes
whose crossings have a slope near 1e-5 along the carrier, where the
`f64` rounding of the residual moves the bisected root by tens of bands.
Pinned by `torus_rows::a_root_the_band_cannot_place_is_not_certified`.
The degree-2 doors bisect the same way on the same kind of residual, so
the same grazes should misplace their roots there too; that is
estimated, NOT measured on them.

## What a fix needs

Each door supplies its residual with a running bound
(`geom_core::Rounded`; `geom_brep::conic_torus_residual` is the torus's
— the sphere's and the wall's are the same chain without the
`ρ − R` step) and a ceiling on `|F| / |residual|` near the surface (`1`
for the quadric doors, whose `F` IS the residual;
`ConicTorusHarmonics::f_per_metre_hi` for circle × torus).
`half_angle_roots` then passes the meter through. Expect more
`Uncertain` at ε 1e-12 on near-tangent poses; measure the lily and the
snowman rows before and after. The line doors' missing meter is the
same posture question (`line-roots-carry-no-root-slack-meter`).
