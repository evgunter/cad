---
id: circle-crossing-closed-forms-cancel-at-unequal-radii
kind: issue
title: The boolean's circle x circle and sphere x sphere closed forms place their meetings by differences of squares, off the smaller carrier at radii orders of magnitude apart
status: open
opened: 2026-10-13
priority: P2
cost: E
---


Found by the sweep of `paths/arc-arc-secant-arm-misses-crossings-at-unequal-radii`,
which fixed the same algebra in the profile validator. Not reproduced at
these sites; the algebra is the one measured there.

## The sites

Each places a meeting at `a = (d² + r₁² − r₂²)/2d` from one centre and
a half-chord `√(r₁² − a²)` or `√((r₁ − a)(r₁ + a))`, which are the same
thing once `a` is rounded: `a` carries about `u·r₁²/d`, so where
`r₁ ≫ r₂` the half-chord's square, at most `r₂²`, carries an error of
about `u·r₁²`, and the meeting sits off the smaller carrier by up to
its whole radius.

- `crates/topo/src/boolean/carrier_cross.rs` `meetings`, the coplanar
  circle × circle arm: the two crossing candidates.
- `crates/topo/src/boolean/rim_wedge.rs`, the coplanar-circles arm of
  the rim candidates (`x` and `y2 = re² − x²`).
- `crates/topo/src/boolean/section_cert.rs` `sphere_sphere`: the
  section circle's witness point `c1 + k·x + b1·rad` (ORBIT shares
  this file).
- `crates/topo/src/boolean/section_cert.rs`, the tube circle against
  the sphere's great circle in `Π` (`along` and `half`, the
  one-tube-circle arm of the pinch reading). The square-wall arm below
  it carries a `section_torus_square_wall_fold_drift` gate for its own
  rounding and refuses past it, so it is guarded rather than silent.

## Measured on the profile twin

`crates/profile/src/seg.rs` `arc_arc` had the same closed form. Against
a 60-digit oracle, over radius ratios 1 to 10¹² and centres up to 10³
from the origin, its crossings sat off the smaller carrier by up to
1.9e8 ulps of the largest of the radii and the centres' coordinates.

## Where to look

`seg.rs` `Circles` (`radical`, `crossings`) takes `a` and `h²` from the clearances,
`r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`:
at most 3.6 ulps off either carrier across the same range, and an
identity, so Interval encloses. A shared `geom_core` home for it is
`work/issues/circle-pair-radical-chord-has-no-shared-home.md`; fixing
these sites before it lands mints a copy at each.
