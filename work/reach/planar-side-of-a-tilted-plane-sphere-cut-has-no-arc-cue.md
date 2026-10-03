---
id: planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue
kind: issue
title: The boolean's planar side of a plane x sphere cut tilted against the sphere's chart has no arc cue: it selects by the wall face's azimuth window, which a tilted section lacks
status: review
opened: 2026-10-02
priority: P1
cost: M
refs: [sphere-seam-in-a-plane-face-loses-the-fallback-recut-to-the-tilted-section-refusal, arc-side-rule-has-two-predicates]
pr: 3985
branch: reach/arc-from-pairing
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. A sphere pair's section
reaches both sides' WALL lanes (the radical plane), where a section
tilted against the chart now takes the run-side arc rule. A plane ×
sphere pair's planar side is the `JoinLane::BoolPlanar` arm, which
selects its arc by the mate wall face's azimuth window handed over by
value (`chord_join::bool_planar_chord_spec`), and a tilted section has
no monotone azimuth, so it refuses `SplitJoinError::SectionNotPolar`.
Every op of each pose below refuses there
(`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_tilted_section_stops_at_the_sector_side_and_the_planar_side`;
`m5_pr9c_sphere_doors.rs`, the die-pips shape; `germ_coplanar_conic.rs`,
the y-poled pip):

- `brick((0.5, 3), (−2, 2), (0, 2))` against the `y`-poled unit ball at
  the origin;
- the unit cube against the `y`-poled ball(0.3) at `(0.5, 0.5, 1)`.

## What the lane tried, measured

Wiring the run-side rule (the arc leaves each run end on the run's
left, `select_arc_by_run_side`) into the planar side as well:

- the box pose above built ∪ and ∖ to the quarter-cap closed form
  (`20 + 4π/3 − c/2`, `20 − c/2`, `c` the cap beyond `x = 0.5`), and ∩
  refused `Merge(InputNotClosed { ScaffoldingStrutVertex ×2 })`;
- the same box mirrored to `z ∈ (−2, 0)` refused `Join(SectionInvariant
  { "a run edge has no closed-form chart image on the divided face's
  chart" })` — `boolean::join` computes the wall face's window for the
  planar side up front (`face_azimuth_window`) and the wall face by then
  carries a tilted-circle arc;
- a box face holding the whole section circle (`brick((0.5, 3), (−2, 2),
  (−2, 2))`) refused `SectionArcSide { NoCertifiedRun }`: both pierces
  sit inside the planar face as a pierce ring, whose run is null
  scaffolding.

So the arc rule alone is not the gap: the planar side's adjacency skip
(`between_edge_in_plane`'s `bool_between_arc_window`) and the up-front
window are azimuth-premised too. The lane reverted to the refusal.

## What a fix owes

An arc cue for the planar side that reads no azimuth: the wall side's
own selection handed over (the wall chord's arc, or a witness point on
it — `boolean::join` would join the wall side first), with the
between-arc test re-stated against the same cue and the window
computed only where a monotone section reads it. The three poses
above, under every op, against their cap closed forms, are its rows.

## Answered (PR 3985, `reach/arc-from-pairing`)

The planar side reads the germ's direction, as the wall side does, and
the up-front `face_azimuth_window` read is gone. Measured against the
quarter-cap closed forms (`c` the cap beyond `x = 0.5`), every op:

- `brick((0.5, 3), (−2, 2), (0, 2))` against the `y`-poled unit ball:
  builds, `20 + 4π/3 − c/2`, `c/2`, `20 − c/2`, `4π/3 − c/2`;
- the same box mirrored to `z ∈ (−2, 0)`: builds, the same forms;
- `brick((0.5, 3), (−2, 2), (−2, 2))` (the planar face holds the whole
  circle, a pierce ring): builds, `40 + 4π/3 − c`, `c`, `40 − c`, `4π/3 − c`;
- the cube against the `y`-poled ball(0.3) at `(0.5, 0.5, 1)`: the
  chords build and the role read refuses `SectionLoopUndecided`
  (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior.md`).

Rows: `crates/sweep/tests/tilted_sphere_pair.rs`,
`a_plane_tilted_against_the_balls_chart_builds_under_every_boolean` and
`a_pip_with_its_seam_in_the_cubes_top_stops_at_the_role_read`.
