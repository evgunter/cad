---
id: a-near-parallel-meets-enclosure-is-decorrelated-by-its-unit-direction
kind: issue
title: A near-parallel Meet's interval enclosure divides independently enclosed terms by the planes' angle
status: open
opened: 2026-11-18
---


`PoseDef::Meet`'s solve (`crates/editor-core/src/eval/wire/pose.rs:416`)
spells the line's point as `(ca (nb × d) + cb (d × na)) / |d|²`, with
`d = na × nb` (`pose.rs:394`). Where the planes are nearly parallel, the
numerator's two terms cancel to far below their own size, but an enclosure
lane holds them independently. Its width is of the order of the terms, and
dividing by `|d|²` makes it of the order of `1/angle`. Bisection does not
shrink it: a leaf's `hi/lo` ratio of the angle stays bounded away from 1.

Measured on `m10_3_driver_interval::sliver_axis`, where the angle is
5ε..35ε:

- **The solve as written:** origin.y is enclosed as ±3.75e7 m and z as
  [-0.67, 2.06]. The f64 samples over the whole box stand at |y| ≤ 1.5e-8,
  so no member is near the session box.
- **Option (a), `(ca (nb × d̂) + cb (d̂ × na)) / |d|`:** still wide.
  `UnitVec3::new(d)` encloses d̂'s components independently of `d`, so
  d̂.x is [0.57, 1.75] where it is exactly 1. The point's y comes out as
  ±5.9e7 m and z as [0.33, 3.06].

The sliver row itself no longer reaches the solve:
`coincide::meet_by_construction` claims the meet where one plane is a spin
of the other about a meet of it (fork 288). A general near-parallel Meet
with no such construction still takes the solve, and a placing door's
session-box ask then escalates at every leaf, so the driver bisects to its
budget.

A Meet of near-parallel planes that share no known line is
ill-conditioned, and a wide enclosure is honest there only up to the
family's true spread. A fix keeps the correlation in the enclosure. Two
ways to do that:

- a spelling that never forms the cancelling term (the `turn_arms` way:
  for example, the point on both planes nearest a reference point, solved
  in the planes' own 2-D frame);
- intersecting two spellings at Interval while reading only today's at
  every other kind, which keeps f64 bit-neutral. This is
  `Real::meet_readings(first, || second)` on the frames lane's branch, not
  yet on main.
