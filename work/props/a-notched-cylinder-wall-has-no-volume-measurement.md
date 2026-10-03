---
id: a-notched-cylinder-wall-has-no-volume-measurement
kind: issue
title: a cylinder wall notched by rulings and an arc has neither a closed form nor a quadrature route
status: open
opened: 2026-10-01
---


## What

A cylinder face whose boundary is iso lines only — rulings and rim
arcs — but not an iso-rectangle has no volume measurement at any
scalar. The closed form (`crates/geom-brep/src/props/curved.rs`,
`cylinder`, the `props_rim_level` rule) refuses it `NotIsoRectangle`,
and `topo`'s face walk (`crates/topo/src/props.rs`, `face_flux`)
routes a face to the certified quadrature only when a trim carrier is
an ellipse, a spiric or a NURBS, so a circle-and-line boundary never
reaches the quadrature either. `topo::mass_properties` and the tier-3
validator both refuse it (`VolumeUncomputable`).

Measured (REACH, branch `reach/volume-backstop`, 2026-10-01): an
extruded half-disk (the `x = 0` line from `(0, 4)` to the origin, back
along the radius-2 arc about `(0, 2)`, height 1) minus
`brick((1.5, 2.5), (1.5, 2.5), (0.5, 2.5))`. The box notches the
curved wall from its top rim along two rulings and an arc at
`z = 0.5`. With the boolean's volume backstop bypassed the result
builds, and `mass_properties` refuses
`Face { face, source: NotIsoRectangle { what: "props_rim_level" } }`
on that wall. Through the boolean it refuses
`VolumeUnmeasured { operand: None, source: <the same> }`, pinned by
`crates/sweep/tests/reach_volume_backstop.rs`
`a_notched_wall_refuses_as_unmeasured`; that row goes red the day the
wall measures, and should then become a build at the analytic volume
`2π − ½·(½·√3.75 + 4·asin ¼ − 1.5)` ≈ 6.0437.

## The shape of a fix

Either arm closes it: a closed form over an iso-POLYGON (the flux of
a region bounded by iso lines is a sum of iso-rectangle terms), or a
structural route that sends a non-rectangular iso boundary to the
cylinder quadrature lane, which reads stored harmonic pcurves and
does not need a conic trim. The dispatch is by carrier kind today
(C5, "never a runtime fallback"), so the second wants a structural
test of the outline rather than a retry on refusal.

## Measured (JOIN-1, 2026-10-02, branch `join/1-germ-locus`)

A second witness: the merged teapot cup minus
`brick((0.02, 0.2), (-0.01, 0.1), (0, 0.3))`
(`crates/sweep/tests/verbs_1031b_arcwind.rs`
`the_boolean_after_the_merge_passes_the_join`). Once the join pairs
the section segments that run along the cup's seam edges, the subtract
builds and refuses
`VolumeUnmeasured { operand: None, source: Face { source: NotIsoRectangle { what: "props_rim_level" } } }`
on a wall half-cylinder the cutter notches along two rulings and an arc.

(Superseded the same day: BAND's one-wall-per-run sweeps, PR 3736,
rebuilt the cup without the merge, and that cup's subtract refuses
`Join(UnpairedLooseEnds { count: 4 })` on main and on JOIN-1 alike —
`the_boolean_on_the_cup_reaches_the_join`. The witness above is the
merged cup's.)

## Evidence (2026-10-01, TANG's circle × cylinder cell): Ev's engraving pose, slid across the rim

The engraving pose of `work/tang/pierce-ring-has-no-join-arm.md` with its tool slid to
`x = 0.035`, so the pocket crosses the cylinder's rim, used to refuse at the curved
pierce arm (a rim circle against the tool's arc wall). With the circle × cylinder root
lane it builds as far as the volume backstop and refuses
`VolumeUnmeasured { operand: None, source: Face { source: NotIsoRectangle { what: "props_rim_level" } } }`:
the result's wall is notched by the pocket. Pinned by
`crates/editor-core/tests/pierce_ring_engraving.rs`,
`a_pocket_across_the_rim_stops_at_the_notched_walls_volume`, which reds when this row lands.
