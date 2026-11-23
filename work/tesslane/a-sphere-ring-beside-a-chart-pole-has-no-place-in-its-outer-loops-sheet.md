---
id: a-sphere-ring-beside-a-chart-pole-has-no-place-in-its-outer-loops-sheet
kind: issue
title: A sphere face's ring beside a chart pole has no place in its outer loop's sheet, so the mesher refuses the face
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm]
---

Filed by ORBIT's `orbit/transient-chart-cut` lane.

## What

A sphere cut-in's chart cut is now closed in the output stage
(`topo::boolean::slit::close_cuts`), so a ball face that keeps the
region round a section circle holds that circle as a **ring**, where it
used to hold the cut's pieces between two faces. Where the circle passes
near the face's chart pole, the mesher refuses the face:
`DomainUnread { cause: Undecided { ring, cause: None } }`, from
`topo::pcurves_rings::RingPlacementError::Undecided` with no in-band
decision. The module docs name that shape: a work bound spent, or no
window round the ring's point has arms (a sphere point past the pole
rows).

Measured in `crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
`Want::RingUnread`:

- `a_circle_beside_a_pole_builds`, the radius-`0.01` ball whose circle
  passes `10⁻⁵` rad from the unit ball's pole: `b ∪ a` and `b ∖ a`.
  The small ball keeps its face less a near-hemisphere, whose circle
  passes beside its own poles.
- The same test's `10⁻³` ball centred on the unit sphere: `a ∪ b`,
  `b ∪ a`, `b ∖ a`. Before the closing, each of these drew.
- `a_trimmed_face_cut_beside_its_pole_builds`: at `δ = 10⁻³`, the same
  three ops (these drew before); at `10⁻⁵`, `b ∪ a` and `b ∖ a`.

Every body is valid, maximal, and at its closed-form volume. Where the
same circle is the boundary of a cap (`∩`), the face meshes. Where the
circle lies far from the pole, a ring meshes (the
`a_ball_on_the_centre_line_builds_at_every_radius_and_depth` poses,
every op).

## Where to look

`topo::pcurves_rings::place_ring` reads the ring's first point against
the outer loop's lifted rows in an analytic window
(`Outer::side`). Next to a sphere's pole the window's inf-side arm
(`polar_arm`) degenerates, so no window has arms. Either place the ring
by a point of it away from the pole, or read the window on the
pole-free chart a tilted circle's face could take
(`a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm`, which
is the same pole's other refusal).
