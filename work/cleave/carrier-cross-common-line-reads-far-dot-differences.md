---
id: carrier-cross-common-line-reads-far-dot-differences
kind: issue
title: carrier_cross's common_line forms its point from a difference of far dot products, off its planes by 1e-11 at the box edge
status: closed
opened: 2026-10-14
priority: P3
cost: E
closed: 2026-10-26
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

## Closed (2026-10-26)

Stale: PR 5154 (`f6237757e6`, the curve-pair reader's stage c1) deleted `carrier_cross::common_line`; `carrier_cross` reads its meetings from `geom_brep::curve_pair::meet`. The world-origin anchoring of a plane-plane solve elsewhere is `work/issues/offset-section-plane-plane-cramer-anchors-at-the-world-origin.md`. (CLEAVE slate triage)
