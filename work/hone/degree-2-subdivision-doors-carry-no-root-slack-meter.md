---
id: degree-2-subdivision-doors-carry-no-root-slack-meter
kind: issue
title: The degree-2 subdivision doors (circle x torus, circle x tilted wall, ellipse x sphere and wall) carry no root-slack meter; the ellipse x torus door shows unplaceable roots are certified without one
status: closed
opened: 2026-10-03
closed: 2026-10-08
priority: P1
cost: M
refs: [line-roots-carry-no-root-slack-meter, 3973, circle-torus-certifies-shallow-roots-off-by-more-than-the-band]
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
(`ellipse_torus::ellipse_torus_roots`) hands it one
(`bool_ellipse_torus_root_slack`). `circle_roots::half_angle_roots`
passes `None`, so its two callers certify every bisected root that
reads ON the surface, whatever its slope:

- `circle_torus::circle_torus_roots` (the general arm);
- `conic_quadric::conic_quadric_roots` (the ladder arm: a circle or an
  ellipse against a sphere or a wall).

## Evidence (on the degree-4 door, not on these)

On the ellipse × torus door, with the meter disabled, the mpmath oracle
(`scripts/oracles/ellipse_torus_mpmath.py`, 3000 draws of
`ellipse_torus::torus_rows::dump_for_the_mpmath_oracle`) found 145 of
the 426 poses whose answer the meter changes certifying roots up to
6.6e-11 m of arc (66 zero bands) from the true root at ε 1e-12 — grazes
whose crossings have a slope near 1e-5 along the carrier, where the
`f64` rounding of the residual moves the bisected root by tens of bands.
Pinned by `torus_rows::a_root_the_band_cannot_place_is_not_certified`.
The degree-2 doors bisect the same way on the same kind of residual, so
the same grazes should misplace their roots there too; that is
estimated, NOT measured on them.

## Measured on the conic × quadric door (review of PR 4042)

The single full review of PR 4042 (branch `analysis/reach-review/4042`,
`review.md`, NOTE-2, probes `probes/door_fuzz.rs`) measured the
ladder's certified roots on this door, against a double-double
true-distance oracle. The roots read ON the surface (to ≤ 0.15·ε), but
lie far along the carrier from the true root:

- the fuzz (2,000 seeded poses per ε, circles and ellipses against
  spheres and walls): up to 1.65e-5 m (16,481·ε) at ε 1e-9, and
  2.7e-4 m at 1e-12;
- the band-edge sweep (`review_arm_edge`, `A₂` 0.5–0.999·ε): up to
  0.0195 m at 1e-12, on an `A₂ = 0.99·ε` pose at r = 100 m that lands
  in the band's gap and so takes the ladder.

The counts are identical on main (the old circle × cylinder and ellipse
doors) and on PR 4042's head: the misplacement is the ladder's, not the
arm switch's. The degree-2 estimate above is therefore measured here.

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

## Measured (dual review of PR 3973, reviewer r1)

No longer only estimated on the degree-2 doors. Reviewer r1's
differential (`probes/r1_circle_diff.rs` on
`analysis/reach-dual/3973-r1`, against an mpmath oracle at 60 digits)
found the circle × torus door certifying misplaced roots on 217 of
3,600 poses at the PR's head (218 on main): 1 to 42 zero bands off, at
ε 1e-9, ×1e3 grazes. These are wrong answers on main — certified roots
off the true crossing — so the row is raised from P2 to P1 (a live
wrong answer on unusual geometry; P0 is reserved for normal geometry).
One of them moved in PR 3973 (ε 1e-12, ×1e3, inner graze: main
certified roots 157,518 bands off; the PR answers `Uncertain`).

## The plumbing exists (2026-10-06)

`circle_roots::half_angle_roots` takes the subdivision's optional
`RootSlack` meter (PR of `an-edge-crossing-a-cone-face-has-no-root-lane`).
The conic × quadric door's cone arm hands it one
(`bool_conic_cone_root_slack`, the residual read by
`geom_brep::conic_cone_residual`); the door's sphere and wall ladder arm
and `circle_torus::circle_torus_roots` still pass `None`, and what this
row asks of them is unchanged.

## Fixed with GERM's P0 (PR 4357, 2026-10-08)

GERM's circle × torus measurement found the general arm wrong outside
the band at `ε = 1e-12` (321 root placements of 9,000 poses, up to
166 `Kε`, against an exact double-double oracle), filed as the P0
`work/germ/circle-torus-certifies-shallow-roots-off-by-more-than-the-band.md`,
and the conic × quadric ladder arm the same way (123 of 3,000 shallow
poses, up to 74 `Kε`). Both doors now hand the subdivision a meter
(`bool_circle_torus_sub_root_slack` on `F` itself through
`geom_brep::conic_torus_implicit`; `bool_conic_quadric_sub_root_slack`
through `geom_brep::conic_quadric_residual`), and both sweeps read 0
wrong after. The measurements and what newly refuses are in that item.
