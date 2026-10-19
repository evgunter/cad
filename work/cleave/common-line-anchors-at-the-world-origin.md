---
id: common-line-anchors-at-the-world-origin
kind: issue
title: carrier_cross::common_line anchors its point at the world origin, so its rounding grows as 1/s² where the problem's grows as 1/s
status: closed
closed: 2026-10-14
opened: 2026-10-12
priority: P3
cost: M
---



Found by the review of PR 4782 (`cleave/one-carrier-reads-ellipses`).
This predates that PR, which reuses the function for ellipses.

## What

`carrier_cross::common_line`, the line two transverse planes share
(read by `circle_meets_plane` and `ellipse_meets_plane`), takes its
point as `Σ nᵢ·(hᵢ − h_j·c)/s²` with `hᵢ = nᵢ·cᵢ` measured from the
world origin. The rounding of `hᵢ` grows with the scene's distance
from the origin and is divided by `s²`, the planes' squared sine. The
problem itself is conditioned as `1/s`.

The reviewer's measurement counts definite answers of the wrong sign
out of 2000 random pairs:

| scene distance | circles | ellipses with a/b = 100 |
|---|---|---|
| 100 m | 0 | 18 |
| 1 km | 54 | 207 |

The pose tests never translate a scene, so nothing pins this.

## Direction

Anchor the line at one of the planes' own points (the circle's or the
ellipse's centre) rather than at the origin, so that `hᵢ` is a local
offset. Then pin a translated family at 100 m and 1 km.

## Closed — its subject is gone (fuse/curve-pair-reader-c)

`carrier_cross::common_line` was deleted with `carrier_cross::meetings` when `carrier_cross` became a caller of `geom_brep::curve_pair::meet` (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). `meet` builds no common line: across planes it solves the first conic's height above the other's plane as a first harmonic in its own angle and polishes each root to the closest pair.
