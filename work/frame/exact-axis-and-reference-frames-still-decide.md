---
id: exact-axis-and-reference-frames-still-decide
kind: issue
title: An axis-aligned frame given as an exact axis and an exact reference still pays the aim mint's decision
status: open
opened: 2026-10-21
---

## What

Found by the class sweep of `frame/exact-frames-are-turns`
(`exact-frame-mints-cover-three-of-the-world-frames`), which moved
the exact PAIRS through `RigidMotion::gram_schmidt` onto written
turns. The same class reaches the aim mint,
`RigidMotion::from_axis_and_reference`, through two helpers whose
callers mostly pass an exact axis and an exact reference:

- `demos/tour/src/scalar.rs` `axis_frame`, under `tour_frame_axis`:
  `tube.rs` `stops` and `tubewall.rs` `hollow` and the solid elbow in
  `stops` pass `(ŷ, x̂)` at the origin.
- `crates/sweep/src/test_support.rs` `tube_frame`: about 45 calls in
  `crates/sweep/tests` (`m6_tube.rs`, `shell7_*`, `verbs_tubewall*`,
  `p1b_r1_probes.rs`, `mate7a_*`, `shell8_multi_solid.rs`,
  `verbs_shell.rs`, `an_arc_rim_picks_the_row_it_lies_on.rs`) pass
  `(ŷ, x̂)`, `(ẑ, x̂)`, `(−ŷ, x̂)` or `(ẑ, x̂)` literals.
- `demos/tour/tests/verbs_demo2_r1_probes.rs` and
  `verbs_teapot_r2_probes.rs` carry their own copy of `axis_frame`
  and pass `(ŷ, x̂)` / `(ẑ, x̂)`.

`from_axis_and_reference` with axis `ŷ` and reference `x̂` is the
frame `(x̂, −ẑ, ŷ)`, a quarter turn back about `x̂`
(`RigidMotion::turned(origin, UnitVec3::exact_x(), Turns::quarters(-1))`),
bit for bit by the Rodrigues entries at a quarter turn.

## Why it was not moved with the pairs

The unit's brief kept `axis_frame` on the decision, and the reading
is the open part: a tube door's caller states the spine axis and the
radial its window starts from, and "the world frame turned a quarter
back about x̂" says that less directly than `(ŷ, x̂)` does. Whether
the written turn is still the natural spelling there, or the tube
fixtures want a helper named for the axis (`tube_frame_along_y`), is
the call this row asks for before the ~50 sites move.
