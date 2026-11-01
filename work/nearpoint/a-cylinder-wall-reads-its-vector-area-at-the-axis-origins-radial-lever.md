---
id: a-cylinder-wall-reads-its-vector-area-at-the-axis-origins-radial-lever
kind: issue
title: A cylinder wall's closed form sums its vector area about the axis origin, a radius from the wall, so a turned wall of a large bulge misses with pad 0
status: open
opened: 2026-10-19
priority: P1
cost: M
---


Found by PR 5185's review (`cleave/rigid-turn-sliver-volume`), which
measured it; that PR's own sweep missed it because it measured only the
lever ALONG the axis (an origin far up the axis, 3e-14), not the one
across it.

## What

`geom_brep::props::curved::cylinder_face` takes the flux as
`R²·chart.area + origin·A⃗`, with `A⃗ = loop_vector_area(edges, origin)`
summed about the axis origin. That point stands `R` from the wall, so
on a wall that is a thin strip of a large cylinder every term of `A⃗` is
`R`-sized and they cancel to the wall's, carrying their rounding at `R`
into a closed form whose pad is zero. At rest the rounding lies in the
directions the dot product with `origin` drops; a rigid turn moves it
off. Same shape as the conic arc's centre lever (fixed in PR 5185) and
`a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever`.

## Witness (the review's)

A unit prism with one bulged sketch edge (an arc of radius set by the
bulge), turned rigidly; the body validates and `mass_properties` returns
`Ok` with pad 0:

| bulge | radius | volume miss | against ε·A |
|---|---|---|---|
| 1e-3 | 250 m (inside D4's 1 km box) | 1.4e-11 | 2.4× at ε = 1e-12 |
| 1e-4 | 2.5 km | 3.5e-8 | 5.8× at ε = 1e-9 |
| 2e-5 | 12.5 km | 4.3e-6 | |

## Priority

A wrong `Ok` with pad 0 on a body inside the box is P1 by the bands
(`work/README.md`), which is what this row is filed at; FLUXTAIL owns
the final call. The cure has to respect what a loop's in-band gaps mean
(the cone row's keel constraint applies: summing about a point of the
loop moved `a_curved_keel_in_the_plane` at 1e-6 for the cone).
