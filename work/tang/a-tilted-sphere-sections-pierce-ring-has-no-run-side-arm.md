---
id: a-tilted-sphere-sections-pierce-ring-has-no-run-side-arm
kind: issue
title: A tilted sphere section that passes through a face as a ring refuses NoCertifiedRun: the run-side arc rule has no cross-loop arm
status: review
opened: 2026-10-02
pr: 3985
branch: reach/arc-from-pairing
---


## What

A sphere section TILTED against the chart's polar axis takes the
run-side arc rule (`chord_join::select_arc_by_run_side`,
`crates/topo/src/chord_join.rs`): it reads the side of the run a chord
CLOSES. Where such a section passes through a sphere face as a ring, the
join is cross-loop (`mekr`), which closes no run, so the rule is handed
none and refuses `Join(SectionArcSide { case: NoCertifiedRun })`.

The polar (azimuth-monotone) ring builds since PR 3851 (TANG,
`tang/pierce-ring`), which selects a cross-loop chord's arc by
containment in the divided face's outer-cycle window; that window is a
containment rule's input and means nothing to the run-side rule
(`ChordRun::FaceWindow`), so the tilted ring keeps this refusal.

Witnesses, all refusing under ∪, ∩ and ∖:

- the tour's snowman with its head moved 0.05 along `z`
  (`demos/tour/src/snowman.rs`,
  `the_head_moved_out_of_the_seam_plane_refuses_at_the_tilted_ring`);
- ball(1) at `(2, 2, 0.5)` against ball(1) at `(3.3, 2, 0.7)`, and
  against ball(0.7) at `(2.9, 2.3, 1.1)`
  (`crates/sweep/tests/tilted_sphere_pair.rs`,
  `a_tilted_section_stops_at_the_pierce_ring_and_the_planar_side`).

## The shape of a fix

The containment rule's statement for a cross-loop chord, in run-side
form: the arc whose side places the divided face's own outer cycle on
the material side, i.e. the face's whole boundary as the "run" with the
chord's two ends on the ring rather than on it. Not attempted here.

## Answered (PR 3985, `reach/arc-from-pairing`)

A pierce ring's cross-loop chord takes the arc the pierce germs'
directions name, on any carrier and tilt. Both witnesses build under
every op to the lens closed form (`tilted_sphere_pair.rs`,
`a_pierce_off_the_seam_plane_builds_under_every_boolean`), and the
tour's snowman with its head moved 0.05 along `z` builds to the two-ball
closed form (`demos/tour/src/snowman.rs`,
`the_head_moved_out_of_the_seam_plane_builds_through_its_ring`).
