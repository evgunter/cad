---
id: a-spheres-circle-fit-recovers-the-radius-through-a-root
kind: issue
title: A sphere face's props_sphere_circle_fit reads its rim's radius through a root, so a sphere of boxed radius refuses at the walk's own scalar
status: open
opened: 2026-11-22
---


Found by the volume-premises lane (branch `boxfinish/volume-premises`,
2026-11-22), building a sphere fixture for
`the-volume-readings-interval-re-derivation-re-asks-a-closed-forms-premises-on-the-value-channel`.
It is the shape of
`a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root`
on a reader that row's widening does not reach (the cut plate has no
sphere).

## Measured (`Sym<Interval>`, ε = 1e-9)

A ball through the revolve door at radius 0.8125
(`sweep::test_support::revolved_about_y_at`, the half disc), its stored
geometry restated (`topo::test_support::restate_geometry`) so every
stored `±0.8125` reads `±r`, `r` one parameter over
`[0.7625, 0.8625]`. Check 7's volume role walk
(`topo::test_support::volume_role_reading`) refuses at the walk's own
scalar:

- `props_sphere_circle_on` decides `Zero` on all four arcs;
- `props_sphere_circle_fit` escalates on all four,
  `[-0.10000000000000031, 0.10000000000000031]` on three and
  `[-0.10000000000000031, 0.10577774934061313]` on one, and the walk
  returns `Face { 1v1, Escalated { "props_sphere_circle_fit", … } }`.

The margin (`geom-brep` `props/curved.rs`, the sphere's loop parse) is
`(w.norm_squared() + r_c.powi(2)).sqrt() − radius`, `w` the arc's
centre less the sphere's: here `√(0 + r²) − r`.

The same ball with its POSITION boxed and its radius exact passes
(`crates/sweep/tests/volume_premises_over_a_box.rs`, the sphere row),
so the hold is this one margin.

## Unmeasured

Whether the rationalized gap `norm_gap` spells (`(|w|² + r_c² − R²)/(√… + R)`)
folds here as it does on `chart_distance`'s arms.
