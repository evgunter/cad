---
id: topo-test-frames-hand-roll-the-across-axis-reference
kind: issue
title: Four topo test fixtures hand-roll an across-axis reference instead of calling Vec3::orthonormal_basis
status: open
opened: 2026-10-26
priority: P4
cost: E
---


Found by the easy-fixes lane's sweep for
`probe/join-probe-charts-hand-roll-the-across-axis-reference`, which
fixed `boolean/join.rs`'s `cylinder_at` and the
`one_frame_serves_both_coaxial_cylinder_sphere_circles` probe.

## What

Four test-only helpers pick a world axis by an `if n.x.abs() < 0.9`
threshold and cross it with the axis, so each carries its own
degenerate-axis policy beside `Vec3::orthonormal_basis`:

- `crates/topo/src/test_support_meeting.rs` (~477): `seed`, then
  `u = n.cross(seed).normalize()`, `v = n.cross(u)`;
- `crates/topo/src/boolean/carrier_eq.rs` `points_on`'s `square`
  (~1156);
- `crates/topo/src/boolean/offer_rows.rs` `seam` (~1395): `across`,
  then `n1 = k.cross(across).normalize()`;
- `crates/topo/src/boolean/circle_torus/meridian_rows.rs` `unit_perp`
  (~115).

## Fix

`let (u, v) = n.orthonormal_basis();` (`b2 = n × b1`, so `v` keeps its
`n.cross(u)` handedness). The fixtures' chart bits move, so each needs
a build and its rows run.

## Sweep

`grep -rnE 'if [a-z_.]+\.(x|y|z)\.abs\(\) <=? [0-9.]+ \{' crates/topo/src`.
The one other hit, `census.rs` (~12687), picks an in-plane axis with no
cross product and is not this shape. The pattern cannot match a seed
chosen by a `match`, or by a comparison of two components.
