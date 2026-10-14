---
id: face-hinge-lever-applies-its-larger-side-both-ways
kind: issue
title: the plane×cylinder face lever applies its larger axial side in both directions
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`Reach::hinge_lever` (`crates/geom-brep/src/extent.rs`) reads a
`Reach::Face` as `max(|above − shift|, |below + shift|)`, and
`pc_parallel_gap` (`decide_across` in `crates/geom-brep/src/intersect.rs`)
levers the tilt at it in both directions. A one-sided face moves the
gap by the tilt times `above` one way and `below` the other, so where
the tilt and the gap's sign agree on the short side the sum over-states
by the difference. Measured in PR 4280's differential on asymmetric
faces (622 good-input escalations of the 9,815 head served).

## The shape of a fix

Read the signed range `(−below, above)` from the hinge's station and
decide the bracket's two ends, rather than `|datum| ± tilt·max(side)`.
