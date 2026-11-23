---
id: the-cone-torus-coaxial-pose-is-decided-twice-and-counted-three-times
kind: issue
title: A coaxial cone × torus pose is decided by two coaxiality readings and counted by three spellings of its closed form
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found in PR 7395's dual review.

## Measured

Two readings decide whether a cone and a torus are coaxial:

- `pair_section/cone_torus.rs` `ConeTorusPose::decide` uses
  `torus::axis_pose` (`section_axes_tilt`, `section_axes_offset`) over
  the torus's own ball, with the coaxial arm's turn term. The section
  value, the march door, `route_pose`, the join and the section
  certificate all read this.
- `intersect.rs` `coaxial_cone_torus` (`cone_torus_section`) uses
  `kot_coaxial` (the torus centre's distance from the cone's axis) and
  `kot_axes_parallel` (the cross product levered at the caller's
  `lever`). The offset door reads this through
  `offset_derive::coaxial_with_cone`.

A pose at the band's edge can read coaxial in one and not in the
other. Then the value says `Elsewhere` while the closed form refuses
`RoutesToGeneralRung`, or the other way round.

The coaxial count is also spelled three times:

- `cone_torus_section`'s `kot_reach` per nappe, which mints the
  circles;
- `section_cert.rs` `cone_torus`'s `section_torus_coaxial_cone_near` /
  `_far`, read across the pose's slack;
- the value, which does not count: it reads the pose elsewhere.

## What the taker owes

One coaxial reading. Either the closed form takes the decided pose
(its slack, with `kot_reach` read across it), or the pose reads the
closed form's rows. Then one count, which the certificate reads off the
closed form's circles rather than re-deriving `r − |R cos α ∓ z sin α|`.
The `kot_*` and `section_torus_coaxial_cone_*` rows are k-lint names,
so the move re-baselines them.
