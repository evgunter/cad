---
id: sphere-sphere-section-radius-cancels-at-unequal-radii
kind: issue
title: sphere_sphere_section's secant arm takes the section circle's radius from a cancelling radical offset, so at radii orders of magnitude apart the circle lies off the smaller sphere
status: open
opened: 2026-10-13
priority: P2
cost: E
---


Found by the sweep of `paths/arc-arc-secant-arm-misses-crossings-at-unequal-radii`,
which fixed the same algebra in the profile validator. Not reproduced
here; the algebra is the one measured there.

## The shape

`crates/geom-brep/src/intersect.rs` `sphere_sphere_section`, the proper
secant arm, stores `Curve3::Circle { center: c1 + n·a, radius:
√((r₁ − a)(r₁ + a)) }` with `a = (d² + r₁² − r₂²)/2d` (its `offset`
closure). `a` carries about `u·r₁²/d`, so where `r₁ ≫ r₂` the radius'
square, at most `r₂²`, carries an error of about `u·r₁²`: the stored
section circle lies off the smaller sphere by up to its whole radius.
This one is stored geometry, not a candidate read and dropped.

## Measured on the profile twin

`crates/profile/src/seg.rs` `arc_arc` had the same closed form in 2-D.
Against a 60-digit oracle, over radius ratios 1 to 10¹² and centres up
to 10³ from the origin, its crossings sat off the smaller carrier by up
to 1.9e8 ulps of the largest of the radii and the centres' coordinates.

## Where to look

`seg.rs` `Circles` (`radical`, `crossings`): `r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`, at most
3.6 ulps off either carrier across the same range, and an identity, so
Interval encloses. The Zero arms' tangent point is
`decided-tangent-point-is-the-radical-foot`'s matter. A shared
`geom_core` home for the algebra is
`work/issues/circle-pair-radical-chord-has-no-shared-home.md`; fixing
this site before it lands mints another copy.

2026-10-14 (PATHS, `paths/circle-pair-shared-home`): the shared home has landed as `geom_core::Circles` (`crates/geom-core/src/arc/pair.rs`): built from `d` and the two radii, it hands out the identity, external and internal margins and the cancellation-free `radical` foot and `half_chord` from one `r₁ + r₂` and `r₁ − r₂`; `Circles2` places them in the plane. profile `seg.rs` and topo `validate.rs` arm 5 read it. Fix this site by reading it, deciding its margins from the same value, rather than minting a copy.
