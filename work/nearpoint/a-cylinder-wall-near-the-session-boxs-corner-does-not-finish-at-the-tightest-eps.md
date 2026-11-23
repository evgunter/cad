---
id: a-cylinder-wall-near-the-session-boxs-corner-does-not-finish-at-the-tightest-eps
kind: issue
title: A unit rod notched across its rim, moved to the session box's corner (998, −998, 998) or turned and moved there, does not finish at ε 1e-12: its wall's volume reads props_cylinder_circle_off_axis undecided
status: open
opened: 2026-11-22
priority: P2
cost: M
refs: [main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated]
---


## Finding

Found by EMBED's lane `embed/iso-end` (PR 7327, 2026-11-22), with the
review's poses of
`sweep::all self_intersection_proves_injectivity::a_notched_rod_holds_across_its_reflex_corners_in_every_pose`.

The radius-1 rod over `z ∈ [0, 1]`, less the brick
`[−0.3, 0.3] × [0.4, 2] × [0.5, 2]`, finishes at every ε. Moved rigidly
by `(998, −998, 998)`, or turned 0.7 rad about `(2, 3, 6)/7` and moved by
`(−997, 997, −997)`, it finishes at the default ε and at 1e-6. At ε 1e-12
`finished` refuses:

`NotFinished { VolumeUncomputable { face: 3v1, Escalated { Indeterminate { margin: [0, 1.40e-12], band: { zero: 1e-12, escalate: 1e-11 }, predicate: "props_cylinder_circle_off_axis" } } } }`

That predicate is in `crates/geom-brep/src/props/curved.rs`. Its margin
is about the coordinates' rounding at a thousand metres, which is the
band at that ε.

The row tries the build at every ε, and skips a case loudly only while
the finish refuses for that predicate alone. The turned pose at the
corner refuses for its edges' and pcurves' incidence instead, which is
`onehome`'s
`a-body-moved-by-a-boxed-turn-refuses-its-edges-and-pcurves-at-first-order-in-the-box`.
At ε 9e-11 and 3e-11 both poses finish (the review's measurement).

This is the family of the closed GREEN row
`main-red-a-carrier-whose-origin-stands-far-off-leaves-the-union-unevaluated`
(five sweep rows refusing `VolumeUncomputable … Escalated` at 1e-12 only).
Whether this is that one move was not checked.
