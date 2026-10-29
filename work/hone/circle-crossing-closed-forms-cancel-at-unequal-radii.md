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
  circle × circle arm: retired. `carrier_cross` reads
  `geom_brep::curve_pair::meet`, whose coplanar circles read their
  distance as a first harmonic with exact metre extremes (stage (c) of
  `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)).
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

`geom_core::Circles` (`radical`, `half_chord`; `crates/geom-core/src/arc/pair.rs`) takes `a` and `h²` from the clearances,
`r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`:
at most 3.6 ulps off either carrier across the same range, and an
identity, so Interval encloses. It is the one home for that algebra;
fixing these sites with a local spelling mints a copy at each.

2026-10-14 (PATHS, `paths/circle-pair-shared-home`): the shared home has landed as `geom_core::Circles` (`crates/geom-core/src/arc/pair.rs`): built from `d` and the two radii, it hands out the identity, external and internal margins and the cancellation-free `radical` foot and `half_chord` from one `r₁ + r₂` and `r₁ − r₂`; `Circles2` places them in the plane. profile `seg.rs` and topo `validate.rs` arm 5 read it. Fix this site by reading it, deciding its margins from the same value, rather than minting a copy.

2026-10-29 (SECT, `sect/circle-crossing-once`): the two torus sites
named above now live in `crates/geom-brep/src/pair_section/torus_arms.rs`.
The square-wall arm (`square_folds`) reads `geom_core::Circles` for its
tube row and its folds. The tube circle against the sphere's great
circle (`sphere_arm`) does not yet, since its secant row is another
decomposition whose move can shift a verdict at `Interval`; it is
`work/sect/sphere-against-a-tube-circle-decides-its-secant-in-its-own-decomposition.md`.
`sphere_sphere` here is unchanged.
