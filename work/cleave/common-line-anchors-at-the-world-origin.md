---
id: common-line-anchors-at-the-world-origin
kind: issue
title: carrier_cross::common_line anchors its point at the world origin, so its rounding grows as 1/s² where the problem's grows as 1/s
status: closed
closed: 2026-10-15
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

## Closed (PR 5190, `cleave/circle-pair-touch-on-both`)

`common_line` is gone. `carrier_cross::conic_meets_plane` meets each
conic with the other's plane in the conic's own angle, from the planes'
offset from each other (`lift = n·(c_e − c_f)`), so nothing is anchored
at the world origin. Pinned by
`carrier_cross::meetings_rows::every_arm_places_its_meetings_on_both_curves`,
whose sphere-section poses now include a pair about 1 km off the origin,
`(990, −742.5, 396)`. That pose is green at ε = 1e-9, 1e-6 and 1e-12, and
red against the `common_line` arms. The review of PR 5190 measured the
990 m case at 63 of 156 silent misses on main and 0 on the branch.
