---
id: a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale
kind: issue
title: A bar through a ball refuses Escalated(bool_join_nearest) at ×1e-3 and RingOffCylinderChart at ×1 and ×1e3: the nearest-facing rank reads chord length in absolute metres
status: open
opened: 2026-10-03
---


Found in the dual review of PR 3985 (`reach/arc-from-pairing`, NOTE N2
of the second lane, by execution), filed by that PR's fix pass: the
ranking is pre-existing, newly reached because the PR lets these
pierces past the chord's arc.

## What

The second reviewer's bar-through-ball pierces
(`analysis/reach-dual/3985-r2`, `probes/review_reach_dual3985_r2_probes.rs`:
bars and L-notches through the unit ball, off its axes), scaled about
the origin:

- at ×1e-3 they refuse `Escalated(bool_join_nearest)`, margin
  −9.7e-9;
- at ×1 and ×1e3 the same poses refuse `RingOffCylinderChart { Sphere }`
  (`work/tang/a-ring-on-a-sphere-face-has-no-island-winding.md`).

(The shipped bar, `crates/sweep/tests/snowman.rs:624`, builds at ×1.)

`find_match` ranks the facing partners that survive the walk filter by
chord length, `decide("bool_join_nearest", Margin::of(dist − bd))`
(`crates/topo/src/boolean/join.rs:949`; its loose-end twin at `:1760`):
a difference of two lengths in metres, unlevered. Two candidates
whose chords differ by a fixed fraction of the body's size tie within
the band once the body is small enough, so the door a pose stops at
depends on its scale.

## Done when

A pose and its scaled copies stop at the same door (or build alike),
whether by ranking only partners the walk leaves adjacent — where no
tie can arise — or by a margin whose comparand is not an absolute
length; pinned by a row at three scales.
