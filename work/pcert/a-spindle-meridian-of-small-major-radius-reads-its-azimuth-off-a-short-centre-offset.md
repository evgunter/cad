---
id: a-spindle-meridian-of-small-major-radius-reads-its-azimuth-off-a-short-centre-offset
kind: issue
title: A spindle meridian of small major radius reads its azimuth off a short centre offset, a miss of about r/R ulps
status: open
opened: 2026-11-12
priority: P3
cost: M
---

Filed by `pcert/derived-sphere-image-accuracy` (PR 6870). The PR fixed
the same conditioning on the sphere arm, and reverted the torus arm's
version of it, because that version is not sound there.

## What

`pcurve_cache.rs` `chart_image`, torus/spindle meridian: the azimuth is
read off `side = w_r · sgn R`, `w_r` the circle's centre's radial part
from the chart's centre. On a spindle with `|R| ≪ r` that part is `|R|`
long. Its error is an ulp of the coordinates, while the azimuth shows at
about `r`. Centre about 12 m out, `r = 2`, at ε 1e-12, the derived
image misses by up to (the most over three azimuths):

| `R` | miss |
|---|---|
| 1e-3·r | 2.7e-13 |
| 1e-5·r | 6.3e-11 |
| 1e-7·r | 7.8e-9 |

The derivation and check 4's lemma read the same `ŵ_r`, so the mint door
refuses or escalates; nothing false is certified.

## Why the plane projection is not the fix

Reading `w_r` along the carrier plane's horizontal direction `e × n`
(the sphere arm's fix) assumes the centre's radial part lies in the
carrier's plane. A true meridian has that. But the arm also takes
circles the class gates read in band. `envelope_lemma_fuzz`'s
`SmallCircle` is a tiny circle in a tangent plane at the outer equator,
read as a meridian within the band. Its plane is the tangent plane, so
`(e × n)·w_r ≈ 0` and the projection loses the azimuth.
`the_envelope_dominates_the_stored_images_residual_on_every_periodic_arm`
then refused with a winding mismatch at ε 1e-6, seed
`0x83b8d33ffb77ad36`, trial 191.

## Direction

Read the azimuth off the better-conditioned of the two readings, which
needs a decided choice. Alternatively, read `w_r` from the carrier's
own points rather than its centre: the centre is itself a short
difference of two far points.
