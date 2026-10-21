---
id: trimmed-cylinder-lane-triangulates-in-raw-angle-by-height
kind: issue
title: The trimmed cylinder lane sizes cells hu (radians) by r·hu (metres) and triangulates in raw (θ, v), so at r much larger than 1 m an off-row trim point may fan across many columns
status: closed
opened: 2026-10-20
priority: P3
cost: E
closed: 2026-10-21
pr: 5560
branch: tess/raw-chart-lanes
---


## What

Unverified, from a designer on `lofted-rational-walls-refuse-at-some-deltas` (2026-10-20): the cylinder lane's parameter aspect is 1:r, so the raw-UV Delaunay sliver that makes lofted walls refuse may also hit trimmed cylinders of large radius.

## To do

Probe first: a trimmed cylinder with r = 20 m and an off-row trim point, at several δ. If it refuses or fans wide, the lattice-metric CDT ruled for the lofted walls covers it (the cylinder's isometric development), and this rides that unit. If not, close it with the measurement.

## Resolution (2026-10-21): confirmed, and fixed with the lattice metric

**Probed and red on `main`.** `m5_pr11_trimmed`'s oblique-cut cylinder half
(an elliptical trim on the cylinder wall), at radius r with the review's
twenty δ scaled by r (5e-2·r → 3e-4·r):

| r | refuses `CertificateExceeded` on `main` | with the metric |
|---|---|---|
| 1 | 0 of 20 | 0 of 20 |
| 5 | **20 of 20** | 0 of 20 |
| 20 | **19 of 20** | 0 of 20 |
| 100 | **7 of 20** (bound up to ~128·δ) | 0 of 20 |

The cause is the one guessed. The lane's lattice is square in 3-D (`hu`
radians by `r·hu` metres), so in (θ, z) it is a strip of aspect ~1/r, and
Delaunay fans an off-row ellipse chord point across it. The fix is the
NURBS lane's own: points enter the CDT at `(u/k, v)` with
`k = nurbs_cert::lattice_metric` of the lattice's realized spacing. That is
one call, the same function and the same `cdt_point` scaling, with no
second spelling. Witness row:
`m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta` (r = 5, 20
and 100 at the twenty δ: watertight, with volume within the chordal sag of
πr²H/2).
