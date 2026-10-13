---
id: circle-pair-edge-candidates-cancel-at-unequal-radii
kind: issue
title: The at-rest validator's circle x circle edge-pair candidates are placed by differences of squares, so at radii orders of magnitude apart they sit off the smaller circle and a crossing can read apart
status: open
opened: 2026-10-13
priority: P1
cost: E
---


Found by the sweep of `paths/arc-arc-secant-arm-misses-crossings-at-unequal-radii`,
which fixed the same algebra in the profile validator. Not reproduced at
this site; the algebra is the one measured there.

## The shape

`crates/topo/src/validate.rs`, the circle × circle arm of the edge-pair
intersector (the one reading `ring_outer_meet_centres`), places its
candidates at `a = (d² + r₁² − r₂²)/2d` from `c₁` and the half-chord
`((r₁ − a)(r₁ + a)).max(0).sqrt()`. Factoring after `a` is rounded does
not help: `r₁ − a` inherits `a`'s rounding, about `u·r₁²/d`, so where
`r₁ ≫ r₂` the half-chord's square, at most `r₂²`, carries an error of
about `u·r₁²`. The candidates then sit off the smaller circle by up to
its whole radius; `meet_at` reads them off its trim, and a pair of arcs
that cross can read apart.

## Measured on the profile twin

`crates/profile/src/seg.rs` `arc_arc` had the same closed form. Against
a 60-digit oracle, over radius ratios 1 to 10¹² and centres up to 10³
from the origin, its crossings sat off the smaller carrier by up to
1.9e8 ulps of the largest of the radii and the centres' coordinates
(all of the smaller radius past a ratio of about 10⁸). Two arc pairs
from the issue read no contact while crossing, and a profile whose arcs
cross validated.

## Where to look

`seg.rs` `radical_chord` takes both from the clearances:
`r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`, with
`r₁ − r₂` and `r₁ + r₂` formed once. Measured there at no more than 3.6
ulps of the same scale off either carrier across the whole range, and
Interval encloses (it is an identity). The tangent arm's foot
(`a = ±r₁`) is `decided-tangent-point-is-the-radical-foot`'s matter,
not this one's.
