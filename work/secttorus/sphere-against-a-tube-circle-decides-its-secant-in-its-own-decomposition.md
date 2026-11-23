---
id: sphere-against-a-tube-circle-decides-its-secant-in-its-own-decomposition
kind: issue
title: A sphere against a torus's tube circle decides the circle pair's secant as min(d + r − ρ, ρ − |d − r|), not Circles::secant, and places the crossing by the textbook radical form
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the sweep of `sect/circle-crossing-once`
(`circle-pair-crossing-is-spelled-three-times`), 2026-10-29.

## Measured

Two rows decide whether a sphere's great circle (radius `ρ`) crosses a
tube circle (radius `r`) whose centre stands `d` from it, as the lesser
of `d + r − ρ` and `ρ − |d − r|`:

- `crates/geom-brep/src/pair_section/torus_arms.rs` `sphere_arm`,
  `cuts`, read by `section_torus_sphere_near_tube` and
  `section_torus_sphere_far_tube`; where exactly one reads true it
  places the crossing at `along = (d² + r² − ρ²)/2d`,
  `half = √((r − along)(r + along))`;
- `crates/geom-brep/src/pair_section/torus.rs` `SphereTorusPose::decide`,
  the `st_coaxial_tube` row, decided across the coaxial offset.

Over the reals that is `geom_core::Circles::new(d, r, ρ).secant()`
(both are the least of the three triangle inequalities), but it is
another decomposition, so its bits differ (about half of 800,000
seeded pairs) and its verdict can too: at `Interval`, `d ∈
[66031.64743788529, 66031.64743788536]`, `r = 778.9461096896242`,
`ρ = 65252.701328195864`, at the band `zero = 2.128e-10`, the old form
encloses `[1.237e-10, 2.110e-10]` (`Zero`) and `secant()`
`[1.164e-10, 2.183e-10]` (indeterminate). One pair in 800,000 at
`Interval`, none at `f64`.

The placement is the textbook radical form, which at radii orders of
magnitude apart sits off the smaller circle: over the same seeds the
torus arms' old form of it reached 9.4·10⁵ ulps of scale off a circle
where `Circles2::crossings` stays within 3.2. This is the site HONE's
`circle-crossing-closed-forms-cancel-at-unequal-radii` names as the
section certificate's "tube circle against the sphere's great circle
in `Π`", since moved here.

## What the taker owes

Whether these rows read `Circles::secant()` (and the crossing
`Circles2::crossings`, deciding from the same value): a change of
decomposition that can move a verdict at the band's edge, so it is
argued on which reading is right, not on the moved bit.
