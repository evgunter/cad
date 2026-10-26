---
id: a-point-has-no-place-in-a-nurbs-faces-trim
kind: issue
title: Point-in-face has no NURBS arm: contain::curved_interior_placement answers Trim(None) for a spline face, so a pierce found on one could not be placed
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane]
---


Filed by the SPLICE orchestrator from the design Ev took on PR 5727
(fork log row 198), on a designer's finding.

`boolean::contain::curved_interior_placement` answers `Trim(None)` for
a `Nurbs` (and `Approx`) face. A meeting the curve × face table finds
inside a spline face's chart is placed in or out of the face by the
face's trim (C5's clause). C4's derived pcurves make that a 2-D
point-in-loop reading at `(u, v)`. Teapot wall 3's pierce needs it.

## A second witness: the dihedral side door's wrong sheet

From the review of PR 5764 (`splice/review-5764b` @ 6d906e2d, row
`a_hairpin_carrier_reads_the_face_half_not_the_overlying_half`, not
landed). `geom_brep::dihedral_side` decides a spline face's foot on the
carrier, and this row is what places it in the face.

The fixture is a hairpin carrier, ruled along `x` (`s ∈ [−1, 1]`) and
degree 1 in `v` with its C⁰ knot at `v = ½`, through the `(y, z)` rows
`(1, 0)`, `(0, 0)`, `(1, h)`, with `h = ε/100` (witness tolerance). The
half `v < ½` is the floor strip `y ∈ [0, 1]`, `z = 0`. The half `v > ½`
folds back over it, rising `h` over the unit run. A face on the lower
half has its edge on the line `y = 3/7`, shared with the plane through
`(0, 3/7, 0)` with normal `∝ (0, 1, 1)`. At `p = (1/7, 3/7, 3h/7)` (a
point on the upper half, `3h/7` inside the band off the face) the
projection's foot is the upper half's, 0.43 m from the crease, and its
normal is the lower half's reversed. `dihedral_side` read
`Ok(Material)` walking `−x`; the lower half alone reads `Outside`. The
row asserted the hairpin reads its face half's side or escalates.

What the door needs from this row is the foot placed in the face's
trim before its normal is read. With that placement, the hairpin's
upper-half foot is out of the face, and the reading falls to the face
half or escalates.
