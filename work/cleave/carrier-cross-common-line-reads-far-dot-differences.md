---
id: carrier-cross-common-line-reads-far-dot-differences
kind: issue
title: carrier_cross's common_line forms its point from a difference of far dot products, off its planes by 1e-11 at the box edge
status: closed
closed: 2026-10-15
opened: 2026-10-14
priority: P3
cost: E
---


(Found by GERM's review r5125 of PR 5125, which moved
`plane_torus_section`'s gaps to the frame rule `n·(q − c)`. That
PR's sweep for the same shape covered geom-brep and missed this
one.)

## What

`common_line` (`crates/topo/src/boolean/carrier_cross.rs:449`)
takes the line two transverse planes share from their offsets to the
WORLD origin. It forms `h1 = n1·(c1 − O)` and `h2 = n2·(c2 − O)`
(`:457`) and then `p0 = n1·(h1 − h2·c)/s² + n2·(h2 − h1·c)/s²`
(`:460`). Both are differences of dot products whose size is the
planes' distance from the origin, not their distance from each
other. Inside D4 ¶4's 1 km box, a frame 990 m out loses about three
digits of `p0` against the same pair at the origin.

A Python replica of the formula, checked against an exact-rational
oracle, gives these distances of `p0` off plane 1:

| `s = ‖n1 × n2‖` | at the origin | at 990 m |
|---|---|---|
| ≈ 0.5 | 3.7e-15 | 1.4e-12 |
| ≈ 0.05 | 1.6e-13 | 7.0e-11 |

At 990 m, both rows exceed `EPS_MIN`, and the `s ≈ 0.05` row exceeds
1e-12. `p0` feeds `circle_meets_plane`'s
`bool_carrier_cross_line_meets_circle` (`:479`–`:487`), where a
`Zero` mints the tangent point `p0 + u·(−b)`. No wrong verdict has
been shown yet. The arithmetic is executed; that it changes a verdict
is argued.

## Fix shape

Pose the line in the circle's own frame. Take `p0` as `c1` plus terms
in `n2·(c2 − c1)`, so that only the planes' gap to each other enters,
as `plane_torus_section` now does (`n·(q − c)`). Add a row that holds
a circle × plane meeting at the box edge to its in-origin verdict at
three ε, red against the current spelling.

2026-10-15 (`cleave/circle-pair-touch-on-both`): `common_line` is gone.
`carrier_cross::conic_meets_plane` meets a conic with the other plane
in the conic's own angle, from `lift = n·(c_e − c_f)`, so only the
planes' offset from each other enters. The same branch measured the
cancellation this row argued: at the 1 km box it is the minor term;
at small tilt the difference `h1 − h2·c` over `s²` put candidates up
to `u·|h|/s²` off both curves (`1e-6` at `s = 1e-5`, ε = 1e-9), which
is the parent row's defect. Pinned by
`carrier_cross::meetings_rows::every_arm_places_its_meetings_on_both_curves`,
whose sphere poses sit at `(3.7, −2.1, 0.4)`.

## Closed (PR 5190)

Closed with its twin `common-line-anchors-at-the-world-origin`: the
function is gone. The translated pose about 1 km off the origin in
`every_arm_places_its_meetings_on_both_curves` pins the box-edge case at
three ε.
